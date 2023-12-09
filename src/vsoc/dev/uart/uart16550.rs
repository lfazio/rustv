use crate::vsoc::bus::BusException;
use crate::vsoc::peripheral::PeripheralInterface;

#[derive(Debug, Default, Clone, Copy)]
pub struct Uart16550 {
    length: usize,
    rbr: u32,
    thr: u32,
    lsr: u32,
}

pub const REG_RBR: usize = 0x1000;
pub const REG_THR: usize = 0x1000;
pub const REG_LSR: usize = 0x1014;
pub const LSR_DR: u32 = 1 << 0;
pub const LSR_OE: u32 = 1 << 1;
pub const LSR_THRE: u32 = 1 << 5;
pub const LSR_TEMT: u32 = 1 << 6;

impl Uart16550 {
    pub fn new(length: usize) -> Uart16550 {
        Uart16550 {
            length,
            rbr: 0,
            thr: 0xff,
            lsr: LSR_THRE | LSR_TEMT,
        }
    }

    pub fn size(&self) -> usize {
        self.length
    }

    fn set(&mut self, reg: usize, value: u32) -> &mut Self {
        match reg {
            REG_THR => self.thr = value,
            REG_LSR => self.lsr = value,
            _ => println!("uart16550: set: reg={}: <invalid>", reg),
        }

        self
    }
}

impl PeripheralInterface for Uart16550 {
    // getc()
    fn fetch(&mut self, width: usize, addr: usize) -> Result<Vec<u8>, BusException> {
        if width != 4 && addr % 4 != 0 {
            return Err(BusException::LoadAddressMisaligned);
        }

        match addr {
            REG_RBR => {
                self.set(REG_LSR, self.lsr & !LSR_DR);
                Ok(u32::to_le_bytes(self.rbr).to_vec())
            },
            REG_LSR => Ok(u32::to_le_bytes(self.rbr).to_vec()),
            _ => Ok(vec![0u8, 0u8, 0u8, 0u8]),
        }
    }

    // putc()
    fn store(&mut self, width: usize, addr: usize, value: &Vec<u8>) -> Option<BusException> {
        if width != 4 && addr % 4 != 0 {
            return Some(BusException::StoreAddressMisaligned);
        }

        match addr {
            REG_THR => {
                if self.lsr & LSR_THRE  == LSR_THRE {
                    self.lsr = self.lsr & !LSR_TEMT;
                    self.thr = u32::from_le_bytes((*value.clone()).try_into().unwrap());
                    self.lsr = self.lsr & !LSR_THRE;
                    print!("{}", (self.thr as u8) as char);
                    self.lsr = self.lsr | (LSR_THRE | LSR_TEMT);
                }
            },
            REG_LSR => self.lsr = (self.lsr & 0xff) as u32,
            _ => return None,
        }

        None
    }
}
