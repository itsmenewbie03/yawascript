use core::panic;
use std::io::Read;

use crate::utils::parser::Token;

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
        let mut state = ProgState::new();
        for token in tokens {
            match *token {
                Token::Increment => state.inc(),
                Token::Decrement => state.dec(),
                Token::ShiftRight => state.shift_right(),
                Token::ShiftLeft => state.shift_left(),
                Token::Output => state.output(),
                Token::Input => state.input(),
                _ => todo!(),
            };
        }
    }
}
