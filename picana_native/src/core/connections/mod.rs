// Connection manager
//
// the Rust asynchronous model is very different than that of other languages. Most other languages
// use a “completion” based model, usually built using some form of callbacks. In this case, when
// an asynchronous action is started, it is submitted with a function to call once the operation
// completes. When the process receives the I/O notification from the operating system, it finds
// the function associated with it and calls it immediately. This is a push based model because the
// value is pushed into the callback.
//
//The rust asynchronous model is pull based. Instead of a Future being responsible for pushing the
//data into a callback, it relies on something else asking if it is complete or not. In the case of
//Tokio, that something else is the Tokio runtime.
//
//The Runtime is responsible for repeatedly calling poll on a Future until its value is returned.
//There are a few different ways this can happen in practice. For example, the basic_scheduler
//configuration will block the current thread and process all spawned tasks in place. The
//threaded_scheduler configuration uses a work-stealing thread pool and distributes load across
//multiple threads. The threaded_scheduler is the default for applications and the basic_scheduler
//is the default for tests.
//
//It’s important to remember that all async tasks must be spawned on the runtime or no work will be
//performed.

mod local;

//use futures_util::StreamExt;
use libc::c_int;
use socketcan::{CANFrame, CANSocket, CANSocketOpenError};
use std::io;
//use std::sync::mpsc::{Sender, Receiver, channel};
use std::future;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use tokio::runtime;
use tokio::stream::StreamExt;
//use futures::future::{Async, Future, Poll};
//use futures::executor::block_on;

//use tokio::prelude::*;
//use futures::future::lazy;
//use tokio::runtime;

//use futures::future::{self, Future};
//use futures::stream::{SplitSink, SplitStream, Stream};

//#[repr(C, packed)]
#[derive(Debug)]
pub struct HandlerResource {
    //Certain Rust types are defined to never be null. This includes references (&T, &mut T), boxes (Box<T>),
    //and function pointers (extern "abi" fn()).
    //When interfacing with C, pointers that might be null are often used,
    //which would seem to require some messy transmutes and/or unsafe code
    //to handle conversions to/from Rust types. However, the language provides a workaround.
    //
    //The most common type that takes advantage of the nullable pointer optimization is Option<T>,
    //where None corresponds to null. So Option<extern "C" fn(c_int) -> c_int> is a correct way to represent a
    //nullable function pointer using the C ABI (corresponding to the C type int (*)(int)).
    //
    //https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization
    pub handler: Option<extern "C" fn(c_int) -> c_int>, //Function should follow C convention and be static!
}

#[derive(Debug)]
struct Connection {
    //sock: CANSocket,
//sock_handler: ,
//transmitter: Mutex<Sender<CANFrame>>,
}

#[allow(unused)]
pub struct ConnectionManager {
    transmitter: Sender<CANFrame>,
    receiver: Receiver<CANFrame>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        ConnectionManager {
            //runtime: runtime::Runtime::new().unwrap(),
            transmitter: tx,
            receiver: rx,
        }
    }

    pub fn connect(
        &mut self,
        iface: &str,
        handler: Option<extern "C" fn(c_int) -> c_int>,
    ) -> Result<(), io::Error> {
        match CANSocket::open(iface) {
            Ok(socket) => {
                socket.set_nonblocking(true)?;
                socket
                    .set_nonblocking(true)
                    .expect("set socket non-blocking");
                let mio_socket = local::MIOCANSocket::from(socket);

                let poll = mio::Poll::new().expect("creating poll");
                poll.register(
                    &mio_socket,
                    mio::Token(0),
                    mio::Ready::readable(),
                    mio::PollOpt::edge(),
                )
                .unwrap();

                let mut events = mio::Events::with_capacity(1024);

                loop {
                    poll.poll(&mut events, None).unwrap();

                    for event in events.iter() {
                        match event.token() {
                            _ => {
                                loop {
                                    // A frame should be ready
                                    match mio_socket.read_frame() {
                                        Ok(frame) => println!("{:?}", frame),
                                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                            break
                                        }
                                        Err(e) => panic!("err={}", e),
                                    }
                                }
                            }
                        }
                    }
                }
                //Ok(())
            }
            Err(_e) => Err(io::Error::new(io::ErrorKind::NotFound, "E")),
        }
    }
}
