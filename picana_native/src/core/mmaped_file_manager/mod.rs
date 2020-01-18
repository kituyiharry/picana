//use std::collections::HashMap;
use hashbrown::HashMap;
use std::io::Error;

use super::mmaped_file::MmapedFile;

#[derive(Debug)]
pub struct MmapedFileManager {
    // Use String to own it!
    mmaped_files: HashMap<String, MmapedFile>,
}

#[allow(dead_code)]
impl MmapedFileManager {
    pub fn start() -> Self {
        MmapedFileManager {
            mmaped_files: HashMap::new(),
        }
    }

    //
    //A quick note about usize
    //Often, marshaling between types is pretty straightforward:
    //f64 to double, u64 to ulong, or simply i32 to int.
    //Rust’s usize, however, turned out to be the most varied, and most ambiguous, type-mapping amongst host languages.
    //The usize type represents an unsigned number the width of a pointer (like 32-bit or 64-bit).
    //This varies by the host platform’s OS so while you could use a ulong or uint32 on your machine it might break elsewhere.
    //Since Rust uses usize quite often for ranges and indices: always make sure to use a type that represents a platform-specific width.
    //In Node.js you’ll want size_t and in C# (or Unity3D) UIntPtr seems to do the trick.
    pub fn add_file(&mut self, path: &str, key: &str) -> Result<usize, Error> {
        if self.mmaped_files.len() >= 3 {
            Ok(0)
        } else {
            let mut mmaped_file = MmapedFile::initialize(path)?;
            let len = mmaped_file.index();
            self.mmaped_files.insert(String::from(key), mmaped_file);
            Ok(len)
        }
    }

    pub fn len_of(&self, key: &str) -> Result<usize, &str> {
        match self.mmaped_files.get(&String::from(key)) {
            Some(value) => Ok((*value).line_tree.len()),
            _ => Err("Meh!"),
        }
    }

    pub fn bytes_at(&self, key: &str, line_no: usize) -> Result<&[u8], &str> {
        match self.mmaped_files.get(key) {
            Some(mmaped_file) => {
                let (offset, length) = mmaped_file.lookup(line_no)?;
                let line: &[u8] = mmaped_file.bytes(offset, length);
                Ok(line)
            }
            _ => Err("Malformed line!"),
        }
    }

    pub fn line_at(&self, key: &str, line_no: usize) -> Result<&str, &str> {
        match self.mmaped_files.get(key) {
            Some(mmaped_file) => {
                let (offset, length) = mmaped_file.lookup(line_no)?;
                let line: &str =
                    unsafe { std::str::from_utf8_unchecked(mmaped_file.bytes(offset, length)) };
                Ok(line)
            }
            _ => Err("Malformed line"),
        }
    }
}
