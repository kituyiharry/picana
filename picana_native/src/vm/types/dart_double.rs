#[macro_use]
use super::vm;
use super::vm::instance::exception;
use types::sys::{Dart_NewDouble};
use types::Value;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct DartDouble;

impl Value<DartDouble> {
    pub unsafe fn uint(value: u64) -> Result<Self, exception::VmError> {
        let handle = Dart_NewDouble(uint);
        std::mem::forget(handle);
        check_if_error! handle, {
            std::mem::forget(handle);
            Ok(Value{
            raw: handle,
            _marker: PhantomData,
        })}
    }
}
