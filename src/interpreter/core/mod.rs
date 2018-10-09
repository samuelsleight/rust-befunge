mod callback;
mod core;

pub use self::callback::{
    InterpreterCallback,
    StackValue,
    DynamicValue
};

pub use self::core::InterpreterCore;
