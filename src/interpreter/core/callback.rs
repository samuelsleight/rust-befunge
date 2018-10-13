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
}

impl StackValue {
    pub fn add(lhs: StackValue, rhs: StackValue) -> StackValue {
        StackValue::Dynamic(DynamicValue::Add(Box::new(lhs), Box::new(rhs)))
    }

    pub fn mul(lhs: StackValue, rhs: StackValue) -> StackValue {
        StackValue::Dynamic(DynamicValue::Mul(Box::new(lhs), Box::new(rhs)))
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
    fn end(&mut self) -> Self::End;
}

pub trait DebugInspectable {
    fn inspect_stack(&self) -> &[StackValue];
    fn inspect_pos(&self) -> (u32, u32);
    fn inspect_next(&self) -> char;
}

pub trait DebuggerCallback<I: DebugInspectable> {
    fn debug_step(&self, inspectable: &I);
}