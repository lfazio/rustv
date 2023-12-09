use crate::vsoc::bus::BusException;
use crate::vsoc::peripheral::PeripheralInterface;

#[derive(Debug)]
pub struct Flash {
    length: usize,
    data: Vec<u32>,
}

impl Flash {
    pub fn new(length: usize) -> Flash {
        Flash {
            length,
            data: vec![0; length],
        }
    }

    pub fn size(&self) -> usize {
        self.length
    }

    fn fetch8(&self, addr: usize) -> u8 {
        let cell: usize = (addr & !3) >> 2;
        let _offset: usize = (addr % 4) * 8;
        let value = self.data[cell];

        ((value >> _offset) & 0xff) as u8
    }

    fn fetch16(&self, addr: usize) -> u16 {
        let cell = (addr & !3) >> 2;
        let _offset = (addr % 4) * 8;
        let value = self.data[cell];

        ((value >> _offset) & 0xffff) as u16
    }

    fn fetch32(&self, addr: usize) -> u32 {
        let cell = addr >> 2;

        self.data[cell]
    }

    fn fetch64(&self, addr: usize) -> u64 {
        let hi = self.fetch32(addr + 4);
        let lo = self.fetch32(addr);
        let value: u64 = ((hi as u64) << 32) | (lo as u64);

        value
    }

    fn store8(&mut self, addr: usize, value: u8) {
        let cell = (addr & !3) >> 2;
        let _offset = (addr % 4) * 8;

        self.data[cell] &= !(0x000000ffu32 << _offset);
        self.data[cell] |= (value as u32) << _offset;
    }

    fn store16(&mut self, addr: usize, value: u16) {
        let cell = (addr & !3) >> 2;
        let _offset = (addr % 4) * 8;

        self.data[cell] &= !(0xffffu32 << _offset);
        self.data[cell] |= (value as u32) << _offset;
    }

    fn store32(&mut self, addr: usize, value: u32) {
        let cell: usize = addr >> 2;

        self.data[cell] = value;
    }

    fn store64(&mut self, addr: usize, value: u64) {
        let hi = value as u32;
        let lo = (value >> 32) as u32;

        self.store32(addr, lo);
        self.store32(addr + 4, hi);
    }
}

impl PeripheralInterface for Flash {
    fn fetch(&mut self, width: usize, addr: usize) -> Result<Vec<u8>, BusException> {
        let mut align: usize = width;

        if width > 4 {
            align = 4;
        }

        if  addr + width > self.length {
            return Err(BusException::LoadAccessFault);
        }

        if align > 1 && ((addr % align) > 0 || (addr + width) % align > 0) {
            return Err(BusException::LoadAddressMisaligned);
        }

        match width {
            1 => Ok(u8::to_le_bytes(self.fetch8(addr)).to_vec()),
            2 => Ok(u16::to_le_bytes(self.fetch16(addr)).to_vec()),
            4 => Ok(u32::to_le_bytes(self.fetch32(addr)).to_vec()),
            8 => Ok(u64::to_le_bytes(self.fetch64(addr)).to_vec()),
            _ => Err(BusException::LoadAccessFault),
        }
    }

    fn store(&mut self, width: usize, addr: usize, value: &Vec<u8>) -> Option<BusException> {
        let mut align: usize = width;
        let mut v = value.clone();

        if width > 4 {
            align = 4;
        }

        if addr + width > self.length {
                return Some(BusException::StoreAccessFault);
        }

        if align > 1 && ((addr % align) > 0 || (addr + width) % align > 0) {
            return Some(BusException::StoreAddressMisaligned);
        }

        v.truncate(width);
        v.shrink_to_fit();
        match width {
            1 => self.store8(addr, u8::from_le_bytes(v.try_into().unwrap())),
            2 => self.store16(addr, u16::from_le_bytes(v.try_into().unwrap())),
            4 => self.store32(addr, u32::from_le_bytes(v.try_into().unwrap())),
            8 => self.store64(addr, u64::from_le_bytes(v.try_into().unwrap())),
            _ => return Some(BusException::StoreAccessFault),
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::vsoc::bus::BusException;
    use crate::vsoc::peripheral::PeripheralInterface;
    use crate::vsoc::dev::flash::Flash;

    #[test]
    fn getters() {
        let flash: Flash = Flash::new(16);

        assert_eq!(flash.size(), 16);
    }

    #[test]
    fn fetch_above_length() {
        let mut flash: Flash = Flash::new(16);

        assert!(flash.fetch(1, 0x10).is_err());
        assert!(flash.fetch(2, 0x10).is_err());
        assert!(flash.fetch(4, 0x10).is_err());
        assert!(flash.fetch(8, 0x10).is_err());

        assert!(flash.fetch(1, 0x10).is_err());
        assert!(flash.fetch(2, 0x0f).is_err());
        assert!(flash.fetch(4, 0x0d).is_err());
        assert!(flash.fetch(8, 0x09).is_err());
    }

    #[test]
    fn store_fetch_width1() {
        let mut flash: Flash = Flash::new(16);

        assert!(flash.store(1, 0x0, &[0x78; 1].to_vec()).is_none());
        assert_eq!(u8::from_le_bytes(flash.fetch(1, 0x0).unwrap().try_into().unwrap()), 0x78);

        assert!(flash.store(1, 0x1, &[0x78; 1].to_vec()).is_none());
        assert_eq!(u8::from_le_bytes(flash.fetch(1, 0x1).unwrap().try_into().unwrap()), 0x78);

        assert!(flash.store(1, 0x2, &[0x78; 1].to_vec()).is_none());
        assert_eq!(u8::from_le_bytes(flash.fetch(1, 0x2).unwrap().try_into().unwrap()), 0x78);

        assert!(flash.store(1, 0x3, &[0x78; 1].to_vec()).is_none());
        assert_eq!(u8::from_le_bytes(flash.fetch(1, 0x3).unwrap().try_into().unwrap()), 0x78);
    }

    #[test]
    fn store_fetch_width2() {
        let mut flash: Flash = Flash::new(16);

        assert!(flash.store(2, 0x0, &u16::to_le_bytes(0x5678).to_vec()).is_none());
        assert_eq!(u16::from_le_bytes(flash.fetch(2, 0x0).unwrap().try_into().unwrap()), 0x5678);

        /* Unaligned access */
        match flash.store(2, 0x1, &u16::to_le_bytes(0x5678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(2, 0x1).is_err());

        assert!(flash.store(2, 0x2, &u16::to_le_bytes(0x5678).to_vec()).is_none());
        assert_eq!(u16::from_le_bytes(flash.fetch(2, 0x2).unwrap().try_into().unwrap()), 0x5678);

        /* Unaligned access */
        match flash.store(2, 0x3, &u16::to_le_bytes(0x5678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(2, 0x3).is_err());

        assert!(flash.store(2, 0x4, &u16::to_le_bytes(0x5678).to_vec()).is_none());
        assert_eq!(u16::from_le_bytes(flash.fetch(2, 0x4).unwrap().try_into().unwrap()), 0x5678);
    }

    #[test]
    fn store_fetch_width4() {
        let mut flash: Flash = Flash::new(16);

        assert!(flash.store(4, 0x0, &u32::to_le_bytes(0x12345678).to_vec()).is_none());
        assert_eq!(u32::from_le_bytes(flash.fetch(4, 0x0).unwrap().try_into().unwrap()), 0x12345678);

        /* Unaligned access */
        match flash.store(4, 0x1, &u32::to_le_bytes(0x12345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(4, 0x1).is_err());

        /* Unaligned access */
        match flash.store(4, 0x2, &u32::to_le_bytes(0x12345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(4, 0x2).is_err());

        /* Unaligned access */
        match flash.store(4, 0x3, &u32::to_le_bytes(0x12345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(4, 0x3).is_err());

        assert!(flash.store(4, 0x4, &u32::to_le_bytes(0x12345678).to_vec()).is_none());
        assert_eq!(u32::from_le_bytes(flash.fetch(4, 0x4).unwrap().try_into().unwrap()), 0x12345678);
    }

    #[test]
    fn store_fetch_width8() {
        let mut flash: Flash = Flash::new(16);

        assert!(flash.store(8, 0x0, &u64::to_le_bytes(0x1234567812345678).to_vec()).is_none());
        assert_eq!(u64::from_le_bytes(flash.fetch(8, 0x0).unwrap().try_into().unwrap()), 0x1234567812345678);

        /* Unaligned access */
        match flash.store(8, 0x1, &u64::to_le_bytes(0x1234567812345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(8, 0x1).is_err());

        /* Unaligned access */
        match flash.store(8, 0x2, &u64::to_le_bytes(0x1234567812345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(8, 0x2).is_err());

        /* Unaligned access */
        match flash.store(8, 0x3, &u64::to_le_bytes(0x1234567812345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(8, 0x3).is_err());

        assert!(flash.store(8, 0x4, &u64::to_le_bytes(0x1234567812345678).to_vec()).is_none());
        assert_eq!(u64::from_le_bytes(flash.fetch(8, 0x4).unwrap().try_into().unwrap()), 0x1234567812345678);

        /* Unaligned access */
        match flash.store(8, 0x5, &u64::to_le_bytes(0x1234567812345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(8, 0x5).is_err());

        /* Unaligned access */
        match flash.store(8, 0x6, &u64::to_le_bytes(0x1234567812345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(8, 0x6).is_err());

        /* Unaligned access */
        match flash.store(8, 0x7, &u64::to_le_bytes(0x1234567812345678).to_vec()) {
            Some(BusException::StoreAddressMisaligned) => (),
            _ => assert!(false),
        }
        assert!(flash.fetch(8, 0x7).is_err());

        assert!(flash.store(8, 0x8, &u64::to_le_bytes(0x1234567812345678).to_vec()).is_none());
        assert_eq!(u64::from_le_bytes(flash.fetch(8, 0x8).unwrap().try_into().unwrap()), 0x1234567812345678);
    }


    #[test]
    fn store_fetch_width_illegal() {
        let mut flash: Flash = Flash::new(16);

        assert!(flash.store(0x0, 0x0, &u64::to_le_bytes(0x1234567812345678).to_vec()).is_some());
        assert!(flash.store(3, 0x0, &u64::to_le_bytes(0x1234567812345678).to_vec()).is_some());
        assert!(flash.store(7, 0x0, &u64::to_le_bytes(0x1234567812345678).to_vec()).is_some());
        assert!(flash.fetch(0x0, 0x0).is_err());
        assert!(flash.fetch(3, 0x0).is_err());
        assert!(flash.fetch(7, 0x0).is_err());
    }
}
