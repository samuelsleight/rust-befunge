use std::marker::PhantomData;

use llvm_sys::{
    LLVMValue,
    LLVMBuilder,
    core::LLVMAppendBasicBlock,
};

use crate::{
    Block,
    FunctionType
};

use std::ffi::CString;

pub struct Function<T: FunctionType> {
    value: *mut LLVMValue,
    phantom: PhantomData<T>
}

impl<T: FunctionType> Function<T> {
    pub(crate) fn new(value: *mut LLVMValue) -> Function<T> {
        Function {
            value,
            phantom: PhantomData
        }
    }

    pub(crate) fn build_call(&self, builder: *mut LLVMBuilder, params: T::Params) {
        T::build_call(builder, self.value, params)
    }

    pub fn add_block<S: AsRef<str>>(&self, name: S) -> Block {
        let name = CString::new(name.as_ref()).unwrap();

        let block = unsafe {
            LLVMAppendBasicBlock(self.value, name.to_bytes_with_nul().as_ptr() as *const i8)
        };

        Block::new(block)
    }
}