use pipeline::Stage;

use crate::{
    error::Error,
    compiler::ir::{
        Block, Action, End
    },
    interpreter::{
        Grid,
        core::{InterpreterCallback, InterpreterCore},
    }
};

pub struct Compiler {}

pub struct State {
    actions: Vec<Action>
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }
}

impl State {
    fn new() -> State {
        State {
            actions: Vec::new()
        }
    }

    fn compile(self, grid: Grid<char>) -> Result<Block, Error> {
        InterpreterCore::new(self).run(grid)
    }
}

impl InterpreterCallback for State {
    type End = Block;

    fn output(&mut self, c: char) {
        self.actions.push(Action::OutputChar(c));
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