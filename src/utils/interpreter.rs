use core::panic;
use std::io::Read;

use crate::utils::parser::Token;

use super::parser::build_brace_map;

#[derive(Debug)]
struct ProgState {
    tape: Vec<u8>,
    data_ptr: usize,
}

impl ProgState {
    fn new() -> Self {
        ProgState {
            tape: vec![0; 30_000],
            data_ptr: 0,
        }
    }

    fn cur_cell_val(&self) -> u8 {
        self.tape[self.data_ptr]
    }

    fn inc(&mut self) {
        let cur_val = self.tape[self.data_ptr];
        if cur_val == 255 {
            self.tape[self.data_ptr] = 0;
        } else {
            self.tape[self.data_ptr] += 1;
        }
    }

    fn dec(&mut self) {
        let cur_val = self.tape[self.data_ptr];
        if cur_val == 0 {
            self.tape[self.data_ptr] = 255;
        } else {
            self.tape[self.data_ptr] -= 1;
        }
    }

    fn shift_right(&mut self) {
        self.data_ptr += 1;
    }

    fn shift_left(&mut self) {
        self.data_ptr -= 1;
    }

    fn output(&self) {
        print!("{}", self.tape[self.data_ptr] as char);
    }

    fn input(&mut self) {
        let mut buf = [0; 1];
        std::io::stdin()
            .read_exact(&mut buf)
            .expect("Failed to read input");
        if !buf[0].is_ascii() {
            panic!("Non ASCII Character Entered (value={})", buf[0]);
        }
        self.tape[self.data_ptr] = buf[0];
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn run(tokens: &[Token]) {
        let brace_map = build_brace_map(tokens);
        if let Err(err) = brace_map {
            panic!("{}", err);
        }
        // NOTE: we will just panic for the Err variant so we can safely
        // shadow the brace_map variable
        let brace_map = brace_map.unwrap();
        let mut state = ProgState::new();
        let mut current_idx = 0;
        // NOTE: for .. in .. is forward only let's manually handle the loop
        loop {
            if current_idx == tokens.len() {
                break;
            }
            let token = &tokens[current_idx];
            match token {
                Token::Increment => state.inc(),
                Token::Decrement => state.dec(),
                Token::ShiftRight => state.shift_right(),
                Token::ShiftLeft => state.shift_left(),
                Token::Output => state.output(),
                Token::Input => state.input(),
                Token::LoopStart => {
                    if state.cur_cell_val() != 0 {
                        current_idx += 1;
                        continue;
                    } else {
                        current_idx = *brace_map
                            .get(&current_idx)
                            .expect("IMPOSSIBLE 100% Rust BUG!");
                    }
                }
                Token::LoopEnd => {
                    if state.cur_cell_val() != 0 {
                        current_idx = *brace_map
                            .get(&current_idx)
                            .expect("IMPOSSIBLE 100% Rust BUG!");
                    }
                }
            };
            current_idx += 1;
        }
    }
}
