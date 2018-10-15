use std::ffi::CString;

use llvm_sys::{
    LLVMBuilder,
    core::{
        LLVMCreateBuilder,
        LLVMDisposeBuilder,
        LLVMBuildRet,
        LLVMBuildAdd,
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

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        let builder = unsafe {
            LLVMCreateBuilder()
        };

        Self {
            builder
        }
    }

    pub fn set_block(&self, block: &Block) {
        block.set_to_builder(self.builder);
    }

    pub fn build_call<T: FunctionType>(&self, function: &Function<T>, params: T::Params) -> T::Return {
        function.build_call(self.builder, params)
    }

    pub fn build_add(&self, lhs: &Value<i32>, rhs: &Value<i32>) -> Value<i32> {
        unsafe {
            let name = CString::new("").unwrap();
            Value::new(LLVMBuildAdd(self.builder, lhs.value(), rhs.value(), name.to_bytes_with_nul().as_ptr() as *const i8))
        }
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
