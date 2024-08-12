use crate::utils::interpreter::Interpreter;
use std::{collections::HashMap, usize};

#[derive(Debug, PartialEq)]
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
                .for_each(
                    |token| match Token::parse(std::str::from_utf8(token).unwrap()) {
                        Ok(token) => {
                            tokens.push(token);
                        }
                        Err(err) => {
                            errors.push(err);
                        }
                    },
                );
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

pub fn build_brace_map(tokens: &[Token]) -> Result<HashMap<usize, usize>, String> {
    let mut brace_map: HashMap<usize, usize> = HashMap::new();
    let mut temp_brace_vec: Vec<usize> = vec![];
    for (idx, token) in tokens.iter().enumerate() {
        match *token {
            Token::LoopStart => temp_brace_vec.push(idx),
            Token::LoopEnd => {
                if temp_brace_vec.is_empty() {
                    return Err("`yAWA` requires a `YAWa` first".to_owned());
                }
                // NOTE: we are checking if the vec is empty above
                // so we can safely unwrap here
                let start = temp_brace_vec.pop().unwrap();
                let end = idx;
                brace_map.insert(start, end);
                brace_map.insert(end, start);
            }
            _ => {
                // NOTE: we can just ignore the rest of the tokens
            }
        }
    }
    if !temp_brace_vec.is_empty() {
        return Err("`YAWa` must have a corresponding `yAWA`".to_owned());
    }
    Ok(brace_map)
}

pub fn run(file: std::path::PathBuf) {
    match tokenize(file) {
        Ok(tokens) => {
            Interpreter::run(&tokens);
            println!();
        }
        Err(err) => println!("{err}"),
    }
}
