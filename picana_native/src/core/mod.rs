//extern crate memmap;
//extern crate socketcan;
mod connections;
pub mod definitions;
mod dump_parser;
mod mmaped_file;
mod mmaped_file_manager;

use log::warn;
use parking_lot::Mutex;
use socketcan::{dump::ParseError, CANFrame};
use std::io;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub struct Picana {
    manager: mmaped_file_manager::MmapedFileManager,
    framelibrary: definitions::FrameDefinitionLibrary,
    connections: connections::ConnectionManager,
    //TODO: Will be deprecated in favor of async API
    transmitter: Mutex<Sender<(i8, Option<(String, CANFrame)>)>>,
    receiver: Mutex<Receiver<(i8, Option<(String, CANFrame)>)>>,
}

#[allow(unused_variables)]
impl Picana {
    pub fn new() -> Self {
        let (transmitter, receiver) = channel::<(i8, Option<(String, CANFrame)>)>();
        let manager = mmaped_file_manager::MmapedFileManager::start();
        let framelibrary = definitions::FrameDefinitionLibrary::new();
        let receiver = Mutex::new(receiver);
        let tx = Mutex::new(transmitter.clone());
        let transmitter = Mutex::new(transmitter);
        let connections = connections::ConnectionManager::from(tx);
        Picana {
            manager,
            framelibrary,
            connections,
            transmitter,
            receiver,
        }
    }

    ///Utilities for manipulating candumps
    pub fn open(&mut self, handle: &str, path: &str) -> Result<usize, io::Error> {
        self.manager.add_file(path, handle)
    }

    pub fn load_dbc(&mut self, handle: &str, dbcfile: &str) -> Result<(), io::Error> {
        self.framelibrary.load(handle, dbcfile)
    }

    pub fn line(&self, key: &str, line: usize) -> Result<&str, &str> {
        self.manager.line_at(key, line)
    }

    pub fn frame(
        &self,
        key: &str,
        line: usize,
    ) -> Result<Option<dump_parser::CanFrameData>, ParseError> {
        match self.manager.bytes_at(key, line) {
            Ok(bytes) => {
                let data = dump_parser::decode_frame_memchr(bytes)?;
                Ok(Some(data))
            }
            _ => Ok(None),
        }
    }

    // Explain a parameter (SPN) belonging to a certain file?
    pub fn explain(
        &self,
        key: &str,
        parameter: &str,
    ) -> Result<definitions::ValueDefinitionBridge, io::Error> {
        match self.framelibrary.define(key, parameter) {
            Some(definition) => Ok(definition),
            None => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }

    ///Connect to an interface locally
    pub fn connect(
        &self,
        interface: &str,
        port: Option<i64>, //handler: Option<extern "C" fn(libc::c_int) -> libc::c_int>,
    ) -> Result<(), io::Error> {
        match port {
            Some(dart_port) => match self.connections.connect_async(interface, dart_port) {
                Ok(r) => Ok(r),
                Err(e) => {
                    warn!("CONNECT: Fatal - => {:?}\n", e);
                    Err(io::Error::from(io::ErrorKind::NotFound))
                }
            },
            None => match self.connections.connect_sync(interface) {
                Ok(r) => Ok(r),
                Err(e) => {
                    warn!("CONNECT: Fatal - => {:?}\n", e);
                    Err(io::Error::from(io::ErrorKind::NotFound))
                }
            },
        }
    }

    //NB: If you are trying to acquire a Write guard whilst this connection is running you won't
    //be able to obtain it because the read won't be free until the listen is closed
    //manually(its on an infinite loop).
    //
    //This means you'll deadlock!
    //
    //NB: This will be deprecated in favor of async connections, available now for historical
    //purpose
    pub fn listen(
        &self,
        callback: Option<extern "C" fn(*const super::picana::FrameResource) -> libc::c_int>,
    ) -> i32 {
        //let mut count = 0;
        match callback {
            Some(handler) => 'handler: loop {
                //print!("Looped!\n");
                match self.receiver.lock().recv() {
                    Ok((0, Some((iface, canframe)))) => {
                        let now = SystemTime::now();
                        let t_usec = match now.duration_since(UNIX_EPOCH) {
                            Ok(t_dur) => t_dur.as_secs(),
                            _ => return -1,
                        };
                        let exitframe =
                            super::picana::FrameResource::from(t_usec, iface.as_str(), canframe);
                        let framebox = Box::into_raw(Box::new(exitframe));
                        handler(framebox);
                        0
                    }
                    Ok((code, None)) => {
                        break 'handler 0;
                    }
                    Err(e) => {
                        warn!("LISTEN: Eeeh--> now this {}", e);
                        -1
                    }
                    _ => {
                        warn!("Unhandled exit!");
                        break 'handler 0;
                    }
                };
            },
            _ => {
                warn!("LISTEN: No handler!");
                -1
            }
        }
    }

    /// Post a message to an interface
    pub fn tell(&self, who: &str, what: CANFrame) -> Result<(), io::Error> {
        self.connections.dispatch(who, what)
    }

    /// Close an interface
    pub fn close(&self, iface: &str) -> Result<(), io::Error> {
        self.connections.kill(iface)
    }

    /// Toggle an interface
    pub fn toggle(&self, iface: &str) -> Result<(), io::Error> {
        self.connections.toggle(iface)
    }

    /// Close all interfaces
    //TODO: Implement some sort of pause functionality!
    pub fn finish(&self) -> i32 {
        match self.connections.killall() {
            Ok(_) => match self.transmitter.lock().send((-1, None)) {
                Ok(transmitter) => 0,
                _ => -1,
            },
            _ => -99,
        }
    }
}
