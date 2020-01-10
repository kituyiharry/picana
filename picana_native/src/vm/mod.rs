//Old style macros used for registering when using darts native extensions
//Adapted from https://github.com/Brooooooklyn/dart-rs
macro_rules! register_module {
  ($module_name:ident, $( $x:ident ),*) => {
    use $crate::sys::*;

    #[no_mangle]
    pub unsafe extern "C" fn $module_name(parent_library: Dart_Handle) -> Dart_Handle {
      if Dart_IsError(parent_library) {
        return parent_library;
      }
      let result_code = Dart_SetNativeResolver(parent_library, Some(__dart__bindings__resolve__name__), None);
      if Dart_IsError(result_code) {
        return result_code;
      };
      return Dart_Null();
    }

    extern "C" fn __dart__bindings__resolve__name__(h: $crate::sys::Dart_Handle, _argc: i32, _auto_setup_scope: *mut bool) -> $crate::sys::Dart_NativeFunction {
      use $crate::sys::*;
      use std::ptr;
      use std::ffi::CString;
      use std::mem;

      println!("Library Resolve dartnames");
      unsafe {
        if !Dart_IsString(h) {
          return None;
        }
        let mut chars = ptr::null();
        let result = Dart_StringToCString(h, &mut chars);
        if Dart_IsError(result) {
          Dart_PropagateError(result);
          return None;
        }
        let name = CString::from_raw(chars as *mut i8).into_string().expect("Get name string fail");

        match name.as_str() {
          $(
            stringify!($x) => {
              unsafe extern "C" fn __bind__to__dart__ (args: Dart_NativeArguments) {
                let call_result = $x(Args::from_raw(args)).to_handle();
                Dart_SetReturnValue(args, call_result);
              }
              mem::forget(name);
              Some(__bind__to__dart__)
            },
            _ => {
              println!("{} not match", name);
              mem::forget(name);
              None
            },
          )*
        }
      }
    }
  };
}

///These are helpers using dart types to fullfill DRY
macro_rules! in_dart_scope {
    ($x:block) => {{
        use $crate::sys::{Dart_EnterScope, Dart_ExitScope};
        Dart_EnterScope();
        $x;
        Dart_ExitScope();
    }};
}

macro_rules! in_dart_isolate {
    ($x:block) => {{
        use $crate::sys::{Dart_EnterScope, Dart_ExitScope};
        Dart_EnterIsolate()
        $x;
        Dart_ExitIsolate();
    }};
}

macro_rules! check_if_null {
    ($x:ident, $y:block) => {{
        use $crate::vm::instance::exception;
        use $crate::sys::Dart_IsNull;
        if Dart_IsNull($x) {
            Err(exception::VmError{
                error: exception::VmErrorType::VmNullPointer,
                handle: $x
            })
        } else $y
    }}
}

macro_rules! check_if_error {
    ($x:ident, $y:block) => {{
        use $crate::vm::instance::exception;
        use $crate::sys::Dart_IsNull;

        if Dart_IsNull($x) {
            Err(exception::VmError{
                error: exception::VmErrorType::VmNullPointer,
                handle: $x
            })
        } else $y
    }}
}

macro_rules! send {
    ($x:expr, $y:expr) => {
        Dart_PostCObject($x, &mut $y);
    };
}

macro_rules! dart_c_bool {
    ($x:expr, bool) => {
        Dart_CObject {
            type_: Dart_CObject_Type::Dart_CObject_kBool,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x },
        };
    };
}

macro_rules! dart_c_int {
    ($x:expr, i32) => {
        Dart_CObject {
            type_: Dart_CObject_Type::Dart_CObject_kInt32,
            value: _Dart_CObject__bindgen_ty_1 { as_int32: $x },
        };
    };

    ($x:expr, i64) => {
        Dart_CObject {
            type_: Dart_CObject_Type::Dart_CObject_kInt64,
            value: _Dart_CObject__bindgen_ty_1 { as_int64: $x },
        };
    };
}

macro_rules! dart_c_double {
    ($x:expr,  f64) => {
        Dart_CObject {
            type_: Dart_CObject_Type::Dart_CObject_kDouble,
            value: _Dart_CObject__bindgen_ty_1 { as_double: $x },
        };
    };
}

macro_rules! dart_c_string {
    ($x:ident) => {
        Dart_CObject {
            type_: Dart_CObject_Type::Dart_CObject_kString,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x },
        };
    };
}

macro_rules! dart_c_array {
    ($x:ident) => {
        Dart_CObject {
            type_: Dart_CObject_Type::Dart_CObject_kArray,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x },
        };
    };
}

macro_rules! dart_c_typed_data {
    ($x:ident) => {{
        Dart_CObject {
            type_: Dart_CObject_Type::Dart_CObject_kTypedData,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x },
        };
    }};
}
//Define here to be able to use macros. Refer to
//https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub mod instance;
pub mod types;
