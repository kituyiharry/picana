extern crate memmap;

use memmap::Mmap;
use std::fs::File;
use std::io::Error;
use std::str;

use std::collections::BTreeMap;

#[derive(Debug)]
struct MmapedFile {
    file_handle: File,
    current_offset: usize,
    mapping: Mmap,
    line_tree: BTreeMap<u64, (usize, usize)>, //Btreemap for lines,  hold offset and length
    pub file_size: u64,
}

#[allow(dead_code)]
impl MmapedFile {
    fn initialize(path: &str) -> Result<MmapedFile, Error> {
        let file_handle: File = File::open(path)?;
        let file_size = (std::fs::metadata(path))?.len();
        let mapping = unsafe { Mmap::map(&file_handle)? };

        Ok(MmapedFile {
            file_handle: file_handle,
            current_offset: 0, // Start at 0
            file_size: file_size,
            line_tree: BTreeMap::new(),
            mapping: mapping,
        })
    }

    //fn create_index(&mut self, offset: usize, length: u64) {}

    fn reset(&mut self) {
        self.current_offset = 0;
    }

    fn bytes(&mut self, offset_at: usize, buffer_length: usize) -> &[u8] {
        &self.mapping[offset_at..offset_at + buffer_length]
    }

    fn is_valid_region(&self, offset: usize) -> bool {
        offset < self.mapping.len()
    }

    // Assumes starting at middle of new line -> Return Option with offset and length
    fn get_line_fuzzy(&self, from_offset: usize) -> Option<(usize, usize)> {
        let mut begin_offset: usize = from_offset;
        let mut len_offset: usize = 0;
        // Are we at the beginning of the line?
        loop {
                match self.mapping[begin_offset] {
                    b'\n' => {
                        print!("Begin offset : {} \n", begin_offset);
                        begin_offset = begin_offset + 1;
                        break;
                    }
                    _byte => {
                        if begin_offset == 0 || self.mapping[begin_offset - 1] == b'\n' {
                            print!("Sweet spot : {}\n", begin_offset);
                            break;
                        }
                        print!("Decreasing offset : {}\n", begin_offset);
                        begin_offset = begin_offset - 1;
                    }
                }
            }
        }
        // So wheres the end of the line?
        loop {
            if !self.is_valid_region(len_offset) {
                break None;
            } else {
                match self.mapping[len_offset] {
                    b'\n' => {
                        if len_offset > begin_offset {
                            print!("End offset smaller! \n");
                            break Some((begin_offset, len_offset - begin_offset));
                        } else {
                            print!("End offset greater! \n");
                            break Some((begin_offset, len_offset));
                        }
                    }
                    _byte => {
                        len_offset += 1;
                    }
                }
            }
        }
    }
}

impl Iterator for MmapedFile {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let offset: usize = self.current_offset;

        if !self.is_valid_region(offset as usize) {
            None
        } else {
            //let mut v: Vec<u8> = Vec::new();

            match self.get_line_fuzzy(self.current_offset) {
                Some(value) => {
                    self.current_offset = value.1 + 1;
                    print!(
                        "Current End offset - {}, Value - {:?}\t",
                        self.current_offset, value
                    );
                    Some(value)
                }
                _ => None,
            }

            /*loop {
                if offset > size {
                    self.current_offset = offset;
                    //v.push(b'\n');
                    break Some(v);
                } else {
                    match self.mapping[offset as usize] {
                        b'\n' => {
                            self.current_offset = offset + 1;
                            break Some(v);
                        }
                        byte => {
                            offset += 1;
                            v.push(byte);
                        }
                    };
                }
            }*/
        }
    }
}

pub fn run(path: &str) -> Result<(), Error> {
    //let random_bytes: Vec<u8> = random_indexes.iter().map(|&index| map[index]).collect();
    let mut file: MmapedFile = MmapedFile::initialize(path)?;

    for i in file.by_ref().take(3) {
        print!("Line at {:?}\n", i);
    }

    //let file_index = &file.nth(284111).unwrap_or_else(|| vec![0])[..];
    //let file_index_b = &file.nth(200).unwrap_or_else(|| vec![0])[..];
    //let word: &str = unsafe { std::str::from_utf8_unchecked(file_index) };
    //let word_b: &str = unsafe { std::str::from_utf8_unchecked(file_index_b) };

    match file.get_line_fuzzy(375) {
        Some((start, end)) => print!("Word C = {}\n", unsafe {
            std::str::from_utf8_unchecked(file.bytes(start, end))
        }),
        None => print!("Invalid byte offset"),
    }

    //print!("5000 word: -> {}\n, 200th -> {}\n", word, word_b);

    //print!(
    //"File Size = {}, Bytes = {}\n\n",
    //file.file_size, file.current_offset
    //);
    Ok(())
}
