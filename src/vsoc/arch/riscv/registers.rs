use std::fmt;

use crate::vsoc::arch::registers::ArchRegister;
use crate::vsoc::arch::types::Uint;

#[derive(Debug, Default)]
pub struct RvRegisters {
    xlen: usize,
    count: usize,
    reg: Vec<ArchRegister>,
}

impl RvRegisters {
    pub fn new(xlen: usize, count: usize) -> RvRegisters {
        let mut reg: Vec<ArchRegister>;

        println!("* Creating RISC-V registers");
        reg = Vec::<ArchRegister>::with_capacity(count);
        for i in 0..count {
            reg.push(ArchRegister::new(format!("x{}", i), i, Uint::zero(xlen)));
        }

        println!("* Populating RISC-V registers");
        reg[0x00] = ArchRegister::new(String::from("zero"), 0, Uint::zero(xlen));
        reg[0x01] = ArchRegister::new(String::from("ra"), 0x01, Uint::zero(xlen));
        reg[0x02] = ArchRegister::new(String::from("sp"), 0x02, Uint::zero(xlen));
        reg[0x03] = ArchRegister::new(String::from("gp"), 0x03, Uint::zero(xlen));
        reg[0x04] = ArchRegister::new(String::from("tp"), 0x04, Uint::zero(xlen));
        reg[0x05] = ArchRegister::new(String::from("t0"), 0x05, Uint::zero(xlen));
        reg[0x06] = ArchRegister::new(String::from("t1"), 0x06, Uint::zero(xlen));
        reg[0x07] = ArchRegister::new(String::from("t2"), 0x07, Uint::zero(xlen));
        reg[0x08] = ArchRegister::new(String::from("s0/fp"), 0x08, Uint::zero(xlen));
        reg[0x09] = ArchRegister::new(String::from("s1"), 0x09, Uint::zero(xlen));
        reg[0x0a] = ArchRegister::new(String::from("a0"), 0x0a, Uint::zero(xlen));
        reg[0x0b] = ArchRegister::new(String::from("a1"), 0x0b, Uint::zero(xlen));
        reg[0x0c] = ArchRegister::new(String::from("a2"), 0x0c, Uint::zero(xlen));
        reg[0x0d] = ArchRegister::new(String::from("a3"), 0x0d, Uint::zero(xlen));
        reg[0x0e] = ArchRegister::new(String::from("a4"), 0x0e, Uint::zero(xlen));
        reg[0x0f] = ArchRegister::new(String::from("a5"), 0x0f, Uint::zero(xlen));

        if count == 32 {
            reg[0x10] = ArchRegister::new(String::from("a6"), 0x10, Uint::zero(xlen));
            reg[0x11] = ArchRegister::new(String::from("a7"), 0x11, Uint::zero(xlen));
            reg[0x12] = ArchRegister::new(String::from("s2"), 0x12, Uint::zero(xlen));
            reg[0x13] = ArchRegister::new(String::from("s3"), 0x13, Uint::zero(xlen));
            reg[0x14] = ArchRegister::new(String::from("s4"), 0x14, Uint::zero(xlen));
            reg[0x15] = ArchRegister::new(String::from("s5"), 0x15, Uint::zero(xlen));
            reg[0x16] = ArchRegister::new(String::from("s6"), 0x16, Uint::zero(xlen));
            reg[0x17] = ArchRegister::new(String::from("s7"), 0x17, Uint::zero(xlen));
            reg[0x18] = ArchRegister::new(String::from("s8"), 0x18, Uint::zero(xlen));
            reg[0x19] = ArchRegister::new(String::from("s9"), 0x19, Uint::zero(xlen));
            reg[0x1a] = ArchRegister::new(String::from("s10"), 0x1a, Uint::zero(xlen));
            reg[0x1b] = ArchRegister::new(String::from("s11"), 0x1b, Uint::zero(xlen));
            reg[0x1c] = ArchRegister::new(String::from("t3"), 0x1c, Uint::zero(xlen));
            reg[0x1d] = ArchRegister::new(String::from("t4"), 0x1d, Uint::zero(xlen));
            reg[0x1e] = ArchRegister::new(String::from("t5"), 0x1e, Uint::zero(xlen));
            reg[0x1f] = ArchRegister::new(String::from("t6"), 0x1f, Uint::zero(xlen));
        }

        RvRegisters { xlen, count, reg }
    }

    pub fn len(&self) -> usize {
        self.xlen
    }

    pub fn name(&self, regidx: usize) -> &str {
        self.reg[regidx].name()
    }

    pub fn set(&mut self, regidx: usize, value: &Uint) {
        self.reg[regidx].set(value);
    }

    pub fn get(&self, regidx: usize) -> Uint {
        self.reg[regidx].get()
    }
}

impl fmt::Display for RvRegisters {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match writeln!(f, "(registers\n     (#count {})\n     (", self.count) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        for reg in self.reg.iter() {
            match writeln!(f, "      {}", reg) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        writeln!(f, "     )\n    )")
    }
}
