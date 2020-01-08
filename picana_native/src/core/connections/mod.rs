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
use std::thread::spawn;
//use std::sync::mpsc::{Sender, Receiver, channel};
use mio::{Events, Interest, Poll, Waker};
use parking_lot::{Mutex, RwLock};
use std::sync::mpsc::Sender;
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
    //waker: Mutex<Waker>,
    transmitter: Mutex<Sender<(i8, Option<(String, CANFrame)>)>>,
    sockets: RwLock<HashMap<String, (Waker, local::MIOCANSocket)>>,
}

impl ConnectionManager {
    pub fn from(transmitter: Mutex<Sender<(i8, Option<(String, CANFrame)>)>>) -> Self {
        let sockets = RwLock::default();
        ConnectionManager {
            //waker,
            transmitter,
            sockets,
        }
    }

    // Clone transmitter and dispatch frames
    pub fn connect(
        &self,
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

                let waker = Waker::new(poll.registry(), mio::Token(99))?;

                let mut events = Events::with_capacity(1024); // 1kb events
                let transmitter = self.transmitter.lock().clone(); /*{
                                                                       Ok(transmitter) => transmitter.clone(),
                                                                       _ => return Err(io::Error::new(io::ErrorKind::NotFound, "No Transmitter!")),
                                                                   };*/

                let mio_socket_dup = mio_socket.clone();
                let siface = String::from(iface);

                spawn(move || {
                    'handler: loop {
                        poll.poll(&mut events, None).unwrap();

                        for event in events.iter() {
                            match event.token() {
                                mio::Token(99) => {
                                    println!("I AM AWOKEN AND EXITING");
                                    drop(transmitter);
                                    break 'handler;
                                }
                                _ => {
                                    //loop {
                                    // A frame should be ready
                                    match mio_socket.read_frame() {
                                        Ok(frame) => {
                                            //TODO: use an enum instead of just i8
                                            match transmitter
                                                .send((0, Some((siface.clone(), frame))))
                                            {
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
                                    //}
                                }
                            }
                        }
                    }
                });
                self.sockets
                    .write()
                    .insert(String::from(iface), (waker, mio_socket_dup));
                //_ => None,
                //};
                Ok(())
            }
            Err(_e) => Err(io::Error::new(io::ErrorKind::NotFound, "E")),
        }
    }

    pub fn killall(&self) -> Result<(), io::Error> {
        //match self.sockets.write() {
        //Ok(mut guard) => {
        // TODO: use map instead of for loop!
        for (_key, (waker, _socket)) in self.sockets.write().drain() {
            match waker.wake() {
                Ok(_) => {}
                _ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
            }
            //_ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
        Ok(())
        //}
        //_ => Err(io::Error::from(io::ErrorKind::Other)),
        //}
    }

    pub fn kill(&self, iface: &str) -> Result<(), io::Error> {
        //match self.sockets.write() {
        //Ok(mut guard) =>
        let mut guard = self.sockets.write();
        match guard.get_mut(iface) {
            Some((waker, _socket)) => match waker.wake() {
                Ok(_) => {
                    guard.remove(iface).unwrap();
                    Ok(())
                }
                _ => {
                    //warn!("[KILL] Not good jim!");
                    Err(io::Error::from(io::ErrorKind::PermissionDenied))
                }
            },
            _ => Err(io::Error::from(io::ErrorKind::Interrupted)),
        }
        //_ => Err(io::Error::from(io::ErrorKind::Interrupted)),
        //}
    }

    // Dispatch a message to an interface!
    pub fn dispatch(&self, destination: &str, message: CANFrame) -> Result<(), io::Error> {
        //let guard = self.sockets.read();
        //match self.sockets.read() {
        //Ok(guard) =>
        match self.sockets.read().get(destination) {
            Some((_handle, socket)) => socket.write_frame_insist(message),
            None => return Err(io::Error::from(io::ErrorKind::AddrNotAvailable)),
        }
        //_ => Err(io::Error::from(io::ErrorKind::AddrNotAvailable)),
        //}
    }
}
