use std::fmt;
use std::fmt::Debug;

use crate::vsoc::peripheral::Peripheral;
use crate::vsoc::peripheral::PeripheralInterface;

#[derive(Debug)]
pub enum BusException {
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
}

#[derive(Debug, Default)]
pub struct Bus {
    map: Vec<(u64, Box<Peripheral>)>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            map: Vec::<(u64, Box<Peripheral>)>::new(),
        }
    }

    pub fn attach(&mut self, origin: u64, p: Box<Peripheral>) {
        self.map.push((origin, p));
        self.map.sort_by(|a, b| a.0.cmp(&b.0));
    }

    pub fn fetch(&mut self, width: usize, addr: u64) -> Result<Vec<u8>, BusException> {
        for (origin, p) in self.map.iter_mut() {
            if addr >= *origin && addr < *origin + p.size() as u64 {
                if addr % width as u64 == 0 {
                    return p.fetch(width, (addr - *origin) as usize);
                } else {
                    let base: u64 = addr - *origin;
                    let mut value: Vec<u8> = Vec::new();

                    for i in 0..width {
                        value.push(p.fetch(1, base as usize + i).unwrap()[0]);
                    }

                    return Ok(value);
                }
            }
        }

        Err(BusException::LoadAccessFault)
    }

    pub fn store(&mut self, width: usize, addr: u64, value: &Vec<u8>) -> Option<BusException> {
        for (origin, p) in self.map.iter_mut() {
            if *origin <= addr && addr < *origin + p.size() as u64 {
                if width == 1 || addr % width as u64 == 0 {
                    return p.store(width, (addr - *origin) as usize, value);
                } else {
                    let base: u64 = addr - *origin;
                    for i in 0..width {
                        match p.store(1, base as usize + i, &[value[i]].to_vec()) {
                            Some(e) => return Some(e),
                            None => (),
                        }
                    }

                    return None;
                }
            }
        }

        Some(BusException::StoreAccessFault)
    }
}

impl fmt::Display for Bus {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match writeln!(f, "(bus (") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        for (origin, p) in self.map.iter() {
            match writeln!(
                f,
                "  ({:#0x}\t{:#0x}\t{})",
                origin,
                origin + p.size as u64,
                p.name
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        writeln!(f, "  )\n )")
    }
}

#[cfg(test)]
mod tests {
    use crate::vsoc::bus::{Bus, BusException};
    use crate::vsoc::peripheral::{Peripheral, PeripheralInterface};

    #[derive(Debug)]
    struct PeripheralLikeStruct;

    impl PeripheralInterface for PeripheralLikeStruct {
        fn fetch(&mut self, _width: usize, addr: usize) -> Result<Vec<u8>, BusException> {
            if (0x0000..0x1000).contains(&addr) {
                return Ok(u8::to_le_bytes(0x01).to_vec());
            }

            Err(BusException::LoadAccessFault)
        }

        fn store(&mut self, _width: usize, addr: usize, _value: &Vec<u8>) -> Option<BusException> {
            if (0x0000..0x1000).contains(&addr) {
                return None;
            }

            Some(BusException::StoreAccessFault)
        }
    }

    #[test]
    fn test_fetch() {
        let mut b: Bus = Bus::new();
        let binding = Box::new(PeripheralLikeStruct);
        let p = Peripheral::new(String::from("test"), 0x1000, binding);

        b.attach(0x8000_0000, Box::new(p));

        assert!(b.fetch(1, 0x8000_0000 - 1).is_err());
        assert_eq!(
            u8::from_le_bytes(b.fetch(1, 0x8000_0000).unwrap().try_into().unwrap()),
            0x01
        );
        assert!(b.fetch(1, 0x8000_1000).is_err());
    }

    #[test]
    fn test_store() {
        let mut b: Bus = Bus::new();
        let binding = Box::new(PeripheralLikeStruct);
        let p = Peripheral::new(String::from("test"), 0x1000, binding);

        b.attach(0x8000_0000, Box::new(p));

        assert!(b.store(1, 0x8000_0000 - 1, &[1u8; 1].to_vec()).is_some());
        assert!(b.store(1, 0x8000_0000, &[0u8; 1].to_vec()).is_none());
        assert!(b.store(1, 0x8000_1000, &[0u8; 1].to_vec()).is_some());
    }
}
