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

//Note that these types aren't guaranteed to perform similarly when using flutter
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

//If you’re invoking unsafe blocks through an interface not marked as unsafe, it is the callee’s, not the caller’s, responsibility
//to make sure that every possible call is safe, since the whole point of unsafe is “static analysis can’t prove this is safe”,
//so I doubt static analysis can meaningfully capture this sort of unsafe hygine, nor do I think it is the compiler’s responsibility to do so.
// ________|
// |
//Why to not use unsafe block in macros! (https://internals.rust-lang.org/t/explicitly-marking-unsafe-macro-expressions/9425/3)
#[inline(always)]
macro_rules! send {
    ($x:expr, $y:expr) => {
        $crate::sys::Dart_PostCObject($x, &mut $y);
    };
}

#[inline(always)]
macro_rules! as_mut_object {
    ($x:expr) => {
        &mut $x as *mut $crate::sys::Dart_CObject
    };
}

macro_rules! dart_c_bool {
    ($x:expr) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kBool,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 { as_bool: $x },
        };
    };
}

macro_rules! dart_c_int {
    ($x:expr, i32) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kInt32,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 { as_int32: $x },
        };
    };

    ($x:expr, i64) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kInt64,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 { as_int64: $x },
        };
    };
}

macro_rules! dart_c_double {
    ($x:expr) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kDouble,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 { as_double: $x },
        };
    };
}

macro_rules! dart_c_string {
    ($x:expr) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kString,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_string: $x as *mut i8,
            },
        };
    };
}

//Todo pass pointer to values in array
macro_rules! dart_c_array {
    ($x:expr) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kArray,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_array: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_3 {
                    length: $x.len() as isize,
                    values: $x.as_mut_ptr(),
                },
            },
        };
    };
}

macro_rules! dart_c_typed_data {
    ($x:expr, u8) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kUint8,
                    length: $x.len() as isize, // isize
                    values: $x.as_mut_ptr(),   //*mut u8
                },
            },
        };
    };
    ($x:expr, i8) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kInt8, // Dart_Typed_data_type
                    length: $x.len() as isize,                        // isize
                    values: $x.as_mut_ptr(),                          //*mut i8
                },
            },
        };
    };
    ($x:expr, i16) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kInt16, // Dart_Typed_data_type
                    length: $x.len() as isize,                         // isize
                    values: $x.as_mut_ptr(),                           //*mut i8
                },
            },
        };
    };
    ($x:expr, u16) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kUint16, // Dart_Typed_data_type
                    length: $x.len() as isize,                          // isize
                    values: $x.as_mut_ptr(),                            //*mut i8
                },
            },
        };
    };
    ($x:expr, u32) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kUint32, // Dart_Typed_data_type
                    length: $x.len() as isize,                          // isize
                    values: $x.as_mut_ptr(),                            //*mut i8
                },
            },
        };
    };
    ($x:expr, i32) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kInt32, // Dart_Typed_data_type
                    length: $x.len() as isize,                         // isize
                    values: $x.as_mut_ptr(),                           //*mut i8
                },
            },
        };
    };
    ($x:expr, i64) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kInt64, // Dart_Typed_data_type
                    length: $x.len() as isize,                         // isize
                    values: $x.as_mut_ptr(),                           //*mut i8
                },
            },
        };
    };
    ($x:expr, u64) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kUint64, // Dart_Typed_data_type
                    length: $x.len() as isize,                          // isize
                    values: $x.as_mut_ptr(),                            //*mut i8
                },
            },
        };
    };
    ($x:expr, f32) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kFloat64, // Dart_Typed_data_type
                    length: $x.len() as isize,                           // isize
                    values: $x.as_mut_ptr(),                             //*mut i8
                },
            },
        };
    };
    ($x:expr, f64) => {
        $crate::sys::Dart_CObject {
            type_: $crate::sys::Dart_CObject_Type::Dart_CObject_kTypedData,
            value: $crate::sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: $crate::sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: $crate::sys::Dart_TypedData_Type::Dart_TypedData_kFloat64, // Dart_Typed_data_type
                    length: $x.len() as isize,                           // isize
                    values: $x.as_mut_ptr(),                             //*mut i8
                },
            },
        };
    };
}
//Define here to be able to use macros. Refer to
//https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub mod instance;
pub mod types;
