use dart_sys::{
    Dart_EnterScope, Dart_ExitScope, Dart_Handle, Dart_Invoke, Dart_NativeArguments,
    Dart_NewSendPort, Dart_NewStringFromCString, Dart_Null, Dart_Port,
};
use log;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem;

#[derive(Clone, Copy, Debug)]
pub struct Args(Dart_NativeArguments);

impl Args {
    pub fn from_raw(d: Dart_NativeArguments) -> Self {
        Args(d)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DartNull;

#[derive(Clone, Copy, Debug)]
pub struct DartSendPort;

/* Phantom Data ->
 * Zero-sized type used to mark things that "act like" they own a T.
 * Adding a PhantomData<T> field to your type tells the compiler that your type acts as though it stores a value of type T,
 * even though it doesn't really. This information is used when computing certain safety properties.
 * For a more in-depth explanation of how to use PhantomData<T>, please see the Nomicon.
 */
#[derive(Clone, Copy, Debug)]
pub struct Value<T> {
    raw: Dart_Handle,
    _marker: PhantomData<T>,
}

impl<T> Value<T> {
    pub fn to_handle(self) -> Dart_Handle {
        self.raw
    }

    pub fn dispose(self) {
        mem::drop(self.raw)
    }
}

impl Value<DartNull> {
    pub fn create_null() -> Value<DartNull> {
        let raw = unsafe { Dart_Null() };
        mem::forget(raw);
        Value {
            raw,
            _marker: PhantomData,
        }
    }
}

impl Value<DartSendPort> {
    pub fn new(port_id: Dart_Port) -> Self {
        let raw = unsafe { Dart_NewSendPort(port_id) };
        mem::forget(raw);
        Value {
            raw,
            _marker: PhantomData,
        }
    }

    //TODO: Args using Dart_NativeArguments!
    pub fn call(&self, func: &str, _args: Vec<u8>) -> Result<bool, bool> {
        //TODO: Check return values!
        unsafe {
            Dart_EnterScope();
            let string = CString::new(func).unwrap().into_raw();
            let dartstr = Dart_NewStringFromCString(string);
            let res = Dart_Invoke(self.raw, dartstr, 1, &mut Value::create_null().to_handle());
            Dart_ExitScope();
        }
        Ok(true)
    }
}
