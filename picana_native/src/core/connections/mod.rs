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
use hashbrown::HashMap;
use socketcan::{CANFrame, CANSocket};
use std::io;
use std::thread::{spawn, JoinHandle};
//use std::sync::mpsc::{Sender, Receiver, channel};
use mio::{Events, Interest, Poll};
use std::sync::{mpsc::Sender, Mutex};
//use tokio::runtime;
//use tokio::stream::StreamExt;
//use futures::future::{Async, Future, Poll};
//use futures::executor::block_on;

//use tokio::prelude::*;
//use futures::future::lazy;
//use tokio::runtime;

//use futures::future::{self, Future};
//use futures::stream::{SplitSink, SplitStream, Stream};

#[allow(unused)]
pub struct ConnectionManager {
    //poll: mio::Poll, //Polling events from sockets
    transmitter: Mutex<Sender<(String, CANFrame)>>,
    sockets: HashMap<String, (JoinHandle<()>, local::MIOCANSocket)>,
}

impl ConnectionManager {
    pub fn from(transmitter: Mutex<Sender<(String, CANFrame)>>) -> Self {
        let sockets = HashMap::new();
        ConnectionManager {
            transmitter,
            sockets,
        }
    }

    // Clone transmitter and dispatch frames
    pub fn connect(
        &mut self,
        iface: &str,
        //handler: Option<extern "C" fn(c_int) -> c_int>,
    ) -> Result<(), io::Error> {
        match CANSocket::open(iface) {
            Ok(socket) => {
                socket.set_nonblocking(true)?;
                let mut mio_socket = local::MIOCANSocket::from(socket);
                let mut poll = Poll::new()?;

                poll.registry().register(
                    &mut mio_socket,
                    mio::Token(0),      // This would have to be dynamic!
                    Interest::READABLE, // I still don't understand this! :(
                )?;
                let mut events = Events::with_capacity(1024); // 1kb events
                let transmitter = match self.transmitter.lock() {
                    Ok(transmitter) => transmitter.clone(),
                    _ => return Err(io::Error::new(io::ErrorKind::NotFound, "No Transmitter!")),
                };

                let mio_socket_dup = mio_socket.clone();
                let siface = String::from(iface);

                let handle = spawn(move || {
                    loop {
                        poll.poll(&mut events, None).unwrap();

                        for event in events.iter() {
                            match event.token() {
                                _ => {
                                    loop {
                                        // A frame should be ready
                                        match mio_socket.read_frame() {
                                            Ok(frame) => {
                                                match transmitter.send((siface.clone(), frame)) {
                                                    // Receiving end is alive!
                                                    Ok(_res) => (),
                                                    // Receiving end is not alive// Data is
                                                    // returned as res!
                                                    Err(res) => println!("Err -> {:?}\n", res),
                                                };
                                            }
                                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                                break
                                            }
                                            Err(_e) => break,
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
                self.sockets
                    .insert(String::from(iface), (handle, mio_socket_dup));
                Ok(())
            }
            Err(_e) => Err(io::Error::new(io::ErrorKind::NotFound, "E")),
        }
    }

    // Dispatch a message to an interface!
    pub fn dispatch(&self, destination: &str, message: CANFrame) -> Result<(), io::Error> {
        match self.sockets.get(destination) {
            Some((_handle, socket)) => socket.write_frame_insist(message),
            None => Err(io::Error::from(io::ErrorKind::AddrNotAvailable)),
        }
    }
}
