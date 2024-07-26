use std::io::Read;

#[derive(Debug)]
pub enum Token {
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

impl Token {
    pub fn parse(input: &str) -> Result<Self, String> {
        match input {
            "yawA" => Ok(Token::ShiftRight),
            "Yawa" => Ok(Token::ShiftLeft),
            "yaWA" => Ok(Token::Increment),
            "YAwa" => Ok(Token::Decrement),
            "yawa" => Ok(Token::Output),
            "YAWA" => Ok(Token::Input),
            "YAWa" => Ok(Token::LoopStart),
            "yAWA" => Ok(Token::LoopEnd),
            _ => Err(format!("{} is not a valid command!", input)),
        }
    }
}

pub fn tokenize(file: std::path::PathBuf) -> Result<Vec<Token>, String> {
    let content = std::fs::read_to_string(file);
    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<String> = vec![];
    match content {
        Ok(content) => {
            content
                .chars()
                .filter(|c| !c.is_whitespace() && c.is_ascii())
                .collect::<String>()
                .as_bytes()
                .chunks(4)
                .for_each(|mut token| {
                    let mut tokenstr = String::new();
                    token.read_to_string(&mut tokenstr).unwrap();
                    match Token::parse(&tokenstr) {
                        Ok(token) => {
                            tokens.push(token);
                        }
                        Err(err) => {
                            errors.push(err);
                        }
                    }
                });
            if !errors.is_empty() {
                // TODO: provide the index of the error
                Err(format!("Syntax Error:\n{}", errors.join("\n")))
            } else {
                Ok(tokens)
            }
        }
        Err(_) => Err("Failed to read the file.".to_owned()),
    }
}

pub fn run(file: std::path::PathBuf) {
    match tokenize(file) {
        Ok(tokens) => println!("Parsed Successs\nTotal Tokens: {}", tokens.len()),
        Err(err) => println!("{err}"),
    }
}
