#[macro_use]
use super::vm;
use super::vm::instance::exception;
use types::sys::{Dart_NewIntegerFromUint64};
use types::Value;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct DartInteger;

impl Value<DartList> {
    pub unsafe fn uint(value: u64) -> Result<Self, exception::VmError> {
        let handle = Dart_NewIntegerFromUint64(uint);
        std::mem::forget(handle);
        check_if_error! handle, {
            std::mem::forget(handle);
            Ok(Value{
            raw: handle,
            _marker: PhantomData,
        }) }
    }
}
