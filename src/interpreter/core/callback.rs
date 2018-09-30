pub trait InterpreterCallback {
    fn output(&mut self, c: char);
}