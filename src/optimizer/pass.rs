use pipeline::Stage;

use std::marker::PhantomData;

use crate::compiler::ir::{
    Action,
    ActionValue,
    Block
};

pub struct PassWrapper<T: ?Sized>(PhantomData<T>);

pub trait Pass {
    fn optimize(input: Vec<Block>) -> Vec<Block>;

    fn new() -> PassWrapper<Self> {
        PassWrapper(PhantomData)
    }
}

impl<T> Stage<!> for PassWrapper<T> where T: Pass {
    type Input = Vec<Block>;
    type Output = Vec<Block>;

    fn run(self, input: Self::Input) -> Result<Self::Output, !> {
        Ok(T::optimize(input))
    }
}

pub struct StringPrintPass;

fn optimize_string_print(block: Block) -> Block {
    let mut actions: Vec<Action> = Vec::new();
    let mut iter = block.actions().iter().cloned().peekable();

    'outer: loop {
        loop {
            match iter.peek() {
                Some(Action::OutputChar(ActionValue::Const(_))) => break,
                Some(_) => actions.push(iter.next().unwrap()),
                None => break 'outer,
            }
        }

        let mut string = String::new();

        while let Some(Action::OutputChar(ActionValue::Const(i))) = iter.next() {
            string.push(i as u8 as char);
        }

        actions.push(Action::OutputString(string));
    }

    Block::new(actions, block.end().clone())
}

impl Pass for StringPrintPass {
    fn optimize(input: Vec<Block>) -> Vec<Block> {
        input.into_iter().map(optimize_string_print).collect()
    }
}