#[macro_use]
extern crate lazy_static;
pub mod core;

//#[cfg(target_os = "linux")]
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
    use std::boxed::Box;
    use std::ffi::{CStr, CString};
    use std::mem;
    use std::os::raw::{c_char, c_uchar};
    use std::string::String;
    //use Arc which guarantees that the value inside lives as long as the last Arc lives.
    use std::sync::{Arc, Mutex};

    lazy_static! {
        static ref PICANA: Arc<Mutex<super::core::Picana>> =
            Arc::new(Mutex::new(super::core::Picana::new()));
    }

    //Any type you expect to pass through an FFI boundary should have repr(C), as C is the
    //lingua-franca of the programming world. This is also necessary to soundly do more elaborate
    //tricks with data layout such as reinterpreting values as a different type.
    //
    //https://doc.rust-lang.org/nomicon/other-reprs.html
    //
    //
    //Worth noting that bool will correspond to i8 because LLVM IR i guess!
    #[no_mangle]
    #[repr(C)]
    pub struct FrameResource {
        // A resource to share across FFI boundaries!
        t_usec: u64,           // Timestamp with microseconds
        id: u32,               // ID of the frame
        device: *const c_char, // Device name
        data: *const c_uchar,  // Data Section
        remote: bool,          // Whether it is a remote Frame
        error: u32,            // Error Code
        extended: bool,        // Whether the frame is extended
    }

    impl FrameResource {
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
                error: 0,
            }
        }
    }

    #[no_mangle]
    // TODO: Use size_t as defined in libc here instead of usize!
    pub unsafe extern "C" fn openfile(absolute_path: *const c_char, alias: *const c_char) -> i32 {
        let picana = Arc::clone(&PICANA);

        // Convert to rust usable strings
        let abs_path_cstr = CStr::from_ptr(absolute_path);
        let alias_cstr = CStr::from_ptr(alias);

        let abs_path = match abs_path_cstr.to_str() {
            Ok(string) => string,
            Err(_) => "",
        };

        let alias_key = match alias_cstr.to_str() {
            Ok(string) => string,
            Err(_) => "",
        };

        let mut linecount = 5;

        // Critical Section
        match picana.lock() {
            Ok(mut guard) => {
                match guard.open(alias_key, abs_path) {
                    Ok(lines) => {
                        linecount = lines;
                    }
                    Err(e) => {
                        print!("\nFatal! => {}\n", e);
                    }
                };
            }
            Err(e) => {
                print!("\nFailed!! {}\n", e);
            }
        }
        linecount as i32
    }

    #[no_mangle]
    pub unsafe extern "C" fn line(alias: *const c_char, index: i32) -> *mut c_char {
        let picana = Arc::clone(&PICANA);
        let mut aline: String = String::new();

        let alias_cstr = CStr::from_ptr(alias);

        let alias_fin = match alias_cstr.to_str() {
            Ok(string) => string,
            Err(e) => {
                print!("\nWhat> => {}\n", e);
                ""
            }
        };

        match picana.lock() {
            Ok(guard) => match guard.line(alias_fin, index as usize) {
                Ok(line) => {
                    aline += line;
                }
                Err(e) => {
                    print!("\nBruh! => {}\n", e);
                }
            },
            Err(e) => {
                print!("\nFatal!\n => {}\n", e);
            }
        }
        CString::new(aline).unwrap().into_raw()
    }

    //Raw pointers are useful for FFI: Rust’s *const T and *mut T are similar to C’s const T* and T*, respectively. For more about this use, consult the FFI chapter.
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
                print!("\nWhat> => {}\n", e);
                return Box::into_raw(Box::new(FrameResource::empty()));
            }
        };

        let exitframe = match picana.lock() {
            Ok(guard) => match guard.frame(alias_fin, index as usize) {
                Ok(Some((t_usec, iface, (id, mut data, remote, error, extended)))) => {
                    data.shrink_to_fit();
                    let frame = FrameResource {
                        t_usec: t_usec,
                        id: id,
                        device: CString::new(iface).unwrap().into_raw(),
                        remote: remote,
                        data: data.as_mut_ptr(),
                        error: error,
                        extended: extended,
                    };
                    //print!("Got id {}, ts {},\n", frame.id, frame.t_usec);
                    //print!(
                    //"error|remote|extended {} | {} | {}\n",
                    //frame.error, frame.remote, frame.extended
                    //);
                    //print!("device: {}\n", iface);
                    //print!("Vector {:?}\t Capacity: ({})\n", data, data.capacity());
                    // We no longer own this memory -> so lets not dealloc it!
                    mem::forget(data);
                    frame
                }
                Ok(None) => {
                    print!("Bruh\n\n!");
                    FrameResource::empty()
                }
                Err(e) => {
                    print!("\nBruh! => {:?}\n", e);
                    FrameResource::empty()
                }
            },
            Err(e) => {
                print!("\nFatal!\n => {}\n", e);
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
        // for this purpose.
        //
        //
        // https://www.reddit.com/r/rust/comments/6m48tx/reprc_structs_and_ffi/
        Box::into_raw(Box::new(exitframe))
    }
}
