// Connection manager
//
use socketcan::{CANSocket, CANSocketOpenError};
use std::collections::HashMap;

struct Connection {
    sock: CANSocket,
}

pub struct ConnectionManager {
    sources: HashMap<String, Connection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            sources: HashMap::new(),
        }
    }

    pub fn connect(&mut self, iface: &str) -> Result<(), CANSocketOpenError> {
        let r = match self.sources.get(iface) {
            Some(_connection) => (),
            None => match CANSocket::open(iface) {
                Ok(connection) => {
                    print!("Connection found\n");
                    self.sources
                        .insert(String::from(iface), Connection { sock: connection });
                    ()
                }
                _ => {
                    print!("Nah!\n");
                    ()
                }
            },
        };
        print!("Reached Connection! --> {}\n", iface);
        Ok(r)
    }
}
