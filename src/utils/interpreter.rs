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
        self.tape[self.data_ptr] += 1;
    }

    fn dec(&mut self) {
        self.tape[self.data_ptr] -= 1;
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
}

pub struct Interpreter {}

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
                _ => todo!(),
            };
        }
    }
}
