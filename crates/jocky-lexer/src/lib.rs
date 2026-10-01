#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Target,
    Collect,
    Analyze,
    Correlate,
    Detect,
    Rules,
    Score,
    Timeline,
    Report,
    Export,
    With,
    System,
    Process,
    File,
    Network,
    User,
    Event,
    Persistence,
    Findings,
    StringLit(String),
    Identifier(String),
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut iter = input.split_whitespace();

    while let Some(word) = iter.next() {
        match word {
            "TARGET" => tokens.push(Token::Target),
            "COLLECT" => tokens.push(Token::Collect),
            "ANALYZE" => tokens.push(Token::Analyze),
            "CORRELATE" => tokens.push(Token::Correlate),
            "DETECT" => tokens.push(Token::Detect),
            "RULES" => tokens.push(Token::Rules),
            "SCORE" => tokens.push(Token::Score),
            "TIMELINE" => tokens.push(Token::Timeline),
            "REPORT" | "GENERATE" => tokens.push(Token::Report), // handle GENERATE REPORT
            "EXPORT" => tokens.push(Token::Export),
            "WITH" => tokens.push(Token::With),
            "SYSTEM" => tokens.push(Token::System),
            "PROCESS" => tokens.push(Token::Process),
            "FILE" | "FILES" => tokens.push(Token::File),
            "NETWORK" => tokens.push(Token::Network),
            "USER" | "USERS" => tokens.push(Token::User),
            "EVENT" => tokens.push(Token::Event),
            "PERSISTENCE" => tokens.push(Token::Persistence),
            "FINDINGS" => tokens.push(Token::Findings),
            _ => {
                if word.starts_with('"') && word.ends_with('"') {
                    tokens.push(Token::StringLit(word[1..word.len() - 1].to_string()));
                } else if word == "REPORT" {
                    tokens.push(Token::Report); // Used with GENERATE REPORT
                } else {
                    tokens.push(Token::Identifier(word.to_string()));
                }
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "TARGET \"LOCAL\"\nCOLLECT PROCESS\nCORRELATE PROCESS WITH NETWORK";
        let tokens = lex(input).unwrap();
        assert_eq!(tokens[0], Token::Target);
        assert_eq!(tokens[1], Token::StringLit("LOCAL".to_string()));
        assert_eq!(tokens[2], Token::Collect);
        assert_eq!(tokens[3], Token::Process);
        assert_eq!(tokens[4], Token::Correlate);
        assert_eq!(tokens[5], Token::Process);
        assert_eq!(tokens[6], Token::With);
        assert_eq!(tokens[7], Token::Network);
    }
}
