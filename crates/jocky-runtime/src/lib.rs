use chrono::Utc;
use jocky_common::{
    Evidence, Finding, FindingNode, GraphEdge, GraphNode, InvestigationGraph, InvestigationResult,
    InvestigationTimeline, RelationshipType, RiskScore, TimelineEvent,
};
use jocky_forensics::CollectorRegistry;
use jocky_ir::{IrOperation, IrProgram};
use std::fs;

pub struct Runtime {
    investigation_id: String,
    start_time: String,
    target: String,
    evidence: Vec<Evidence>,
    edges: Vec<GraphEdge>,
    findings: Vec<Finding>,
    risk_score: u32,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            investigation_id: format!("inv-{}", uuid::Uuid::new_v4()),
            start_time: Utc::now().to_rfc3339(),
            target: String::new(),
            evidence: Vec::new(),
            edges: Vec::new(),
            findings: Vec::new(),
            risk_score: 0,
        }
    }

    pub fn execute(&mut self, program: &IrProgram) -> Result<(), String> {
        for op in &program.operations {
            match op {
                IrOperation::Target { value } => {
                    self.target = value.clone();
                }
                IrOperation::Collect { target } => {
                    let collector = match target.as_str() {
                        "system" => CollectorRegistry::get_system_collector(),
                        "process" => CollectorRegistry::get_process_collector(),
                        "network" => CollectorRegistry::get_network_collector(),
                        _ => Box::new(jocky_forensics::mock::MockSystemCollector::new()),
                    };

                    match collector.collect() {
                        Ok(mut ev) => self.evidence.append(&mut ev),
                        Err(e) => return Err(format!("Failed to collect {}: {}", target, e)),
                    }
                }
                IrOperation::Correlate { left, right } => {
                    if left == "process" && right == "network" {
                        let mut new_edges = Vec::new();
                        let networks: Vec<_> = self
                            .evidence
                            .iter()
                            .filter(|e| e.evidence_type == "network")
                            .collect();
                        let processes: Vec<_> = self
                            .evidence
                            .iter()
                            .filter(|e| e.evidence_type == "process")
                            .collect();

                        for net in networks {
                            if let Some(net_pid) =
                                net.attributes.get("pid").and_then(|p| p.as_u64())
                            {
                                for proc in &processes {
                                    if let Some(proc_pid) =
                                        proc.attributes.get("pid").and_then(|p| p.as_u64())
                                    {
                                        if net_pid == proc_pid {
                                            new_edges.push(GraphEdge {
                                                source: proc.id.clone(),
                                                relationship: RelationshipType::ConnectedTo,
                                                target: net.id.clone(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        self.edges.extend(new_edges);
                    }
                }
                IrOperation::Detect { rule } => {
                    let mut new_findings = jocky_detection::detect(rule, &self.evidence);
                    self.findings.append(&mut new_findings);
                }
                IrOperation::DetectRules { path } => {
                    let content = fs::read_to_string(path)
                        .map_err(|e| format!("Failed to read rules file {}: {}", path, e))?;
                    let rules = jocky_detection::parse_rules_yaml(&content)
                        .map_err(|e| format!("Failed to parse rules from {}: {}", path, e))?;
                    let mut new_findings = jocky_detection::detect_rules(&rules, &self.evidence);
                    self.findings.append(&mut new_findings);
                }
                IrOperation::Score => {
                    self.risk_score = self.findings.iter().map(|f| f.severity.weight()).sum();
                }
                IrOperation::Report => {}
            }
        }

        let mut finding_edges = Vec::new();
        for finding in &self.findings {
            for ev_id in &finding.evidence_ids {
                finding_edges.push(GraphEdge {
                    source: ev_id.clone(),
                    relationship: RelationshipType::GeneratedFinding,
                    target: finding.id.clone(),
                });
            }
        }
        self.edges.extend(finding_edges);

        Ok(())
    }

    pub fn get_evidence(&self) -> &[Evidence] {
        &self.evidence
    }

    pub fn generate_result(&self) -> InvestigationResult {
        InvestigationResult {
            schema_version: "0.1".to_string(),
            investigation_id: self.investigation_id.clone(),
            target: self.target.clone(),
            platform: if cfg!(target_os = "windows") {
                "windows".to_string()
            } else {
                "linux".to_string()
            },
            start_time: self.start_time.clone(),
            end_time: Utc::now().to_rfc3339(),
            graph: self.build_graph(),
            timeline: self.build_timeline(),
            findings: self.findings.clone(),
            risk_score: RiskScore::new(self.risk_score),
        }
    }

    fn build_graph(&self) -> InvestigationGraph {
        let mut nodes = Vec::new();
        for ev in &self.evidence {
            nodes.push(GraphNode::Evidence(ev.clone()));
        }
        for finding in &self.findings {
            nodes.push(GraphNode::Finding(FindingNode {
                id: finding.id.clone(),
                rule_id: finding.rule_id.clone(),
                title: finding.title.clone(),
                severity: finding.severity.clone(),
                confidence: finding.confidence,
                timestamp: finding.timestamp.clone(),
            }));
        }
        InvestigationGraph {
            nodes,
            edges: self.edges.clone(),
        }
    }

    pub fn get_edges(&self) -> &[GraphEdge] {
        &self.edges
    }

    pub fn get_findings(&self) -> &[Finding] {
        &self.findings
    }

    pub fn build_timeline(&self) -> InvestigationTimeline {
        let mut events = Vec::new();

        for ev in &self.evidence {
            events.push(TimelineEvent {
                timestamp: ev.timestamp.clone(),
                event_type: ev.evidence_type.to_uppercase(),
                evidence_id: ev.id.clone(),
                host: ev.host.clone(),
                description: format!("Evidence collected: {}", ev.evidence_type),
            });
        }

        for finding in &self.findings {
            events.push(TimelineEvent {
                timestamp: finding.timestamp.clone(),
                event_type: "FINDING".to_string(),
                evidence_id: finding.id.clone(),
                host: self.target.clone(),
                description: format!("Indicator detected: {}", finding.title),
            });
        }

        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        InvestigationTimeline { events }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jocky_common::Severity;
    use serde_json::json;
    use std::path::PathBuf;

    #[test]
    fn test_full_validation_pipeline() {
        let mut runtime = Runtime::new();
        runtime.target = "TEST-TARGET".to_string();

        let ev_proc1 = Evidence::new(
            "TEST-HOST",
            "process",
            "windows",
            json!({ "pid": 100, "name": "normal.exe" }),
        );
        let ev_proc2 = Evidence::new(
            "TEST-HOST",
            "process",
            "windows",
            json!({ "pid": 200, "name": "example.exe" }),
        );
        let ev_net1 = Evidence::new(
            "TEST-HOST",
            "network",
            "windows",
            json!({ "pid": 200, "remote_port": "4444" }),
        );

        runtime.evidence = vec![ev_proc1.clone(), ev_proc2.clone(), ev_net1.clone()];

        let prog_correlate = IrProgram {
            operations: vec![IrOperation::Correlate {
                left: "process".to_string(),
                right: "network".to_string(),
            }],
        };
        runtime.execute(&prog_correlate).unwrap();

        assert_eq!(runtime.edges.len(), 1);
        assert_eq!(runtime.edges[0].source, ev_proc2.id);
        assert_eq!(runtime.edges[0].target, ev_net1.id);
        assert_eq!(
            format!("{:?}", runtime.edges[0].relationship),
            "ConnectedTo"
        );

        // Locate rules dir
        let mut rules_path = PathBuf::from("../../rules/process.yml");
        if !rules_path.exists() {
            rules_path = PathBuf::from("rules/process.yml");
        }

        let prog_detect = IrProgram {
            operations: vec![IrOperation::DetectRules {
                path: rules_path.to_string_lossy().to_string(),
            }],
        };
        runtime.execute(&prog_detect).unwrap();

        assert_eq!(runtime.findings.len(), 1);
        let finding = &runtime.findings[0];
        assert_eq!(finding.rule_id, "PROC-SUSPICIOUS-NAME-001");
        assert_eq!(finding.severity, Severity::Medium);
        assert_eq!(finding.evidence_ids.len(), 1);
        assert_eq!(finding.evidence_ids[0], ev_proc2.id);

        let prog_score = IrProgram {
            operations: vec![IrOperation::Score],
        };
        runtime.execute(&prog_score).unwrap();
        assert_eq!(runtime.risk_score, 25); // Medium weight is 25

        let result = runtime.generate_result();

        assert_eq!(result.graph.nodes.len(), 4);
        assert_eq!(result.timeline.events.len(), 4);

        let mut prev = &result.timeline.events[0].timestamp;
        for ev in &result.timeline.events[1..] {
            assert!(ev.timestamp >= *prev);
            prev = &ev.timestamp;
        }

        let json_str = serde_json::to_string(&result).unwrap();
        let parsed: InvestigationResult = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed.investigation_id, result.investigation_id);
    }
}
