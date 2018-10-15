use pipeline::Stage;

use std::{
    char,
    marker::PhantomData
};

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

pub trait BlockPass {
    fn optimize_block(block: Block) -> Block;
}

impl<T> Stage<!> for PassWrapper<T> where T: Pass {
    type Input = Vec<Block>;
    type Output = Vec<Block>;

    fn run(self, input: Self::Input) -> Result<Self::Output, !> {
        Ok(T::optimize(input))
    }
}

impl<T> Pass for T where T: BlockPass {
    fn optimize(input: Vec<Block>) -> Vec<Block> {
        input.into_iter().map(Self::optimize_block).collect()
    }
}

pub struct StringPrintPass;

impl BlockPass for StringPrintPass {
    fn optimize_block(block: Block) -> Block {
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
                string.push(unsafe { char::from_u32_unchecked(i as u32) });
            }

            actions.push(Action::OutputString(string));
        }

        Block::new(actions, block.end().clone())
    }
}
