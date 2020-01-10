use crate::vm::instance::exception;
use crate::vm::types::Value;
use dart_sys::Dart_NewIntegerFromUint64;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct DartInteger;

impl Value<DartInteger> {
    pub unsafe fn uint(value: u64) -> Result<Self, exception::VmError> {
        let handle = Dart_NewIntegerFromUint64(value);
        std::mem::forget(handle);
        check_if_error!(handle, {
            std::mem::forget(handle);
            Ok(Value {
                raw: handle,
                _marker: PhantomData,
            })
        })
    }
}
