use std::fmt;

use super::types::Uint;

#[derive(Debug)]
pub struct ArchRegister {
    name: String,
    addr: usize,
    value: Uint,
}

impl ArchRegister {
    pub fn new(_width: usize, name: String, addr: usize, initial_value: Uint) -> ArchRegister {
        ArchRegister {
            name,
            addr,
            value: initial_value.clone(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set(&mut self, value: &Uint) {
        self.value = value.clone();
    }

    pub fn get(&self) -> Uint {
        self.value.clone()
    }
}

impl fmt::Display for ArchRegister {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "(${}\t{}\t{})", self.addr, self.name, &self.value)
    }
}
