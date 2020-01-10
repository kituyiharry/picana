use crate::vm::instance::exception;
use crate::vm::types::Value;
use dart_sys::{Dart_False, Dart_True};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct DartBool;

impl Value<DartBool> {
    pub unsafe fn true_() -> Result<Self, exception::VmError> {
        let handle = Dart_True();
        check_if_error!(handle, {
            std::mem::forget(handle);
            Ok(Value {
                raw: handle,
                _marker: PhantomData,
            })
        })
    }

    pub unsafe fn false_() -> Result<Self, exception::VmError> {
        let handle = Dart_False();
        check_if_error!(handle, {
            std::mem::forget(handle);
            Ok(Value {
                raw: handle,
                _marker: PhantomData,
            })
        })
    }
}
