extern crate memmap;
extern crate socketcan;
pub mod connections;
pub mod definitions;
pub mod dump_parser;
pub mod mmaped_file;
pub mod mmaped_file_manager;

use socketcan::{dump::ParseError, CANFrame};
use std::{io, sync::mpsc};

#[allow(dead_code)]
pub struct Picana {
    manager: mmaped_file_manager::MmapedFileManager,
    framelibrary: definitions::FrameDefinitionLibrary,
    connections: connections::ConnectionManager,
}

#[allow(unused_variables)]
impl Picana {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<CANFrame>();
        let manager = mmaped_file_manager::MmapedFileManager::start();
        let framelibrary = definitions::FrameDefinitionLibrary::new();
        let connections = connections::ConnectionManager::from(tx).expect("Some good!");
        Picana {
            manager,
            framelibrary,
            connections,
        }
    }

    pub fn open(&mut self, handle: &str, path: &str) -> Result<usize, io::Error> {
        self.manager.add_file(path, handle)
    }

    pub fn load_dbc(&mut self, handle: &str, dbcfile: &str) -> Result<(), io::Error> {
        self.framelibrary.load(handle, dbcfile)
    }

    // TODO: use error types
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
                let data = dump_parser::decode_frame(bytes)?;
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
        callback: Option<extern "C" fn(libc::c_int) -> libc::c_int>,
    ) -> Result<(), io::Error> {
        print!("Connecting!!\n");
        //TODO: Receive data from rx channel!
        match self.connections.connect(interface) {
            Ok(r) => Ok(r),
            Err(e) => {
                print!("Fatal - => {:?}\n", e);
                Err(io::Error::from(io::ErrorKind::NotFound))
            }
        }
    }
}
