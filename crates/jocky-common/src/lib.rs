use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceType {
    System,
    Process,
    File,
    Network,
    User,
    Event,
    Persistence,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub host: String,
    pub timestamp: String,
    pub evidence_type: String,
    pub source: String,
    pub attributes: serde_json::Value,
}

impl Evidence {
    pub fn new(host: &str, ev_type: &str, source: &str, attributes: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            host: host.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            evidence_type: ev_type.to_string(),
            source: source.to_string(),
            attributes,
        }
    }
}

// Deprecated in favor of GraphEdge for Phase 4, but kept for parser backwards compatibility internally if needed.
// However, the prompt says to formalize the GraphEdge. I'll redefine Relationship type to use GraphEdge instead in Runtime.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub source: String,
    pub relationship: String,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn weight(&self) -> u32 {
        match self {
            Severity::Info => 0,
            Severity::Low => 10,
            Severity::Medium => 25,
            Severity::High => 50,
            Severity::Critical => 80,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub rule_id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub confidence: f64,
    pub evidence_ids: Vec<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskScore {
    pub score: u32,
    pub priority: String,
}

impl RiskScore {
    pub fn new(score: u32) -> Self {
        let priority = if score >= 80 {
            "CRITICAL"
        } else if score >= 50 {
            "HIGH"
        } else if score >= 20 {
            "MEDIUM"
        } else {
            "LOW"
        };
        Self {
            score,
            priority: priority.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp: String,
    pub event_type: String,
    pub evidence_id: String,
    pub host: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestigationTimeline {
    pub events: Vec<TimelineEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GraphNode {
    Evidence(Evidence),
    Finding(FindingNode),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindingNode {
    pub id: String,
    pub rule_id: String,
    pub title: String,
    pub severity: Severity,
    pub confidence: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    Spawned,
    ConnectedTo,
    Created,
    Modified,
    AssociatedWith,
    Triggered,
    GeneratedFinding,
    RelatedTo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub relationship: RelationshipType,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestigationGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestigationResult {
    pub schema_version: String,
    pub investigation_id: String,
    pub target: String,
    pub platform: String,
    pub start_time: String,
    pub end_time: String,
    pub graph: InvestigationGraph,
    pub timeline: InvestigationTimeline,
    pub findings: Vec<Finding>,
    pub risk_score: RiskScore,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestigationSummary {
    pub id: String,
    pub target: String,
    pub platform: String,
    pub start_time: String,
    pub end_time: String,
    pub finding_count: usize,
    pub risk_score: u32,
    pub priority: String,
}
