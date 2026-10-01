use jocky_ast::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IrOperation {
    Target { value: String },
    Collect { target: String },
    Correlate { left: String, right: String },
    Detect { rule: String },
    DetectRules { path: String },
    Score,
    Report,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrProgram {
    pub operations: Vec<IrOperation>,
}

pub fn generate_ir(ast: &[Command]) -> IrProgram {
    let mut operations = Vec::new();
    for cmd in ast {
        match cmd {
            Command::Target(t) => operations.push(IrOperation::Target { value: t.clone() }),
            Command::Collect { target } => operations.push(IrOperation::Collect {
                target: format!("{:?}", target).to_lowercase(),
            }),
            Command::Correlate { left, right } => operations.push(IrOperation::Correlate {
                left: format!("{:?}", left).to_lowercase(),
                right: format!("{:?}", right).to_lowercase(),
            }),
            Command::Detect(rule) => operations.push(IrOperation::Detect { rule: rule.clone() }),
            Command::DetectRules(path) => {
                operations.push(IrOperation::DetectRules { path: path.clone() })
            }
            Command::ScoreFindings => operations.push(IrOperation::Score),
            Command::GenerateReport => operations.push(IrOperation::Report),
        }
    }
    IrProgram { operations }
}
