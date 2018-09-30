pub trait InterpreterCallback {
    type End;

    fn output(&mut self, c: char);

    fn end(&mut self) -> Self::End;
}