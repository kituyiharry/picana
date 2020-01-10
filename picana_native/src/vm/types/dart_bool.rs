#[macro_use]
use super::vm;
use super::vm::instance::exception;
use types::sys::{Dart_True, Dart_False};
use types::Value;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct DartBool;

impl Value<DartBool> {
    pub unsafe fn true_() -> Result<Self, exception::VmError> {
        let handle = Dart_True();
        check_if_error! handle, { 
            std::mem::forget(handle);
            Ok(Value{
                raw: handle,
                _marker: PhantomData;
            }) }
    }

    pub unsafe fn false_() -> Result<Self, exception::VmError> {
        let handle = Dart_False();
        check_if_error! handle, { 
            std::mem::forget(handle);
            Ok(Value{
                raw: handle,
                _marker: PhantomData;
            }) }
    }
}
