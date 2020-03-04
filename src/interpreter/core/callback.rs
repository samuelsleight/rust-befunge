pub use crate::interpreter::core::QueuedState;

#[derive(Debug, Clone)]
pub enum StackValue {
    Const(i32),
    Dynamic(DynamicValue)
}

#[derive(Debug, Clone)]
pub enum DynamicValue {
    Tagged(usize),
    Add(Box<StackValue>, Box<StackValue>),
    Mul(Box<StackValue>, Box<StackValue>),
    Sub(Box<StackValue>, Box<StackValue>),
}

impl StackValue {
    pub fn add(lhs: Self, rhs: Self) -> Self {
        StackValue::Dynamic(DynamicValue::Add(Box::new(lhs), Box::new(rhs)))
    }

    pub fn mul(lhs: Self, rhs: Self) -> Self {
        StackValue::Dynamic(DynamicValue::Mul(Box::new(lhs), Box::new(rhs)))
    }

    pub fn sub(lhs: Self, rhs: Self) -> Self {
        StackValue::Dynamic(DynamicValue::Sub(Box::new(lhs), Box::new(rhs)))
    }
}

impl From<i32> for StackValue {
    fn from(value: i32) -> Self {
        StackValue::Const(value)
    }
}

pub trait InterpreterCallback {
    type End;

    fn output(&mut self, value: StackValue);
    fn input(&mut self) -> StackValue;

    fn if_zero(&mut self, value: DynamicValue, t: QueuedState, f: QueuedState) -> Self::End;
    fn end(&mut self) -> Self::End;
}

pub trait DebugInspectable {
    fn inspect_stack(&self) -> &[StackValue];
    fn inspect_pos(&self) -> (usize, usize);
    fn inspect_next(&self) -> char;
}

pub trait DebuggerCallback<I: DebugInspectable> {
    fn debug_step(&mut self, inspectable: &I);
}
