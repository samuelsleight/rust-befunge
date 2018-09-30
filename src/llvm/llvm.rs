use std::{
    path::Path,
    ffi::{
        CStr, 
        CString
    },
};

use llvm_sys::{
    LLVMModule,
    core::{
        LLVMModuleCreateWithName,
        LLVMSetSourceFileName,
        LLVMPrintModuleToString,
        LLVMDisposeModule,
        LLVMDisposeMessage,
    },
};

use crate::inspector::Inspectable;

pub struct Module {
    module: *mut LLVMModule
}

impl Inspectable for Module {
    fn inspect(&self) {
        unsafe {
            let s = LLVMPrintModuleToString(self.module);
            println!("{}", CStr::from_ptr(s).to_string_lossy());
            LLVMDisposeMessage(s);
        }
    }
}

impl Module {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(name: S, source: P) -> Module {
        let name = CString::new(name.as_ref()).unwrap();
        let source = CString::new(source.as_ref().as_os_str().to_str().unwrap()).unwrap();

        let module = unsafe {
            let module = LLVMModuleCreateWithName(name.to_bytes_with_nul().as_ptr() as *const i8);

            let source_bytes = source.to_bytes();
            LLVMSetSourceFileName(module, source_bytes.as_ptr() as *const i8, source_bytes.len());

            module
        };

        Module {
            module
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}
