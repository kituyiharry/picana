use dart_sys::*;
use libc::c_int;
use log;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem;

pub mod dart_bool;
pub mod dart_c_objects;
pub mod dart_double;
pub mod dart_integer;
pub mod dart_list;

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

//Maybe this is too much!
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
    pub fn create_send_port(port_id: Dart_Port) -> Self {
        let raw = unsafe { Dart_NewSendPort(port_id) };
        mem::forget(raw);
        Value {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn call(&self, func: &str, num_args: c_int, mut args: Dart_Handle) -> Result<bool, bool> {
        //TODO: Check return values!
        unsafe {
            let string = CString::new(func).unwrap().into_raw();
            let dartstr = Dart_NewStringFromCString(string);
            let mut mnull = Value::create_null().to_handle();
            let mut marrayargs = [&mut args];
            let mut mptr = marrayargs[0] as *mut Dart_Handle;
            //NB: Look more into using std::ptr!
            let res = Dart_Invoke(self.raw, dartstr, num_args, mptr);
        }
        Ok(true)
    }
}
