use std::fmt::{self, Display};

#[derive(Debug)]
pub struct AtomicCtx {
    xlen: usize,
    address: u64,
    valid: bool,
}

impl AtomicCtx {
    pub fn new(xlen: usize) -> Self {
        Self {
            xlen,
            address: 0,
            valid: false,
        }
    }

    pub fn reserve(&mut self, address: u64) {
        self.address = address;
        self.valid = true;
    }

    pub fn check(&self, address: u64) -> bool {
        if self.valid && self.address == address { true } else { false }
    }

    pub fn release(&mut self) {
        self.address = 0;
        self.valid = false;
    }
}

impl Display for AtomicCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "(atomic-ctx ((address {:0x}) (valid {})))", self.address, self.valid)
    }
}