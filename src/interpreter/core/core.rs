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

#[derive(Clone)]
pub struct State {
    grid: Grid<char>,
    ip: Ip,

    delta: Option<Delta>,
    stack: Vec<StackValue>,
    stringmode: Stringmode,
}

pub struct QueuedState(State);

impl From<Grid<char>> for QueuedState {
    fn from(grid: Grid<char>) -> Self {
        QueuedState(State::new(grid, None))
    }
}

impl State {
    fn new(grid: Grid<char>, ip: Option<Ip>) -> Self {
        let ip = ip.unwrap_or_else(|| grid.ip());

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

    fn with_delta(&self, delta: Delta) -> Self {
        Self {
            delta: Some(delta),
            ..self.clone()
        }
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

    fn run(self, input: Self::Input) -> Result<Self::Output, Error> {
        self.interpret(input.into())
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

    pub fn interpret(mut self, state: QueuedState) -> Result<<Self as Stage<Error>>::Output, Error> {
        let mut state = state.0;

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
                c @ '0' ..= '9' => state.push(i32::from(c as u8 - b'0')),
                c @ 'a' ..= 'f' => state.push(i32::from((c as u8 + 10) - b'a')),

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
                    (lhs, rhs) => state.push(StackValue::sub(lhs, rhs))
                },

                // Duplication
                ':' => match state.pop() {
                    StackValue::Const(value) => {
                        state.push(value);
                        state.push(value);
                    },

                    StackValue::Dynamic(value) => {
                        let value = self.callback.duplicate(value);
                        state.push(value.clone());
                        state.push(value);
                    },
                },

                // If (Horizontal)
                '_' => match state.pop() {
                    StackValue::Const(value) => if value == 0 {
                        state.set_delta(Delta::Right);
                    }
                    else {
                        state.set_delta(Delta::Left);
                    },

                    StackValue::Dynamic(value) => return Ok(self.callback.if_zero(value, QueuedState(state.with_delta(Delta::Right)), QueuedState(state.with_delta(Delta::Left))))
                },

                // If (Vertical)
                '|' => match state.pop() {
                    StackValue::Const(value) => if value == 0 {
                        state.set_delta(Delta::Down);
                    }
                    else {
                        state.set_delta(Delta::Up);
                    },

                    StackValue::Dynamic(value) => return Ok(self.callback.if_zero(value, QueuedState(state.with_delta(Delta::Down)), QueuedState(state.with_delta(Delta::Up))))
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
