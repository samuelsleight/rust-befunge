use pipeline::Stage;

use crate::{
    error::Error,
    interpreter::{
        Error as InterpreterError,
        core::{
            InterpreterCallback,
            StackValue,
        },
        grid::{
            Grid,
            Ip,
            Delta,
        }
    }
};

pub struct InterpreterCore<Callback> {
    callback: Callback
}

struct State {
    grid: Grid<char>,
    ip: Ip,

    delta: Option<Delta>,
    stack: Vec<StackValue>,
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

    fn push<T: Into<StackValue>>(&mut self, value: T) {
        self.stack.push(value.into());
    }

    fn pop(&mut self) -> StackValue {
        self.stack.pop().unwrap_or(StackValue::Const(0))
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

impl<Callback> Stage<Error> for InterpreterCore<Callback> where Callback: InterpreterCallback {
    type Input = Grid<char>;
    type Output = Callback::End;

    fn run(mut self, input: Self::Input) -> Result<Self::Output, Error> {
        let mut state = State::new(input);

        while let Some(c) = state.next() {
            match c {
                // Stringmode
                '"' => state.toggle_stringmode(),
                c if state.stringmode() => state.push(c as i32),

                // Simple Movement
                ' ' => (),
                '<' => state.set_delta(Delta::Left),
                '>' => state.set_delta(Delta::Right),
                '^' => state.set_delta(Delta::Up),
                'v' => state.set_delta(Delta::Down),
                '#' => state.advance(),

                // Value Pushing
                c @ '0' ... '9' => state.push((c as u8 - b'0') as i32),

                // Addition
                '+' => match (state.pop(), state.pop()) {
                    (StackValue::Const(lhs), StackValue::Const(rhs)) => state.push(lhs + rhs),
                    (lhs, rhs) => state.push(StackValue::add(lhs, rhs))
                },

                // Multiplication
                '*' => match (state.pop(), state.pop()) {
                    (StackValue::Const(lhs), StackValue::Const(rhs)) => state.push(lhs * rhs),
                    (lhs, rhs) => state.push(StackValue::mul(lhs, rhs))
                },

                // Duplication
                ':' => match state.pop() {
                    StackValue::Const(value) => {
                        state.push(value);
                        state.push(value);
                    },

                    _ => unimplemented!("Duplication of non-const values is not yet implemented")
                },

                // If (Horizontal)
                '_' => match state.pop() {
                    StackValue::Const(value) => if value == 0 {
                        state.set_delta(Delta::Right);
                    }
                    else {
                        state.set_delta(Delta::Left);
                    },

                    _ => unimplemented!("Comparison of non-const values is not yet implemented")
                },


                // Char IO
                '~' => state.push(self.callback.input()),
                ',' => self.callback.output(state.pop()),

                // End
                '@' => return Ok(self.callback.end()),

                c => unimplemented!("The interpreter hit an unimplemented instruction: '{}'", c)
            }
        }

        Err(Error::Interpreter(InterpreterError::EOF))
    }
}

impl<Callback> InterpreterCore<Callback> where Callback: InterpreterCallback {
    pub fn new(callback: Callback) -> InterpreterCore<Callback> {
        InterpreterCore {
            callback
        }
    }
}


