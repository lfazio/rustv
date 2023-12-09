use crate::vsoc::bus::BusException;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Peripheral {
    pub name: String,
    pub size: usize,
    pub io: Box<dyn PeripheralInterface>,
}

pub trait PeripheralInterface: Debug {
    fn fetch(&mut self, width: usize, addr: usize) -> Result<Vec<u8>, BusException>;
    fn store(&mut self, width: usize, addr: usize, value: &Vec<u8>) -> Option<BusException>;
}

impl Peripheral {
     pub fn new(name: String, size: usize, peripheral: Box<dyn PeripheralInterface>) -> Peripheral {
        Peripheral {
	    name,
            size,
            io: peripheral,
        }
    }

    pub fn size(&self) -> usize {
	self.size
    }
}

impl PeripheralInterface for Peripheral {
    fn fetch(&mut self, width: usize, addr: usize) -> Result<Vec<u8>, BusException> {
        if addr + width > self.size {
            return Err(BusException::LoadAccessFault);
        }

        if width != 1 && width != 2 && width != 4 && width != 8 && width != 16 {
            return Err(BusException::LoadAccessFault);
        }

        self.io.fetch(width, addr)
    }

    fn store(&mut self, width: usize, addr: usize, value: &Vec<u8>) -> Option<BusException> {
        if addr + width > self.size {
            return Some(BusException::LoadAccessFault);
        }

        if width != 1 && width != 2 && width != 4 && width != 8 {
            return Some(BusException::LoadAccessFault);
        }

        self.io.store(width, addr, value)
    }
}

impl fmt::Display for Peripheral {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({} {:x})", self.name, self.size)
    }
}


#[cfg(test)]
mod tests {
    use super::{Peripheral, PeripheralInterface};
    use crate::vsoc::bus::BusException;

    #[derive(Debug)]
    pub struct PeripheralLikeStruct;

    impl PeripheralInterface for PeripheralLikeStruct {
        fn fetch(&mut self, _width: usize, addr: usize) -> Result<Vec<u8>, BusException> {
            if addr < 0x0000_1000 {
                return Ok(u64::to_le_bytes(0xfee1c001fee1c001).to_vec());
            }

            Err(BusException::LoadAccessFault)
        }

        fn store(&mut self, _width: usize, addr: usize, _value: &Vec<u8>) -> Option<BusException> {
            if addr < 0x0000_1000 {
                return None;
            }

            Some(BusException::LoadAccessFault)
        }
    }

    #[test]
    fn test_store_peripheral() {
        let mut io = PeripheralLikeStruct;
        let mut p: Peripheral =
            Peripheral::new(String::from("test"), 0x1000, Box::new(io));

        assert!(p.store(1, 0x0, &[0u8; 1].to_vec()).is_none());
        assert!(p.store(1, 0x1000, &[0u8; 1].to_vec()).is_some());
    }

    #[test]
    fn test_fetch_peripheral() {
        let mut io = PeripheralLikeStruct;
        let mut p: Peripheral = Peripheral::new(String::from("test"), 0x1000, Box::new(io));

        assert_eq!(u64::from_le_bytes(p.fetch(8, 0x0).unwrap().try_into().unwrap()), 0xfee1c001fee1c001);
        assert!(p.fetch(8, 0x1000).is_err());
    }
}
