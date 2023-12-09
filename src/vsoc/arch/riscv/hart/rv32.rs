use std::fmt;

use crate::vsoc::arch::Arch32Interface;
use super::registers::RvRegisters;
use super::csr;

#[derive(Debug, Copy)]
pub struct Rv32 {
    pub reg: RvRegisters,
    pub pc: u32,

    csr: csr::Csr,
}

impl Rv32 {
    pub fn new(arch: String, pc: u32) -> Rv32 {
        let mut extensions: u32 = 0;
        let mut csr: csr::Csr;
        let mut registers: usize = 32;

        if arch.contains("E") {
            extensions |= ext::EXT_E;
            registers = 16;
        } else {
            if arch.contains("I") {
                extensions |= ext::EXT_I;
            }
        }
        if arch.contains("M") {
            extensions |= ext::EXT_M;
        }
        if arch.contains("A") {
            extensions |= ext::EXT_A;
        }
        if arch.contains("F") {
            extensions |= ext::EXT_F;
        }
        if arch.contains("D") {
            extensions |= ext::EXT_D | ext::EXT_F;
        }

        csr = csr::Csr::new(4, arch.contains("S"), arch.contains("H"));
        csr.set(csr::MHARTID, &[0; 4]);
        csr.set(csr::MISA, &extensions.to_le_bytes());

        Rv32 {
            reg: RvRegisters::new(registers),
            pc,
            csr,
        }
    }
}

impl Arch32Interface for Rv32 {
    fn step(&mut self) -> Self {
        todo!();

        self
    }
}

impl fmt::Display for Rv32 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "(pc=0x{:0x}, {})", self.pc, self.reg)
    }
}
