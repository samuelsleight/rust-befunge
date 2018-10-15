use llvm_sys::{
    LLVMBasicBlock,
    LLVMBuilder,
    core::LLVMPositionBuilderAtEnd
};

pub struct Block {
    value: *mut LLVMBasicBlock
}

impl Block {
    pub(crate) fn new(value: *mut LLVMBasicBlock) -> Self {
        Self {
            value
        }
    }

    pub(crate) fn set_to_builder(&self, builder: *mut LLVMBuilder) {
        unsafe {
            LLVMPositionBuilderAtEnd(builder, self.value);
        }
    }
}
