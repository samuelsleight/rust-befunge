use pipeline::Stage;

use crate::{
    error::Error,
    interpreter::{
        Error as InterpreterError,
        core::{
            InterpreterCallback,
            DebuggerCallback,
            DebugInspectable,
            StackValue,
        },
        grid::{
            Grid,
            Ip,
            Delta,
        }
    }
};

pub struct InterpreterCore<Callback, Debugger> {
    callback: Callback,
    debugger: Debugger,
}

#[derive(Clone, Copy, PartialEq)]
enum Stringmode {
    Not,
    Once,
    Stringmode
}

pub struct State {
    grid: Grid<char>,
    ip: Ip,

    delta: Option<Delta>,
    stack: Vec<StackValue>,
    stringmode: Stringmode,
}

impl State {
    fn new(grid: Grid<char>) -> Self {
        let ip = grid.ip();

        Self {
            grid,
            ip,

            delta: None,
            stack: Vec::new(),
            stringmode: Stringmode::Not,
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
        match self.stringmode {
            Stringmode::Not => self.stringmode = Stringmode::Stringmode,
            Stringmode::Once | Stringmode::Stringmode => self.stringmode = Stringmode::Not,
        }
    }

    fn once_stringmode(&mut self) {
        self.stringmode = Stringmode::Once
    }

    fn stringmode(&self) -> Stringmode {
        self.stringmode
    }
}

impl DebugInspectable for State {
    fn inspect_stack(&self) -> &[StackValue] {
        &self.stack
    }

    fn inspect_pos(&self) -> (usize, usize) {
        (self.ip.row(), self.ip.col())
    }

    fn inspect_next(&self) -> char {
        self.grid[self.ip]
    }
}

impl<Callback, Debugger> Stage<Error> for InterpreterCore<Callback, Debugger>
where
    Callback: InterpreterCallback,
    Debugger: DebuggerCallback<State>
{
    type Input = Grid<char>;
    type Output = Callback::End;

    fn run(mut self, input: Self::Input) -> Result<Self::Output, Error> {
        let mut state = State::new(input);

        while let Some(c) = state.next() {
            self.debugger.debug_step(&state);

            match c {
                // Stringmode
                '"' => state.toggle_stringmode(),

                c if state.stringmode() != Stringmode::Not => {
                    state.push(c as i32);

                    if state.stringmode() == Stringmode::Once {
                        state.toggle_stringmode();
                    }
                },

                '\'' => state.once_stringmode(),

                // Simple Movement
                ' ' => (),
                '<' => state.set_delta(Delta::Left),
                '>' => state.set_delta(Delta::Right),
                '^' => state.set_delta(Delta::Up),
                'v' => state.set_delta(Delta::Down),
                '#' => state.advance(),

                // Value Pushing
                c @ '0' ... '9' => state.push(i32::from(c as u8 - b'0')),
                c @ 'a' ... 'f' => state.push(i32::from((c as u8 + 10) - b'a')),

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

                // Subtraction
                '-' => match (state.pop(), state.pop()) {
                    (StackValue::Const(lhs), StackValue::Const(rhs)) => state.push(lhs - rhs),
                    (_, _) => unimplemented!("Subtraction of non-const values is not yet implemented")
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

                // If (Vertical)
                '|' => match state.pop() {
                    StackValue::Const(value) => if value == 0 {
                        state.set_delta(Delta::Down);
                    }
                    else {
                        state.set_delta(Delta::Up);
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

impl<Callback, Debugger> InterpreterCore<Callback, Debugger>
where
    Callback: InterpreterCallback,
    Debugger: DebuggerCallback<State>
{
    pub fn new(callback: Callback, debugger: Debugger) -> Self {
        Self {
            callback,
            debugger
        }
    }
}
