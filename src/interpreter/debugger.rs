use crate::interpreter::{
    Interpreter,
    core::{
        DebuggerCallback,
        DebugInspectable,
        InterpreterCore
    }
};

pub struct Debugger;

impl Debugger {
    pub fn stage() -> InterpreterCore<Interpreter, Debugger> {
        InterpreterCore::new(Interpreter, Debugger)
    }
}

impl<I: DebugInspectable> DebuggerCallback<I> for Debugger {
    fn debug_step(&self, inspectable: &I) {
        println!();
        println!("Stack: {:?}", inspectable.inspect_stack());
        println!("Next: {:?}: {}", inspectable.inspect_pos(), inspectable.inspect_next());
    }
}