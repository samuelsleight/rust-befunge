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
    LLVMLinkage,
    core::{
        LLVMArrayType,
        LLVMInt8Type,
        LLVMModuleCreateWithName,
        LLVMSetSourceFileName,
        LLVMPrintModuleToString,
        LLVMAddFunction,
        LLVMDisposeModule,
        LLVMDisposeMessage,
        LLVMConstString,
        LLVMAddGlobal,
        LLVMSetLinkage,
        LLVMSetGlobalConstant,
        LLVMSetInitializer,
        LLVMConstBitCast
    },
};

use crate::{
    types::ValueType,
    FunctionType,
    Function,
    Value
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

    pub fn add_string<S: AsRef<str>>(&self, string: S) -> Value<String> {
        let string = CString::new(string.as_ref()).unwrap();
        let bytes = string.to_bytes_with_nul();

        let global = {
            let name = CString::new("string").unwrap();

            unsafe {
                LLVMAddGlobal(self.module, LLVMArrayType(LLVMInt8Type(), bytes.len() as u32), name.to_bytes_with_nul().as_ptr() as *const i8)
            }
        };

        let string = unsafe {
            LLVMConstString(bytes.as_ptr() as *const i8, bytes.len() as u32, 1)
        };

        unsafe {
            LLVMSetLinkage(global, LLVMLinkage::LLVMInternalLinkage);
            LLVMSetGlobalConstant(global, 1);
            LLVMSetInitializer(global, string);

            Value::new(LLVMConstBitCast(global, String::value_type()))
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
