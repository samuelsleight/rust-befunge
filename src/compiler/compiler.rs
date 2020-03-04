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
            QueuedState,
            StackValue,
            DynamicValue,
            InterpreterCallback,
            InterpreterCore
        },
    }
};

pub struct Compiler {
    tag: usize,
    idx: usize,
    queue: Vec<QueuedState>
}

pub struct State<'a> {
    compiler: &'a mut Compiler,
    actions: Vec<Action>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            tag: 0,
            idx: 0,
            queue: Vec::new()
        }
    }

    pub fn tag(&mut self) -> usize {
        self.tag += 1;
        self.tag - 1
    }

    fn push(&mut self, state: QueuedState) -> usize {
        self.queue.push(state);
        self.idx += 1;
        self.idx - 1
    }

    fn pop(&mut self) -> Option<QueuedState> {
        self.queue.pop()
    }
}

impl<'a> State<'a> {
    fn new(compiler: &'a mut Compiler) -> Self {
        Self {
            compiler,
            actions: Vec::new(),
        }
    }

    fn compile(self, state: QueuedState) -> Result<Block, Error> {
        InterpreterCore::new(self, NullDebugger).interpret(state)
    }
}

impl<'a> InterpreterCallback for State<'a> {
    type End = Block;

    fn output(&mut self, value: StackValue) {
        match value {
            StackValue::Const(i) => self.actions.push(Action::OutputChar(ActionValue::Const(i))),
            StackValue::Dynamic(value) => self.actions.push(Action::OutputChar(ActionValue::Dynamic(value)))
        }
    }

    fn input(&mut self) -> StackValue {
        let tag = self.compiler.tag();
        self.actions.push(Action::Input(tag));
        StackValue::Dynamic(DynamicValue::Tagged(tag))
    }

    fn if_zero(&mut self, value: DynamicValue, t: QueuedState, f: QueuedState) -> Self::End {
        let t_idx = self.compiler.push(t);
        let f_idx = self.compiler.push(f);
        Block::new(self.actions.clone(), End::If(ActionValue::Dynamic(value), t_idx, f_idx))
    }

    fn end(&mut self) -> Self::End {
        Block::new(self.actions.clone(), End::End)
    }
}

impl Stage<Error> for Compiler {
    type Input = Grid<char>;
    type Output = Vec<Block>;

    fn run(mut self, input: Self::Input) -> Result<Self::Output, Error> {
        self.push(input.into());

        let mut blocks = Vec::new();

        while let Some(state) = self.pop() {
            blocks.push(State::new(&mut self).compile(state)?)
        }

        Ok(blocks)
    }

}
