// Connection manager
//
use libc::c_int;
use socketcan::{CANSocket, CANSocketOpenError};
use std::collections::HashMap;

//#[repr(C, packed)]
pub struct HandlerResource {
    //Certain Rust types are defined to never be null. This includes references (&T, &mut T), boxes (Box<T>), and function pointers (extern "abi" fn()).
    //When interfacing with C, pointers that might be null are often used, which would seem to require some messy transmutes and/or unsafe code
    //to handle conversions to/from Rust types. However, the language provides a workaround.
    //
    //The most common type that takes advantage of the nullable pointer optimization is Option<T>,
    //where None corresponds to null. So Option<extern "C" fn(c_int) -> c_int> is a correct way to represent a
    //nullable function pointer using the C ABI (corresponding to the C type int (*)(int)).
    //
    //https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization
    handler: Option<extern "C" fn(c_int) -> c_int>, //Function should follow C convention and be static!
}

struct Connection {
    sock: CANSocket,
    sock_handler: HandlerResource,
}

pub struct ConnectionManager {
    sources: HashMap<String, Connection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            sources: HashMap::new(),
        }
    }

    pub fn connect(
        &mut self,
        iface: &str,
        handler: Option<extern "C" fn(c_int) -> c_int>,
    ) -> Result<(), CANSocketOpenError> {
        let r = match self.sources.get(iface) {
            Some(_connection) => (),
            None => match CANSocket::open(iface) {
                Ok(connection) => {
                    let ret = match handler {
                        //Yaaaa.....its functional time!
                        Some(function) => function(121),
                        _ => -1,
                    };
                    print!("Connection found -- Called handler -> {}!\n", ret);
                    self.sources.insert(
                        String::from(iface),
                        Connection {
                            sock: connection,
                            sock_handler: HandlerResource { handler },
                        },
                    );
                    ()
                }
                _ => {
                    print!("Nah!\n");
                    ()
                }
            },
        };
        print!("Reached Connection! --> {}\n", iface);
        Ok(r)
    }
}
