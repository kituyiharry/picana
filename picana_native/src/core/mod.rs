extern crate memmap;
extern crate socketcan;
pub mod mmaped_file;
pub mod mmaped_file_manager;
pub mod parser;

use socketcan::dump::ParseError;
use std::io::Error;

#[allow(dead_code)]
pub struct Picana {
    manager: mmaped_file_manager::MmapedFileManager,
}

#[allow(unused_variables)]
impl Picana {
    pub fn new() -> Self {
        let manager = mmaped_file_manager::MmapedFileManager::start();
        Picana { manager }
    }

    pub fn open(&mut self, handle: &str, path: &str) -> Result<usize, Error> {
        self.manager.add_file(path, handle)
    }

    // TODO: use error types
    pub fn line(&self, key: &str, line: usize) -> Result<&str, &str> {
        self.manager.line_at(key, line)
    }

    pub fn frame(
        &self,
        key: &str,
        line: usize,
    ) -> Result<Option<parser::CanFrameData>, ParseError> {
        match self.manager.bytes_at(key, line) {
            Ok(bytes) => {
                let data = parser::decode_frame(bytes)?;
                Ok(Some(data))
            }
            _ => Ok(None),
        }
    }
}
