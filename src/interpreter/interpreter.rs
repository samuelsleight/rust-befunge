use crate::interpreter::core::{
    InterpreterCallback,
    InterpreterCore
};

pub struct Interpreter;

impl Interpreter {
    pub fn stage() -> InterpreterCore<Interpreter> {
        InterpreterCore::new(Interpreter)
    }
}

impl InterpreterCallback for Interpreter {
    fn output(&mut self, c: char) {
        print!("{}", c);
    }
}