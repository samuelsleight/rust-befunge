use std::ffi::CString;

use llvm_sys::{
    LLVMType,
    LLVMValue,
    LLVMBuilder,
    core::{
        LLVMVoidType,
        LLVMInt32Type,
        LLVMFunctionType,
        LLVMBuildCall,
    }
};

use crate::Value;

pub trait FunctionType {
    type Params;

    fn function_type() -> *mut LLVMType;
    fn build_call(builder: *mut LLVMBuilder, function: *mut LLVMValue, params: Self::Params);
}

pub trait ValueType {
    fn value_type() -> *mut LLVMType;
}

macro_rules! value_type {
    ($t:ty => $e:expr) => {
        impl ValueType for $t {
            fn value_type() -> *mut LLVMType {
                unsafe {
                    $e
                }
            }
        }
    }
}

value_type!(() => LLVMVoidType());
value_type!(i32 => LLVMInt32Type());

fn function_type(ret: *mut LLVMType, params: Vec<*mut LLVMType>) -> *mut LLVMType {
    unsafe {
        LLVMFunctionType(ret, params.as_ptr() as *mut _, params.len() as u32, 0)
    }
}

fn build_call(builder: *mut LLVMBuilder, function: *mut LLVMValue, params: Vec<*mut LLVMValue>)
{
    unsafe {
        let name = CString::new("").unwrap();
        LLVMBuildCall(builder, function, params.as_ptr() as *mut _, params.len() as u32, name.to_bytes_with_nul().as_ptr() as *const i8);
    }
}

impl<R> FunctionType for fn() -> R where R: ValueType {
    type Params = ();

    fn function_type() -> *mut LLVMType {
        function_type(R::value_type(), vec![])
    }

    fn build_call(builder: *mut LLVMBuilder, function: *mut LLVMValue, _: Self::Params) {
        build_call(builder, function, vec![])
    }
}

impl<T, R> FunctionType for fn(T) -> R where R: ValueType, T: ValueType {
    type Params = (Value<T>,);

    fn function_type() -> *mut LLVMType {
        function_type(R::value_type(), vec![T::value_type()])
    }

    fn build_call(builder: *mut LLVMBuilder, function: *mut LLVMValue, params: Self::Params) {
        build_call(builder, function, vec![params.0.value()])
    }
}
