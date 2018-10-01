use llvm_sys::{
    LLVMBuilder,
    core::{
        LLVMCreateBuilder,
        LLVMDisposeBuilder,
        LLVMBuildRet,
    }
};

use crate::{
    Block,
    Function,
    FunctionType,
    Value
};

pub struct Builder {
    builder: *mut LLVMBuilder
}

impl Builder {
    pub fn new() -> Builder {
        let builder = unsafe {
            LLVMCreateBuilder()
        };

        Builder {
            builder
        }
    }

    pub fn set_block(&self, block: &Block) {
        block.set_to_builder(self.builder);
    }

    pub fn build_call<T: FunctionType>(&self, function: &Function<T>, params: T::Params) {
        function.build_call(self.builder, params)
    }

    pub fn build_ret(&self) {
        unsafe {
            LLVMBuildRet(self.builder, Value::<i32>::constant(0).value());
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}