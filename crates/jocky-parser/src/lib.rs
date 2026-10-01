use jocky_ast::Command;
use jocky_common::EvidenceType;
use jocky_lexer::Token;

pub fn parse(tokens: &[Token]) -> Result<Vec<Command>, String> {
    let mut commands = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Target => {
                if i + 1 < tokens.len() {
                    if let Token::StringLit(s) = &tokens[i + 1] {
                        commands.push(Command::Target(s.clone()));
                        i += 1;
                    } else {
                        return Err(format!(
                            "Expected string literal after TARGET at index {}",
                            i
                        ));
                    }
                } else {
                    return Err("Unexpected EOF after TARGET".to_string());
                }
            }
            Token::Collect => {
                if i + 1 < tokens.len() {
                    let ev_type = match &tokens[i + 1] {
                        Token::System => EvidenceType::System,
                        Token::Process => EvidenceType::Process,
                        Token::File => EvidenceType::File,
                        Token::Network => EvidenceType::Network,
                        Token::User => EvidenceType::User,
                        Token::Event => EvidenceType::Event,
                        Token::Persistence => EvidenceType::Persistence,
                        _ => {
                            return Err(format!(
                                "Expected evidence type after COLLECT at index {}",
                                i
                            ))
                        }
                    };
                    commands.push(Command::Collect { target: ev_type });
                    i += 1;
                } else {
                    return Err("Unexpected EOF after COLLECT".to_string());
                }
            }
            Token::Correlate => {
                if i + 3 < tokens.len() {
                    let left = match &tokens[i + 1] {
                        Token::Process => EvidenceType::Process,
                        _ => return Err("Unsupported left correlation type".to_string()),
                    };
                    if tokens[i + 2] != Token::With {
                        return Err("Expected WITH after left evidence type".to_string());
                    }
                    let right = match &tokens[i + 3] {
                        Token::Network => EvidenceType::Network,
                        Token::File => EvidenceType::File,
                        _ => return Err("Unsupported right correlation type".to_string()),
                    };
                    commands.push(Command::Correlate { left, right });
                    i += 3;
                } else {
                    return Err("Unexpected EOF in CORRELATE".to_string());
                }
            }
            Token::Detect => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Rules => {
                            if i + 2 < tokens.len() {
                                if let Token::StringLit(s) = &tokens[i + 2] {
                                    commands.push(Command::DetectRules(s.clone()));
                                    i += 2;
                                } else {
                                    return Err(
                                        "Expected string literal after DETECT RULES".to_string()
                                    );
                                }
                            } else {
                                return Err("Unexpected EOF in DETECT RULES".to_string());
                            }
                        }
                        Token::Identifier(id) => {
                            commands.push(Command::Detect(id.clone()));
                            i += 1;
                        }
                        _ => return Err("Expected RULES or identifier after DETECT".to_string()),
                    }
                } else {
                    return Err("Unexpected EOF in DETECT".to_string());
                }
            }
            Token::Score => {
                if i + 1 < tokens.len() && tokens[i + 1] == Token::Findings {
                    commands.push(Command::ScoreFindings);
                    i += 1;
                } else {
                    return Err("Expected FINDINGS after SCORE".to_string());
                }
            }
            Token::Report => {
                // If it's GENERATE REPORT, lexer might map GENERATE to Report.
                if i + 1 < tokens.len() && tokens[i + 1] == Token::Report {
                    commands.push(Command::GenerateReport);
                    i += 1;
                } else {
                    commands.push(Command::GenerateReport);
                }
            }
            _ => {}
        }
        i += 1;
    }

    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jocky_lexer::lex;

    #[test]
    fn test_parse() {
        let tokens =
            lex("TARGET \"LOCAL\"\nCOLLECT PROCESS\nCORRELATE PROCESS WITH NETWORK").unwrap();
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast.len(), 3);
    }
}
