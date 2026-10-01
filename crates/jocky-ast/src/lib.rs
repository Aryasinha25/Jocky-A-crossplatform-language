use jocky_common::EvidenceType;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Target(String),
    Collect {
        target: EvidenceType,
    },
    Correlate {
        left: EvidenceType,
        right: EvidenceType,
    },
    Detect(String),
    DetectRules(String),
    ScoreFindings,
    GenerateReport,
}
