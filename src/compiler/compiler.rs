use pipeline::Stage;

use crate::{
    error::Error,
    compiler::ir::{
        Block,
        Action,
        ActionValue,
        End
    },
    interpreter::{
        Grid,
        NullDebugger,
        core::{
            StackValue,
            DynamicValue,
            InterpreterCallback,
            InterpreterCore
        },
    }
};

pub struct Compiler {}

pub struct State {
    actions: Vec<Action>,
    tag: usize
}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }
}

impl State {
    fn new() -> Self {
        Self {
            actions: Vec::new(),
            tag: 0
        }
    }

    fn compile(self, grid: Grid<char>) -> Result<Block, Error> {
        InterpreterCore::new(self, NullDebugger).run(grid)
    }
}

impl InterpreterCallback for State {
    type End = Block;

    fn output(&mut self, value: StackValue) {
        match value {
            StackValue::Const(i) => self.actions.push(Action::OutputChar(ActionValue::Const(i))),
            StackValue::Dynamic(value) => self.actions.push(Action::OutputChar(ActionValue::Dynamic(value)))
        }
    }

    fn input(&mut self) -> StackValue {
        self.actions.push(Action::Input(self.tag));
        self.tag += 1;
        StackValue::Dynamic(DynamicValue::Tagged(self.tag - 1))
    }

    fn end(&mut self) -> Self::End {
        Block::new(self.actions.clone(), End::End)
    }
}

impl Stage<Error> for Compiler {
    type Input = Grid<char>;
    type Output = Vec<Block>;

    fn run(self, input: Self::Input) -> Result<Self::Output, Error> {
        State::new()
            .compile(input)
            .map(|block| vec![block])
    }

}
