use crate::ValueType;

use std::marker::PhantomData;

use llvm_sys::{
    LLVMValue,
    core::LLVMConstInt
};

pub trait Constant: ValueType {
    fn constant(&self) -> *mut LLVMValue;
}

impl Constant for i32 {
    fn constant(&self) -> *mut LLVMValue {
        unsafe {
            LLVMConstInt(Self::value_type(), *self as u64, 0)
        }
    }
}

pub struct Value<T: ValueType> {
    value: *mut LLVMValue,
    phantom: PhantomData<T>
}

impl<T: ValueType> Value<T> {
    pub fn constant(t: T) -> Value<T> where T: Constant {
        Value {
            value: t.constant(),
            phantom: PhantomData
        }
    }

    pub(crate) fn value(&self) -> *mut LLVMValue {
        self.value
    }
}