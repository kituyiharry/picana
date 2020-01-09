#[macro_use]
use super::vm;
use std::marker::PhantomData;
use super::vm::instance::exception;
use types::sys::{Dart_ListLength, Dart_ListGetAt, Dart_ListSetAt, Dart_NewList};
use types::Value;

#[derive(Clone, Copy, Debug)]
pub struct DartList;

//Example of passing arguments to Invoke
///Creates a list of length size to hold elements
impl Value<DartList> {
    //New List
    pub unsafe fn list(size: isize) -> Resut<Self, exception::VmError> {
        let raw = Dart_NewList(size);
        std::mem::forget(handle);
        check_if_error! handle, {
            std::mem::forget(handle);
            Ok(Value {
                raw,
                _marker: PhantomData,
            })
        }
    }

    //Length of list
    pub unsafe fn length(&self) -> Result<isize, exception::VmError> {
        let mut listsize = -1;
        let handle = Dart_ListLength(self.raw, &mut listsize);
        check_if_null! handle, { 
            std::mem::forget(handle);
            Ok(listsize) 
        }
    }

    //Insert elementa at index;
    pub unsafe fn insert(&self, element: Dart_Handle, at: isize) -> Result<(), exception::VmError> {
        let handle = Dart_ListSetAt(self.raw, at, element);
        check_if_null! handle, { Ok(()) }
    }

    pub unsafe fn get(&self, index: isize) -> Result<Dart_Handle, exception::VmError> {
        let handle = Dart_ListGetAt(self.raw, index);
        check_if_null! handle, { Ok(handle) }
    }
}
