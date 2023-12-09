use std::fmt;

#[derive(Debug)]
pub enum ArchRegister {
    Arch32(Arch32Register),
    Arch64(Arch64Register),
    Arch128(Arch128Register),
}

#[derive(Debug)]
pub struct Arch32Register {
    name: String,
    addr: usize,
    value: u32,
}

#[derive(Debug)]
pub struct Arch64Register {
    name: String,
    addr: usize,
    value: u64,
}

#[derive(Debug)]
pub struct Arch128Register {
    name: String,
    addr: usize,
    value: u128,
}

impl ArchRegister {
    pub fn new(width: usize, name: String, addr: usize, initial_value: &[u8]) -> ArchRegister {
        match width {
            32 => ArchRegister::Arch32(Arch32Register::new(name, addr, u32::from_le_bytes(initial_value.try_into().unwrap()))),
            64 => ArchRegister::Arch64(Arch64Register::new(name, addr, u64::from_le_bytes(initial_value.try_into().unwrap()))),
            128 => ArchRegister::Arch128(Arch128Register::new(name, addr, u128::from_le_bytes(initial_value.try_into().unwrap()))),
            _ => panic!("Unsupported width architecture!"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ArchRegister::Arch32(r) => &r.name,
            ArchRegister::Arch64(r) => &r.name,
            ArchRegister::Arch128(r) => &r.name,
        }
    }

    pub fn set(&mut self, value: &[u8]) -> &mut Self {
        match self {
            ArchRegister::Arch32(r) => r.value = u32::from_le_bytes(value.try_into().unwrap()),
            ArchRegister::Arch64(r) => r.value = u64::from_le_bytes(value.try_into().unwrap()),
            ArchRegister::Arch128(r) => r.value = u128::from_le_bytes(value.try_into().unwrap()),
        }
        
        self
    }
    
    pub fn get(&self) -> Vec<u8> {
        match self {
            ArchRegister::Arch32(r) => r.value.to_le_bytes().to_vec(),
            ArchRegister::Arch64(r) => r.value.to_le_bytes().to_vec(),
            ArchRegister::Arch128(r) => r.value.to_le_bytes().to_vec(),
        }
    }
}

impl fmt::Display for ArchRegister {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            ArchRegister::Arch32(a32) => write!(f, "(${}\t{}\t{:#0x})", a32.addr, a32.name, a32.value),
            ArchRegister::Arch64(a64) => write!(f, "(${}\t{}\t{:#0x})", a64.addr, a64.name, a64.value),
            ArchRegister::Arch128(a128) => write!(f, "(${}\t{}\t{:#0x})", a128.addr, a128.name, a128.value),
        }
    }
}

impl Arch32Register {
    pub fn new(name: String, addr: usize, initial_value: u32) -> Arch32Register {
        Arch32Register {
            name,
            addr,
            value: initial_value,
        }
    }
}

impl Arch64Register {
    pub fn new(name: String, addr: usize, initial_value: u64) -> Arch64Register {
        Arch64Register {
            name,
            addr,
            value: initial_value,
        }
    }
}

impl Arch128Register {
    pub fn new(name: String, addr: usize, initial_value: u128) -> Arch128Register {
        Arch128Register {
            name,
            addr,
            value: initial_value,
        }
    }
}
