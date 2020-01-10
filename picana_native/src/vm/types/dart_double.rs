use crate::vm::instance::exception;
use crate::vm::types::Value;
use dart_sys::Dart_NewDouble;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct DartDouble;

impl Value<DartDouble> {
    pub unsafe fn uint(value: f64) -> Result<Self, exception::VmError> {
        let handle = Dart_NewDouble(value);
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
