//extern crate memmap;
//extern crate socketcan;
mod connections;
pub mod definitions;
mod dump_parser;
mod mmaped_file;
mod mmaped_file_manager;

use log::warn;
use socketcan::{dump::ParseError, CANFrame};
use std::sync::Mutex;
use std::{io, sync::mpsc};

#[allow(dead_code)]
pub struct Picana {
    manager: mmaped_file_manager::MmapedFileManager,
    framelibrary: definitions::FrameDefinitionLibrary,
    connections: connections::ConnectionManager,
    receiver: Mutex<mpsc::Receiver<CANFrame>>,
}

#[allow(unused_variables)]
impl Picana {
    pub fn new() -> Self {
        let (tx, receiver) = mpsc::channel::<CANFrame>();
        let manager = mmaped_file_manager::MmapedFileManager::start();
        let framelibrary = definitions::FrameDefinitionLibrary::new();
        let receiver = Mutex::new(receiver);
        let tx = Mutex::new(tx);
        let connections = connections::ConnectionManager::from(tx);
        Picana {
            manager,
            framelibrary,
            connections,
            receiver,
        }
    }

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

    pub fn connect(
        &mut self,
        interface: &str,
        //handler: Option<extern "C" fn(libc::c_int) -> libc::c_int>,
    ) -> Result<(), io::Error> {
        match self.connections.connect(interface) {
            Ok(r) => {
                //self.listen(handler);
                Ok(r)
            }
            Err(e) => {
                warn!("CONNECT: Fatal - => {:?}\n", e);
                Err(io::Error::from(io::ErrorKind::NotFound))
            }
        }
    }

    pub fn listen(&self, callback: Option<extern "C" fn(libc::c_int) -> libc::c_int>) -> i32 {
        let mut count = 0;
        match callback {
            Some(handler) => loop {
                //print!("Looped!\n");
                match self.receiver.lock() {
                    Ok(recv) => match recv.recv() {
                        Ok(what) => {
                            handler(count);
                            count += 1;
                            0
                        }
                        Err(e) => {
                            warn!("LISTEN: Eeeh--> now this {}", e);
                            -1
                        }
                    },
                    Err(e) => {
                        warn!("LISTEN: Receiver couldn't lock?");
                        -1
                    }
                };
            },
            _ => {
                warn!("LISTEN: No handler!");
                -1
            }
        }
    }

    pub fn tell(&self, who: &str, what: CANFrame) -> Result<(), io::Error> {
        self.connections.dispatch(who, what)
    }
}
