#[macro_use]
extern crate lazy_static;
//extern crate canparse;
//extern crate libc;
//extern crate mio;
//extern crate socketcan;
#[macro_use]
pub mod vm;
pub mod core;

use dart_sys as sys;
// Following is some advice from som rust-lang forum!
//
//Although Rust is a great language for FFI, it is a very unsafe thing to do, leading very easily to UB 3.

//When doing so, I always use the following

//FFI safety belt
//any pointer from the C world becomes Option<NonNull<_>>, (or Option<unsafe extern "C" fn (...) -> ... > for function pointers);

//this tackles C code being able to feed NULLs at will, forcing the Rust code to handle them;
//use ::libc::c_void 3 to represent C’s void. Thus void * becomes Option<NonNull<::libc::c_void>>;

//even though panic = "abort" is a setting that can be added to Cargo.toml compilation profile, I prefer to have such a guard within the exported code;

//structs and enums should be #[repr(C)] (or #[repr(transparent)] for newtypes)

//if receiving an enum from C / FFI, it should be of integer type. If it isn’t, it should be instantly transmuted into an integer and then matched against integer values to get a Rust enum back.

//this includes booleans:
//let rust_bool: bool = mem::transmute<_, i32>(c_bool) != 0;
//do not use static muts, not even for FFI; you should use

//lazy_static! with RwLocks when in doubt,

//thread_local!s with RefCells for single-threaded programs,

//or, if you really wanna go down the unsafe path, a static UnsafeSyncCell<_>:

//Following is how to switch memory allocators
//#[cfg(target_os = "linux")]
//use std::alloc::System;

// You can replace jemalloc with system memory allocator(malloc) if needed
//use std::alloc::System;

//#[global_allocator]
//static ALLOCATOR: System = System;

///Module providing an interface to the library
///its responsible for FFI and marshalling arguments across the ffi layer
///It isn't designed to be thread safe but could come up naturally by rusts model
///
///
///Most functions here are `extern "C"` and discoverable(no_mangle)
pub mod picana {
    // Lazy static
    //Using this macro, it is possible to have statics that require code to be executed at runtime
    //in order to be initialized. This includes anything requiring heap allocations, like vectors
    //or hash maps, as well as anything that requires function calls to be computed.
    //static S_PICANA: super::core::Picana = super::core::Picana::new();
    //use std::borrow::BorrowMut;
    //
    //Arcs make it shareable across threads
    //
    //CString is to &CStr as String is to &str: the former in each pair are owned strings; the
    //latter are borrowed references.
    //
    //CString is intended for working with traditional C-style strings (a sequence of non-nul bytes
    //terminated by a single nul byte); the primary use case for these kinds of strings is
    //interoperating with C-like code.
    //use dart_sys::Dart_Handle;
    use libc::{c_char, c_int, c_uchar, c_uint};
    use std::boxed::Box;
    use std::ffi::{CStr, CString};
    use std::mem;
    use std::string::String;
    //use Arc which guarantees that the value inside lives as long as the last Arc lives.
    //use dart_sys as dffi; -- Research this
    use super::sys::*;
    use super::vm;
    use log::warn;
    use parking_lot::{Mutex, RwLock};
    use std::borrow::BorrowMut;
    use std::sync::Arc;
    use vm::types::Value;

    // DONE: use a RWLock in place of a mutex as a mutex always blocks or refer to
    // many reader locks can be held at once
    // https://users.rust-lang.org/t/storing-c-callbacks-in-rust/27000/6
    // https://doc.rust-lang.org/std/sync/struct.RwLock.html
    // TODO: switch to once cell!
    lazy_static! {
        /// Creates a global static reference lazily
        static ref PICANA: Arc<RwLock<super::core::Picana>> =
            Arc::new(RwLock::new(super::core::Picana::new()));

        //static ref VM: Mutex<vm::Value<RwLock<vm::DartNull>>> = Mutex::new(vm::Value::create_null());
    }

    //register_module!(picana_Init, getDartPrimitive);
    //Any type you expect to pass through an FFI boundary should have repr(C), as C is the
    //lingua-franca of the programming world. This is also necessary to soundly do more elaborate
    //tricks with data layout such as reinterpreting values as a different type.
    //
    //https://doc.rust-lang.org/nomicon/other-reprs.html
    //
    //
    //Worth noting that bool will correspond to i8 because LLVM IR i guess!
    //
    //This resource is useful for construction of CAN frames
    /// A Structure holding parameters for creating a CAN frame usable via FFI
    #[no_mangle]
    #[repr(C)]
    pub struct LiteFrameResource {
        ///ID for the frame
        id: c_uint,
        /// Data as &[u8]
        data: *mut c_uchar,
        /// Is this a remote frame
        remote: bool,
        /// Is this an error frame!
        error: bool, // Is this an error frame,
    }

    // A resource to share across FFI boundaries!
    /// A Bulkier resource carrying information of a CANFrame specifically from a candump!
    #[no_mangle]
    #[repr(C)]
    pub struct FrameResource {
        /// Timestamp with microseconds
        t_usec: u64,
        /// ID of the frame
        id: u32,
        /// Device name eg can0, can1
        device: *const c_char,
        /// Data Section (8 bytes)
        data: *const c_uchar,
        /// Whether it is a remote Frame
        remote: bool,
        /// Whether an Error Code
        error: bool,
        /// Whether the frame is extended
        extended: bool,
        /// Associated error code?
        error_code: u32,
    }

    impl FrameResource {
        /// Creates an invalid frame - useful in cases of failed decoding to avoid complex logiv
        /// handling
        /// ```rust
        /// let invalid_frame = FrameResource::empty()
        /// ```
        pub fn empty() -> Self {
            let data = vec![0, 0, 0, 0, 0, 0, 0, 0].as_mut_ptr();
            mem::forget(data);
            FrameResource {
                t_usec: 0,
                id: 0,
                device: CString::new("").unwrap().into_raw(),
                data: data,
                remote: false,
                extended: false,
                error: false,
                error_code: 1,
            }
        }

        /// Creates a new resourc from a set of required parameter, makes code cleaner
        /// Specifically made to be passed across the ffi boundary
        ///
        /// # Arguments
        ///
        /// * `t_usec` -   Timestamp with microseconds
        /// * `device` -   socketcan interface e.g can0, vcan1
        /// * `canframe` - CANFrame relevant
        ///
        /// ```rust
        /// let frame = FrameResource::from(15666699, "can0", CANFrame{...});
        /// ```
        pub fn from(t_usec: u64, device: &str, canframe: socketcan::CANFrame) -> Self {
            let mut ownedframedata = canframe.data().to_vec();
            let frame = FrameResource {
                t_usec: t_usec,
                id: canframe.id(),
                device: CString::new(device).unwrap().into_raw(),
                remote: canframe.is_rtr(),
                data: ownedframedata.as_mut_ptr(),
                error: canframe.is_error(),
                extended: canframe.is_extended(),
                error_code: canframe.err(),
            };
            //we don't own this after creating
            std::mem::forget(ownedframedata);
            frame
        }
    }

    /// Resource to proxy interperetations from DBC files
    #[no_mangle]
    #[repr(C)]
    pub struct DefinitionResource {
        available: bool,
        pub bridge: Option<super::core::definitions::ValueDefinitionBridge>,
    }

    impl DefinitionResource {
        ///Builds a `DefinitionResource` from the parameters
        pub fn from(bridge: super::core::definitions::ValueDefinitionBridge) -> Self {
            DefinitionResource {
                available: true,
                bridge: Some(bridge),
            }
        }

        /// Creates an invalid resource, check via available trait
        pub fn empty() -> Self {
            DefinitionResource {
                available: false,
                bridge: None,
            }
        }
    }

    ///Calls a methods on the definition resource
    #[no_mangle]
    pub unsafe extern "C" fn invoke(resource: *mut DefinitionResource, data: &[u8]) -> f32 {
        //Perhaps this is being consumed!!
        //So we need to forget it....its like itsb being re-owned on entry!
        //You will segfault if you don't forget the memory
        let mut resbridge: Box<DefinitionResource> = Box::from_raw(resource);
        //let defres: &mut DefinitionResource = resbridge.borrow_mut();

        let value = match resbridge.bridge.borrow_mut() {
            Some(access) => match access.interpret(data) {
                Some(value) => value,
                _ => 0.0,
            },
            _ => 0.0,
        };

        std::mem::forget(resbridge);
        value
    }

    /// Opens a file using Mmap style
    /// To be used specifically read candumps!
    ///
    /// #Arguments
    ///
    /// * `absolute_path`: Path to the file
    /// * `alias`: a key to identify the file, should be unique!
    #[no_mangle]
    // TODO: Use size_t as defined in libc here instead of usize!
    pub unsafe extern "C" fn openfile(absolute_path: *const c_char, alias: *const c_char) -> i32 {
        let picana = Arc::clone(&PICANA);

        // Convert to rust usable strings
        let abs_path_cstr = CStr::from_ptr(absolute_path);
        let alias_cstr = CStr::from_ptr(alias);

        let abs_path = match abs_path_cstr.to_str() {
            Ok(string) => string,
            Err(_) => return -1,
        };

        let alias_key = match alias_cstr.to_str() {
            Ok(string) => string,
            Err(_) => return -1,
        };

        let mut linecount = 0;

        // Critical Section
        match picana.write().open(alias_key, abs_path) {
            //Ok(mut guard) => {
            //match guard.open(alias_key, abs_path) {
            Ok(lines) => {
                linecount = lines;
                //guard.load_dbc(alias_key, "zeva_30.dbc");
            }
            Err(e) => {
                warn!("OPENFILE: Fatal! => {}\n", e);
            }
        };
        //}
        //Err(e) => {
        //warn!("OPENFILE {}\n", e);
        //}
        //}
        linecount as i32
    }

    /// Opens a dbc file containing instructions on decoding CAN frames
    ///
    /// #Arguments
    ///
    /// * `absolute_path`: path to the file
    /// * `absolute_path`: unique key to identify the file
    #[no_mangle]
    pub unsafe extern "C" fn opendbc(absolute_path: *const c_char, alias: *const c_char) -> i32 {
        let picana = Arc::clone(&PICANA);
        // Convert to rust usable strings
        let abs_path_cstr = CStr::from_ptr(absolute_path);
        let alias_cstr = CStr::from_ptr(alias);

        let abs_path = match abs_path_cstr.to_str() {
            Ok(string) => string,
            Err(_) => return -1,
        };

        let alias_key = match alias_cstr.to_str() {
            Ok(string) => string,
            Err(_) => return -1,
        };
        match picana.write().load_dbc(alias_key, abs_path) {
            //Ok(mut guard) => {
            //match guard.load_dbc(alias_key, abs_path) {
            Ok(_) => 0,
            Err(e) => {
                warn!("OPENFILE: Fatal! => {}\n", e);
                return -1;
            }
        };
        0
    }

    /// Gets a single line from a mmaped file, requires the file be opened
    /// using `openfile` and registered with key `alias`
    #[no_mangle]
    pub unsafe extern "C" fn line(alias: *const c_char, index: i32) -> *mut c_char {
        let picana = Arc::clone(&PICANA);
        let mut aline: String = String::new();

        let alias_cstr = CStr::from_ptr(alias);

        let alias_fin = match alias_cstr.to_str() {
            Ok(string) => string,
            Err(e) => {
                warn!("LINE: What> => {}\n", e);
                ""
            }
        };

        match picana.read().line(alias_fin, index as usize) {
            //Ok(guard) => match guard.line(alias_fin, index as usize) {
            Ok(line) => {
                aline += line;
            }
            Err(e) => {
                warn!("LINE! => {}\n", e);
            } //},
              //Err(e) => {
              //warn!("LINE! => {}\n", e);
              //}
        }
        CString::new(aline).unwrap().into_raw()
    }

    //Raw pointers are useful for FFI: Rust’s *const T and *mut T are similar to C’s const T* and T*, respectively.
    //For more about this use, consult the FFI chapter(nomicon).
    /// Gets information from a CANFrame as a `FrameResource`, line number needed as index
    #[no_mangle]
    pub unsafe extern "C" fn canframedata(
        alias: *const c_char,
        index: i32,
    ) -> *const FrameResource {
        let picana = Arc::clone(&PICANA);

        let alias_cstr = CStr::from_ptr(alias);

        let alias_fin = match alias_cstr.to_str() {
            Ok(string) => string,
            Err(e) => {
                warn!("CANFrameDATA> => {}\n", e);
                return Box::into_raw(Box::new(FrameResource::empty()));
            }
        };

        let exitframe = match picana.read().frame(alias_fin, index as usize) {
            Ok(Some((t_usec, iface, canframe))) => {
                let mut ownedframedata = canframe.data().to_vec();
                let frame = FrameResource {
                    t_usec: t_usec,
                    id: canframe.id(),
                    device: CString::new(iface).unwrap().into_raw(),
                    remote: canframe.is_rtr(),
                    data: ownedframedata.as_mut_ptr(),
                    error: canframe.is_error(),
                    extended: canframe.is_extended(),
                    error_code: canframe.err(),
                };
                // We no longer own this memory -> so lets not dealloc it!
                // If youre seeing `return var owned by local fn` means that you're attempting to give
                // away a value owned here so figure it out and forget it to pass to the ffi!
                mem::forget(ownedframedata);
                frame
            }
            Ok(None) => {
                warn!("CANFrameDATA: No Frame found!!\n");
                FrameResource::empty()
            }
            Err(e) => {
                warn!("CANFrameData: when getting frame! => {:?}\n", e);
                FrameResource::empty()
            }
        };

        // &frame as *const frame
        //  ^ ^
        //  | |
        // This is so wrong. You're creating a value on the stack (because you didn't wrap it in
        // any sort of heap allocation and Rust is all about those value type semantics), and then
        // you return a pointer to that value. Of course when the function returns that stack
        // memory is now free to be reused by other functions, so even though you called forget to
        // skip the destructor, the memory used by new_twin is still free to be overwritten by
        // later functions. What you really want to do is use Box which has methods specifically
        // for this purpose. (Returning objects from FFI)
        //
        //
        // https://www.reddit.com/r/rust/comments/6m48tx/reprc_structs_and_ffi/
        Box::into_raw(Box::new(exitframe))
    }

    /// Creates a `DefinitionResource` from a SPN if found in the DBC file loaded with `parameter`
    #[no_mangle]
    pub unsafe extern "C" fn explainer(
        alias: *const c_char,
        parameter: *const c_char,
    ) -> *const DefinitionResource {
        let picana = Arc::clone(&PICANA);
        let alias_cstr = CStr::from_ptr(alias);
        let params_cstr = CStr::from_ptr(parameter);

        let alias_fin = match alias_cstr.to_str() {
            Ok(string) => string,
            Err(e) => {
                warn!("EXPLAINER: What> => {}\n", e);
                return Box::into_raw(Box::new(DefinitionResource::empty()));
            }
        };

        let parameter_fin = match params_cstr.to_str() {
            Ok(string) => string,
            Err(e) => {
                warn!("EXPLAINER: What> => {}", e);
                return Box::into_raw(Box::new(DefinitionResource::empty()));
            }
        };

        let defined = match picana.read().explain(alias_fin, parameter_fin) {
            Ok(bridge) => DefinitionResource::from(bridge),
            _ => DefinitionResource::empty(),
        };

        //Rust's owned boxes (Box<T>) use non-nullable pointers as handles which point to the contained object.
        //However, they should not be manually created because they are managed by internal allocators.
        //References can safely be assumed to be non-nullable pointers directly to the type.
        //However, breaking the borrow checking or mutability rules is not guaranteed to be safe,
        //so prefer using raw pointers (*) if that's needed because the compiler can't make as many assumptions about them.
        //       -----------------|
        //      |
        //      V
        Box::into_raw(Box::new(defined))
    }

    /// Connects to an interface on the local machine!
    ///
    /// * iface:    Interface
    /// * use_port: A valid dart port!
    #[no_mangle]
    pub unsafe extern "C" fn connect(iface: *const c_char, use_port: Dart_Port) -> i32 {
        let picana = Arc::clone(&PICANA);
        let alias_fin = match CStr::from_ptr(iface).to_str() {
            Ok(string) => string,
            Err(e) => {
                warn!("CONNECT: What> => {}\n", e);
                return -1;
            }
        };
        let port = if use_port > 0 { Some(use_port) } else { None };
        let r = match picana.read().connect(alias_fin, port) {
            //Ok(guard) => match guard.connect(alias_fin) {
            Ok(_) => 0,
            _ => -3,
            //},
            //_ => -9,
        };
        r
    }

    #[no_mangle]
    /// Polls any socket opened for frames
    /// NB: Calling handler from a separate thread can crash other process!!
    /// NB: This function may be deprecated in preference to using Dart ports!
    /// only use for synchronous communication
    pub unsafe extern "C" fn listen(handler: extern "C" fn(*const FrameResource) -> c_int) -> i32 {
        let picana = Arc::clone(&PICANA);
        let r = picana.read().listen(Some(handler));
        r
    }

    #[no_mangle]
    /// Writes a frame to the Socket!
    pub unsafe extern "C" fn say(to: *const c_char, that: *const LiteFrameResource) -> i32 {
        let picana = Arc::clone(&PICANA);
        let iface = match CStr::from_ptr(to).to_str() {
            Ok(string) => string,
            Err(e) => {
                warn!("SAY: What> => {}\n", e);
                return -1;
            }
        };

        let can_frame = {
            //data
            let dvec = std::vec::Vec::from_raw_parts((*that).data, 8, 8);
            let id = (*that).id;
            let remote = (*that).remote;
            let error = (*that).error;
            match socketcan::CANFrame::new(id, &dvec, remote, error) {
                Ok(frame) => frame,
                _ => return -4,
            }
        };

        let r = match picana.read().tell(iface, can_frame) {
            //Ok(guard) => match guard.tell(iface, can_frame) {
            Ok(_) => 0,
            _ => -2,
            //},
            //_ => -1,
        };
        r
    }

    /// Closes an interface e.g vcan0 and stops polling it!
    #[no_mangle]
    pub unsafe extern "C" fn terminate(to: *const c_char) -> i32 {
        let picana = Arc::clone(&PICANA);
        let iface = match CStr::from_ptr(to).to_str() {
            Ok(string) => string,
            Err(e) => {
                warn!("SAY: What> => {}\n", e);
                return -1;
            }
        };
        let r = match picana.read().close(iface) {
            Ok(_) => 0,
            _ => -1,
        };
        r
    }

    /// Closes all interfaces!
    #[no_mangle]
    pub unsafe extern "C" fn silence() -> i32 {
        let picana = Arc::clone(&PICANA);
        let r = picana.read().finish(); // {
                                        //Ok(guard) => guard.finish(),
                                        //_ => return -1,
                                        //};
        r
    }

    ///NB: This is here only for testing and may be removed later!
    #[no_mangle]
    pub unsafe extern "C" fn primitive(port_id: i64) -> i64 {
        println!("Port is => {}\n", port_id);

        //TODO: Handle vs CObjects?
        // Doesn't require scopes
        send!(port_id, dart_c_double!(-12213321.2331, f64));
        in_dart_scope! {
            {
                // Now to figure out posting CObjects!
                //let sendport = Value::create_send_port(port_id);
                //Like this and post it
                let mut obj = Dart_CObject{
                    type_: Dart_CObject_Type::Dart_CObject_kBool,
                    value: _Dart_CObject__bindgen_ty_1 { as_bool: true }
                };

                //let mut some_integer = dart_c_int!(300, i32);

                Dart_PostCObject(port_id, &mut obj);
                //Requires scopes!
                //let list = Dart_NewList(3);
                //Dart_ListSetAt(list, 0, Dart_NewInteger(-1));
                //Dart_ListSetAt(list, 1,Dart_NewInteger(0));
                //Dart_ListSetAt(list, 2,Dart_NewInteger(-11));
                //sendport.call("send", 1, list);
                //sendport.call("send", 1, rangehandle);
                //sendport.call("send", 1, Dart_ListGetAt(list, 2));
                send!(port_id, dart_c_double!(-12213321.2331, f64));
                send!(port_id, dart_c_int!(-122133212331, i64));
                //send!(port_id, some_integer);
                //testlist.dispose();
                //sendport.dispose();
            }
        };
        port_id
    }
}
