use pipeline::Stage;

use crate::{
    error::Error,
    interpreter::grid::{
        Grid,
        Ip,
        Delta,
    }
};

pub struct Interpreter {}

struct State {
    grid: Grid<char>,
    ip: Ip,

    delta: Option<Delta>,
    stack: Vec<i32>,
    stringmode: bool,
}

impl State {
    fn new(grid: Grid<char>) -> State {
        let ip = grid.ip();

        State {
            grid,
            ip,

            delta: None,
            stack: Vec::new(),
            stringmode: false,
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(delta) = self.delta {
            self.ip.advance(delta);
        }
        else {
            self.delta = Delta::Right.into();
        }

        Some(self.grid[self.ip])
    }

    fn advance(&mut self) {
        self.ip.advance(self.delta.unwrap_or(Delta::Right));
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(0 )
    }

    fn set_delta(&mut self, delta: Delta) {
        self.delta = delta.into();
    }

    fn toggle_stringmode(&mut self) {
        self.stringmode = !self.stringmode;
    }

    fn stringmode(&self) -> bool {
        self.stringmode
    }
}

impl Stage<Error> for Interpreter {
    type Input = Grid<char>;
    type Output = ();

    fn run(self, input: Self::Input) -> Result<Self::Output, Error> {
        let mut state = State::new(input);

        while let Some(c) = state.next() {
            match c {
                // Stringmode
                '"' => state.toggle_stringmode(),
                c if state.stringmode() => state.push(c as i32),

                // Simple Movement
                '<' => state.set_delta(Delta::Left),
                '>' => state.set_delta(Delta::Right),
                '^' => state.set_delta(Delta::Up),
                'v' => state.set_delta(Delta::Down),
                '#' => state.advance(),

                // Value Pushing
                c @ '0' ... '9' => state.push((c as u8 - b'0') as i32),

                // Multiplication
                '*' => {
                    let lhs = state.pop();
                    let rhs = state.pop();
                    state.push(lhs * rhs);
                },

                // Duplication
                ':' => {
                    let value = state.pop();
                    state.push(value);
                    state.push(value);
                }

                // If (Horizontal)
                '_' => if state.pop() == 0 {
                    state.set_delta(Delta::Right);
                }
                else {
                    state.set_delta(Delta::Left);
                }

                // Output Char
                ',' => print!("{}", state.pop() as u8 as char),

                // End
                '@' => return Ok(()),

                _ => unimplemented!()
            }
        }

        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }
}

