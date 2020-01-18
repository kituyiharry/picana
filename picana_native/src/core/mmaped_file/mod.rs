use hashbrown::HashMap;
use memmap::Mmap;
use std::fs::File;
use std::io::Error;

#[derive(Debug)]
pub struct MmapedFile {
    file_handle: File,
    current_offset: usize,
    indexed_lines: usize,
    pub is_indexed: bool,
    mapping: Mmap,
    pub line_tree: HashMap<usize, (usize, usize)>, //Hashmap for lines,  hold offset and length
    pub file_size: u64,
}

#[allow(dead_code)]
// Not safe for any concurrent access
impl MmapedFile {
    pub fn initialize(path: &str) -> Result<MmapedFile, Error> {
        let file_handle: File = File::open(path)?;
        //let file_size = (std::fs::metadata(path))?.len();
        let mapping = unsafe { Mmap::map(&file_handle)? };
        let indexed_lines = 0;

        Ok(MmapedFile {
            file_handle: file_handle,
            current_offset: 0, // Start at 0
            file_size: mapping.len() as u64,
            is_indexed: false,
            indexed_lines: indexed_lines,
            //line_tree: BTreeMap::new(),
            line_tree: HashMap::new(),
            mapping: mapping,
        })
    }

    fn add_index(&mut self, key: usize, value: (usize, usize)) {
        self.line_tree.insert(key, value);
    }

    // Creates a btree index for offsets in the map
    pub fn index(&mut self) -> usize {
        self.indexed_lines = 0;
        while let Some(offsets) = self.next() {
            self.add_index(self.indexed_lines, offsets);
            self.indexed_lines += 1;
        }
        self.is_indexed = true;
        self.reset(None);
        self.indexed_lines
    }

    // Lookup a line! requires the file be indexed to avoid iteration
    pub fn lookup(&self, line: usize) -> Result<(usize, usize), &str> {
        if self.is_indexed {
            match self.line_tree.get(&line) {
                Some(value) => Ok(*value),
                _ => Err("Lookup failed!"),
            }
        } else {
            Err("You need to index the file")
        }
    }

    // Resets the index to a
    fn reset(&mut self, new_offset: Option<usize>) -> () {
        match new_offset {
            Some(value) => self.current_offset = value,
            _ => self.current_offset = 0,
        }
    }

    // Retrieve the bytes at the range from the mmaped file
    // takes an offset and a length from that offset
    pub fn bytes(&self, offset_at: usize, buffer_length: usize) -> &[u8] {
        &self.mapping[offset_at..(offset_at + buffer_length)]
    }

    //Checks whether a region false within a certain offset
    fn is_valid_region(&self, offset: usize) -> bool {
        offset < self.mapping.len()
    }

    // Assumes starting at middle of new line -> Return Option with offset and length of line from
    // offset
    fn get_line_fuzzy(&self, from_offset: usize) -> Option<(usize, usize)> {
        let mut begin_offset: usize = from_offset;
        let mut len_offset: usize = 0;

        //Are we even within the file
        if !self.is_valid_region(begin_offset) {
            return None;
        }

        // Are we at the beginning of the line?
        loop {
            match self.mapping[begin_offset] {
                b'\n' => {
                    // So we begin from the next line
                    begin_offset = begin_offset + 1;
                    break;
                }
                _byte => {
                    //We are at the start of the file
                    if begin_offset == 0 {
                        break;
                    }
                    //Decrement till we hit a  newline or start of file
                    begin_offset = begin_offset - 1;
                }
            }
        }
        // So wheres the end of the line?
        loop {
            if !self.is_valid_region(begin_offset + len_offset) {
                break Some((begin_offset, len_offset));
            } else {
                match self.mapping[begin_offset + len_offset] {
                    b'\n' => {
                        //Uncomment to include the newline and break
                        //len_offset += 1;
                        break Some((begin_offset, len_offset));
                    }
                    b'\0' => {
                        break Some((begin_offset, len_offset));
                    }
                    _byte => {
                        len_offset += 1;
                    }
                }
            }
        }
    }
}

// Iterate over the lines in the file
impl Iterator for MmapedFile {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        //let offset: usize = self.current_offset;

        if !self.is_valid_region(self.current_offset as usize) {
            None
        } else {
            match self.get_line_fuzzy(self.current_offset) {
                Some((offset, length)) => {
                    self.current_offset = offset + length;
                    //self.indexed_lines += 1;
                    //self.add_index(self.indexed_lines, (offset, length));
                    Some((offset, length))
                }
                _ => None,
            }
        }
    }
}
