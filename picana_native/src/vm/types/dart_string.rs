use crate::vm::instance::exception;
use crate::vm::types::Value;
use dart_sys::{Dart_NewStringFromCString, Dart_NewStringFromUTF8, Dart_StringStorageSize};
use std::ffi::CString;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct DartString;

//Example of passing arguments to Invoke
///Creates a list of length size to hold elements
impl Value<DartString> {
    pub unsafe fn from_c_string(
        value: *const std::os::raw::c_char,
    ) -> Result<Self, exception::VmError> {
        let handle = Dart_NewStringFromCString(value);
        check_if_error!(handle, {
            std::mem::forget(handle);
            Ok(Value {
                raw: handle,
                _marker: PhantomData,
            })
        })
    }

    pub unsafe fn size(&self) -> Result<isize, exception::VmError> {
        let mut string_size: isize = -1;
        let handle = Dart_StringStorageSize(self.raw, &mut string_size);
        check_if_null!(handle, { Ok(string_size) })
    }

    pub unsafe fn from_str(slice: &str) -> Result<Self, exception::VmError> {
        let string = CString::new(slice).unwrap().into_raw();
        let handle = Dart_NewStringFromCString(string);
        check_if_error!(handle, {
            std::mem::forget(handle);
            Ok(Value {
                raw: handle,
                _marker: PhantomData,
            })
        })
    }
}
