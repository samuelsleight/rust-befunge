use crate::interpreter::core::{
    StackValue,
    InterpreterCallback,
    DebuggerCallback,
    DebugInspectable,
    InterpreterCore
};

use std::{
    mem, char,
    io::{stdin, Read},
};

pub struct Interpreter;
pub struct NullDebugger;

impl Interpreter {
    pub fn stage() -> InterpreterCore<Self, NullDebugger> {
        InterpreterCore::new(Interpreter, NullDebugger)
    }
}

impl InterpreterCallback for Interpreter {
    type End = ();

    fn output(&mut self, value: StackValue) {
        match value {
            StackValue::Const(i) => print!("{}", unsafe { char::from_u32_unchecked(i as u32) }),
            _ => panic!("Interpreter output received a dynamic value")
        }
    }

    fn input(&mut self) -> StackValue {
        let mut buf: [u8; 1] = unsafe { mem::uninitialized() };
        stdin()
            .read(&mut buf)
            .map(|_| buf[0] as char as i32)
            .map(StackValue::Const)
            .expect("Unable to read character from input")
    }

    fn end(&mut self) {}
}

impl<I: DebugInspectable> DebuggerCallback<I> for NullDebugger {
    fn debug_step(&mut self, _: &I) {}
}
