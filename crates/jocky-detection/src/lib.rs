use jocky_common::{Evidence, Finding, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleCondition {
    Equals { field: String, value: String },
    Contains { field: String, value: String },
    StartsWith { field: String, value: String },
    EndsWith { field: String, value: String },
    GreaterThan { field: String, value: f64 },
    LessThan { field: String, value: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub evidence_type: String,
    pub conditions: Vec<RuleCondition>,
}

impl RuleCondition {
    pub fn matches(&self, evidence: &Evidence) -> bool {
        match self {
            RuleCondition::Equals { field, value } => {
                evidence.attributes.get(field).and_then(|v| v.as_str()) == Some(value)
            }
            RuleCondition::Contains { field, value } => evidence
                .attributes
                .get(field)
                .and_then(|v| v.as_str())
                .map(|s| s.contains(value))
                .unwrap_or(false),
            RuleCondition::StartsWith { field, value } => evidence
                .attributes
                .get(field)
                .and_then(|v| v.as_str())
                .map(|s| s.starts_with(value))
                .unwrap_or(false),
            RuleCondition::EndsWith { field, value } => evidence
                .attributes
                .get(field)
                .and_then(|v| v.as_str())
                .map(|s| s.ends_with(value))
                .unwrap_or(false),
            RuleCondition::GreaterThan { field, value } => evidence
                .attributes
                .get(field)
                .and_then(|v| v.as_f64())
                .map(|f| f > *value)
                .unwrap_or(false),
            RuleCondition::LessThan { field, value } => evidence
                .attributes
                .get(field)
                .and_then(|v| v.as_f64())
                .map(|f| f < *value)
                .unwrap_or(false),
        }
    }
}

pub fn detect(rule: &str, evidence: &[Evidence]) -> Vec<Finding> {
    // Legacy hard-coded rules for fallback
    let mut findings = Vec::new();

    if rule == "SUSPICIOUS_PROCESS" {
        for ev in evidence {
            if ev.evidence_type == "process" {
                if let Some(name) = ev.attributes.get("name").and_then(|n| n.as_str()) {
                    if name == "example.exe" || name == "suspicious.exe" {
                        findings.push(Finding {
                            id: uuid::Uuid::new_v4().to_string(),
                            rule_id: "LEGACY-PROC".to_string(),
                            title: "Suspicious Process Name".to_string(),
                            description: format!("Suspicious process found: {}", name),
                            severity: Severity::Medium,
                            confidence: 0.8,
                            evidence_ids: vec![ev.id.clone()],
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                }
            }
        }
    } else if rule == "SUSPICIOUS_NETWORK" {
        for ev in evidence {
            if ev.evidence_type == "network" {
                if let Some(port) = ev.attributes.get("remote_port").and_then(|p| p.as_u64()) {
                    if port == 443 || port == 4444 {
                        findings.push(Finding {
                            id: uuid::Uuid::new_v4().to_string(),
                            rule_id: "LEGACY-NET".to_string(),
                            title: "Suspicious Network Port".to_string(),
                            description: format!("Suspicious network port connected: {}", port),
                            severity: Severity::Medium,
                            confidence: 0.8,
                            evidence_ids: vec![ev.id.clone()],
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                }
            }
        }
    }

    findings
}

pub fn detect_rules(rules: &[DetectionRule], evidence: &[Evidence]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for rule in rules {
        for ev in evidence {
            if ev.evidence_type == rule.evidence_type {
                let mut all_match = true;
                for cond in &rule.conditions {
                    if !cond.matches(ev) {
                        all_match = false;
                        break;
                    }
                }

                if all_match && !rule.conditions.is_empty() {
                    findings.push(Finding {
                        id: uuid::Uuid::new_v4().to_string(),
                        rule_id: rule.id.clone(),
                        title: rule.name.clone(),
                        description: rule.description.clone(),
                        severity: rule.severity.clone(),
                        confidence: 0.9,
                        evidence_ids: vec![ev.id.clone()],
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    });
                }
            }
        }
    }

    findings
}

pub fn parse_rules_yaml(yaml: &str) -> Result<Vec<DetectionRule>, String> {
    serde_yaml::from_str(yaml).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jocky_common::Evidence;

    #[test]
    fn test_yaml_rule_parsing_and_evaluation() {
        let yaml = r#"
- id: TEST-001
  name: Test Rule
  description: A test detection rule
  severity: medium
  evidence_type: process
  conditions:
    - !equals
      field: name
      value: "malware.exe"
"#;
        let rules = parse_rules_yaml(yaml).unwrap();
        assert_eq!(rules.len(), 1);

        let ev = Evidence::new(
            "LOCAL",
            "process",
            "test",
            serde_json::json!({
                "name": "malware.exe"
            }),
        );

        let findings = detect_rules(&rules, &[ev]);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "TEST-001");
        assert_eq!(findings[0].severity, Severity::Medium);
    }
}
