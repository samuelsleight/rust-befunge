use std::{
    path::Path,
    fmt::{
        Debug,
        Formatter,
        self},
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
        LLVMAddFunction,
        LLVMDisposeModule,
        LLVMDisposeMessage,
    },
};

use crate::{
    FunctionType,
    Function
};

pub struct Module {
    module: *mut LLVMModule
}

impl Debug for Module {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unsafe {
            let s = LLVMPrintModuleToString(self.module);
            let result = writeln!(f, "{}", CStr::from_ptr(s).to_string_lossy());
            LLVMDisposeMessage(s);
            result
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

    pub fn add_function<S: AsRef<str>, T: FunctionType>(&self, name: S) -> Function<T> {
        let name = CString::new(name.as_ref()).unwrap();

        let function = unsafe {
            LLVMAddFunction(self.module, name.to_bytes_with_nul().as_ptr() as *const i8, T::function_type())
        };

        Function::new(function)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}
