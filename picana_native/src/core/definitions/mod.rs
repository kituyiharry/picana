// Module to read Data definitions and extract useul information from CAN messages
// A CAN DBC file lets you convert raw CAN bus data to physical, readable data.
// By default, a CAN analyzer records raw CAN data
//

use canparse::pgn::{ParseMessage, PgnLibrary, SpnDefinition};
use hashbrown::HashMap;
use std::io;

// Maybe this be a Reference Counted Type
pub struct ValueDefinitionBridge {
    spn: String,
    definition: SpnDefinition,
}

impl ValueDefinitionBridge {
    pub fn interpret(&self, data: &[u8]) -> Option<f32> {
        self.definition.parse_message(data)
    }

    pub fn get_name(&self) -> &String {
        &self.spn
    }
}

#[derive(Debug)]
pub struct FrameDefinitionLibrary {
    pgn_map: HashMap<String, PgnLibrary>,
}

// Holds libraries containining instructions of how to decode messages
impl FrameDefinitionLibrary {
    pub fn new() -> Self {
        let pgn_map = HashMap::new();
        FrameDefinitionLibrary { pgn_map }
    }

    // Same keys will overwrite
    pub fn load(&mut self, key: &str, file: &str) -> Result<(), io::Error> {
        let pgnlibrary = PgnLibrary::from_dbc_file(file)?;
        self.pgn_map.insert(String::from(key), pgnlibrary);
        Ok(())
    }

    // Gets the definition of something
    pub fn define(&self, key: &str, spn: &str) -> Option<ValueDefinitionBridge> {
        match self.pgn_map.get(key) {
            Some(library) => {
                if let Some(spndef) = library.get_spn(spn) {
                    let def = ValueDefinitionBridge {
                        spn: String::from(spn),
                        definition: spndef.clone(),
                    };
                    Some(def)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_valid_dbc() {
        let mut library = FrameDefinitionLibrary::new();
        let res = library.load("test", "../../../test/zeva_30.dbc").unwrap();
        assert_eq!(res, ())
    }
}
