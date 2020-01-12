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
use crate::sys::{
    Dart_CObject, Dart_CObject_Type, Dart_Port, Dart_PostCObject, Dart_TypedData_Type,
    _Dart_CObject__bindgen_ty_1, _Dart_CObject__bindgen_ty_1__bindgen_ty_3,
    _Dart_CObject__bindgen_ty_1__bindgen_ty_4,
};
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
//

const WAKER_TOKEN: mio::Token = mio::Token(std::usize::MAX);
const POLL_TOKEN: mio::Token = mio::Token(std::usize::MIN);
const PAUSE_TOKEN: mio::Token = mio::Token(99);

type Remote = (Waker, Waker); // Kill and Pause tokens

#[allow(unused)]
pub struct ConnectionManager {
    transmitter: Mutex<Sender<(i8, Option<(String, CANFrame)>)>>,
    sockets: RwLock<HashMap<String, (Remote, local::MIOCANSocket)>>,
}

impl ConnectionManager {
    pub fn from(transmitter: Mutex<Sender<(i8, Option<(String, CANFrame)>)>>) -> Self {
        let sockets = RwLock::default();
        ConnectionManager {
            transmitter,
            sockets,
        }
    }

    pub fn connect_async(
        &self,
        iface: &str,
        port: i64, //handler: Option<extern "C" fn(c_int) -> c_int>,
    ) -> Result<(), io::Error> {
        match CANSocket::open(iface) {
            Ok(socket) => {
                socket.set_nonblocking(true)?;
                let mut mio_socket = local::MIOCANSocket::from(socket);
                let mut poll = Poll::new()?;

                poll.registry().register(
                    &mut mio_socket,
                    POLL_TOKEN,         // This would have to be dynamic!
                    Interest::READABLE, // I still don't understand this! :(
                )?;

                let waker = Waker::new(poll.registry(), WAKER_TOKEN)?;
                let remote = Waker::new(poll.registry(), PAUSE_TOKEN)?;

                let mut events = Events::with_capacity(1024); // 1kb events

                //Having internal state in the socket can cause wrong logical information e.g
                //who's state is correct, the cloned socket or the previous?
                //Especially since we are using a dup to clone without considering other state!
                //TODO: Fix this!!
                let mio_dup = mio_socket.clone();
                let mut siface = String::from(iface);

                spawn(move || {
                    'handler: loop {
                        poll.poll(&mut events, None).unwrap();

                        for event in events.iter() {
                            match event.token() {
                                WAKER_TOKEN => {
                                    println!("I AM AWOKEN AND EXITING");
                                    break 'handler;
                                }
                                PAUSE_TOKEN => {
                                    if mio_socket.ispaused() {
                                        match mio_socket.unpause() {
                                            Ok(_) => println!("Un-Paused"),
                                            _ => println!("Un-Pause failed"),
                                        }
                                    } else {
                                        match mio_socket.pause() {
                                            Ok(_) => println!("Paused"),
                                            _ => println!("Pause failed"),
                                        }
                                    }
                                }
                                _ => {
                                    //loop {
                                    // A frame should be ready
                                    match mio_socket.read_frame() {
                                        Ok(frame) => {
                                            let id = frame.id() as i32;
                                            let mut data = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
                                            data.copy_from_slice(frame.data());
                                            let mut array = [
                                                as_mut_object!(dart_c_int!(id, i32)),         // ID of the frame
                                                as_mut_object!(dart_c_bool!(frame.is_rtr())), // Is the frame remote?
                                                as_mut_object!(dart_c_typed_data!(data, u8)), // Payload
                                                as_mut_object!(dart_c_bool!(frame.is_error())), // Is this an Error frame
                                                as_mut_object!(dart_c_string!(siface.as_mut_ptr())), // Is this an Error frame
                                            ];
                                            let mut dart_array = dart_c_array!(array);
                                            send!(port, dart_array);
                                        }
                                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                            send!(port, dart_c_bool!(false));
                                            break;
                                        }
                                        Err(_e) => break,
                                    }
                                }
                            }
                        }
                    }
                });
                self.sockets
                    .write()
                    .insert(String::from(iface), ((waker, remote), mio_dup));
                Ok(())
            }
            Err(_e) => Err(io::Error::new(io::ErrorKind::NotFound, "E")),
        }
    }

    // Clone transmitter and dispatch frames
    // Connects but allows for synchronous listening
    // NB: Will be deprecated!
    pub fn connect_sync(&self, iface: &str) -> Result<(), io::Error> {
        match CANSocket::open(iface) {
            Ok(socket) => {
                socket.set_nonblocking(true)?;
                let mut mio_socket = local::MIOCANSocket::from(socket);
                let mut poll = Poll::new()?;

                poll.registry().register(
                    &mut mio_socket,
                    POLL_TOKEN,         // This would have to be dynamic!
                    Interest::READABLE, // I still don't understand this! :(
                )?;

                let waker = Waker::new(poll.registry(), WAKER_TOKEN)?;
                let remote = Waker::new(poll.registry(), PAUSE_TOKEN)?;

                let mut events = Events::with_capacity(1024); // 1kb events
                let transmitter = self.transmitter.lock().clone();

                let mio_dup = mio_socket.clone();
                let siface = String::from(iface);

                spawn(move || {
                    'handler: loop {
                        poll.poll(&mut events, None).unwrap();

                        for event in events.iter() {
                            match event.token() {
                                WAKER_TOKEN => {
                                    println!("I AM AWOKEN AND EXITING");
                                    drop(transmitter);
                                    break 'handler;
                                }
                                PAUSE_TOKEN => {
                                    if mio_socket.ispaused() {
                                        match mio_socket.unpause() {
                                            Ok(_) => println!("Un-Paused"),
                                            _ => println!("Un-Pause failed"),
                                        }
                                    } else {
                                        match mio_socket.pause() {
                                            Ok(_) => println!("Paused"),
                                            _ => println!("Pause failed"),
                                        }
                                    }
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
                                }
                            }
                        }
                    }
                });
                self.sockets
                    .write()
                    .insert(String::from(iface), ((waker, remote), mio_dup));
                Ok(())
            }
            Err(_e) => Err(io::Error::new(io::ErrorKind::NotFound, "E")),
        }
    }

    pub fn killall(&self) -> Result<(), io::Error> {
        // TODO: use map instead of for loop!
        // NB: Using drain on hashmap won't wake the socket!
        for (_key, ((waker, _remote), _socket)) in self.sockets.write().iter() {
            match waker.wake() {
                Ok(_) => {}
                _ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
            }
        }
        Ok(())
    }

    pub fn kill(&self, iface: &str) -> Result<(), io::Error> {
        let mut guard = self.sockets.write();
        match guard.get_mut(iface) {
            Some(((waker, _remote), _socket)) => match waker.wake() {
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
    }

    // Dispatch a message to an interface!
    // TODO: use frame filters to implement pause functionality
    pub fn dispatch(&self, destination: &str, message: CANFrame) -> Result<(), io::Error> {
        match self.sockets.read().get(destination) {
            Some((_handle, socket)) => socket.write_frame_insist(message),
            None => return Err(io::Error::from(io::ErrorKind::AddrNotAvailable)),
        }
    }

    pub fn toggle(&self, iface: &str) -> Result<(), io::Error> {
        match self.sockets.read().get(iface) {
            Some(((_handle, remote), socket)) => remote.wake(),
            None => return Err(io::Error::from(io::ErrorKind::AddrNotAvailable)),
        }
    }
}
