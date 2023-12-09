use std::fmt;

use crate::vsoc::arch::registers::ArchRegister;

#[derive(Debug, Default)]
pub struct RvRegisters {
    width: usize,
    count: usize,
    reg: Vec<ArchRegister>,
}

impl RvRegisters {
    pub fn new(width: usize, count: usize) -> RvRegisters {
        let mut reg: Vec<ArchRegister>;
        let zero32: &[u8; 4] = &[0; 4];
        let zero64: &[u8; 8] = &[0; 8];
        let zero128: &[u8; 16] = &[0; 16];
        let zero: &[u8];

        match width {
            32 => zero = zero32,
            64 => zero = zero64,
            128 => zero = zero128,
            _ => unreachable!(),
        }

        println!("* Creating RISC-V registers");
        reg = Vec::<ArchRegister>::with_capacity(count);
        for i in 0..count {
            reg.push(ArchRegister::new(width, format!("r{}", i), i, zero));
        }

        println!("* Populating RISC-V registers");
        reg[0x00] = ArchRegister::new(width, String::from("zero"), 0, zero);
        reg[0x01] = ArchRegister::new(width, String::from("ra"), 0x01, zero);
        reg[0x02] = ArchRegister::new(width, String::from("sp"), 0x02, zero);
        reg[0x03] = ArchRegister::new(width, String::from("gp"), 0x03, zero);
        reg[0x04] = ArchRegister::new(width, String::from("tp"), 0x04, zero);
        reg[0x05] = ArchRegister::new(width, String::from("t0"), 0x05, zero);
        reg[0x06] = ArchRegister::new(width, String::from("t1"), 0x06, zero);
        reg[0x07] = ArchRegister::new(width, String::from("t2"), 0x07, zero);
        reg[0x08] = ArchRegister::new(width, String::from("s0/fp"), 0x08, zero);
        reg[0x09] = ArchRegister::new(width, String::from("s1"), 0x09, zero);
        reg[0x0a] = ArchRegister::new(width, String::from("a0"), 0x0a, zero);
        reg[0x0b] = ArchRegister::new(width, String::from("a1"), 0x0b, zero);
        reg[0x0c] = ArchRegister::new(width, String::from("a2"), 0x0c, zero);
        reg[0x0d] = ArchRegister::new(width, String::from("a3"), 0x0d, zero);
        reg[0x0e] = ArchRegister::new(width, String::from("a4"), 0x0e, zero);
        reg[0x0f] = ArchRegister::new(width, String::from("a5"), 0x0f, zero);

        if count == 32 {
            reg[0x10] = ArchRegister::new(width, String::from("a6"), 0x10, zero);
            reg[0x11] = ArchRegister::new(width, String::from("a7"), 0x11, zero);
            reg[0x12] = ArchRegister::new(width, String::from("s2"), 0x12, zero);
            reg[0x13] = ArchRegister::new(width, String::from("s3"), 0x13, zero);
            reg[0x14] = ArchRegister::new(width, String::from("s4"), 0x14, zero);
            reg[0x15] = ArchRegister::new(width, String::from("s5"), 0x15, zero);
            reg[0x16] = ArchRegister::new(width, String::from("s6"), 0x16, zero);
            reg[0x17] = ArchRegister::new(width, String::from("s7"), 0x17, zero);
            reg[0x18] = ArchRegister::new(width, String::from("s8"), 0x18, zero);
            reg[0x19] = ArchRegister::new(width, String::from("s9"), 0x19, zero);
            reg[0x1a] = ArchRegister::new(width, String::from("s10"), 0x1a, zero);
            reg[0x1b] = ArchRegister::new(width, String::from("s11"), 0x1b, zero);
            reg[0x1c] = ArchRegister::new(width, String::from("t3"), 0x1c, zero);
            reg[0x1d] = ArchRegister::new(width, String::from("t4"), 0x1d, zero);
            reg[0x1e] = ArchRegister::new(width, String::from("t5"), 0x1e, zero);
            reg[0x1f] = ArchRegister::new(width, String::from("t6"), 0x1f, zero);
        }

        RvRegisters {
            width,
            count,
            reg,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn name(&self, regidx: usize) -> &str {
        self.reg[regidx].name()
    }

    pub fn set(&mut self, regidx: usize, value: &[u8]) {
        self.reg[regidx].set(value);
    }

    pub fn get(&self, regidx: usize) -> Vec<u8> {
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
