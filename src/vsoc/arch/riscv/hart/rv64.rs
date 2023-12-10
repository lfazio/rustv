use std::fmt;

use crate::vsoc::arch::interface::ArchInterface;
use crate::vsoc::arch::types::Uint;
use crate::vsoc::bus::Bus;
use crate::vsoc::bus::BusException;
use super::super::csr;
use super::super::ext;
use super::super::exception::RvException;
use super::super::instr::Instr;
use super::super::registers::RvRegisters;

#[derive(Debug)]
pub struct Rv64 {
    width: usize,
    pub reg: RvRegisters,
    pub pc: Uint,
    
    pub csr: csr::Csr,
}

impl Rv64 {
    pub fn new(width: usize, arch: &str, pc: u64) -> Rv64 {
        let mut extensions: u32 = 0;
        let mut csr: csr::Csr;
        let mut registers: usize = 32;
        
        if arch.contains('I') { 
            println!("Extension: I");
            extensions |= ext::EXT_I;
        } else if arch.contains('E') {
            println!("Extension: E");
            extensions |= ext::EXT_E;
            registers = 16;
        }
        if arch.contains('M') {
            println!("Extension: M");
            extensions |= ext::EXT_M;
        }
        if arch.contains('A') {
            println!("Extension: A");
            extensions |= ext::EXT_A;
        }
        if arch.contains('F') {
            println!("Extension: F");
            extensions |= ext::EXT_F;
        }
        if arch.contains('D') {
            println!("Extension: D");
            extensions |= ext::EXT_D | ext::EXT_F;
        }
        
        csr = csr::Csr::new(width, arch.contains('S'), arch.contains('H'));
        match width {
            32 => csr.set(csr::MISA, &Uint::from(extensions as u32)),
            64 => csr.set(csr::MISA, &Uint::from(extensions as u64)),
            128 => csr.set(csr::MISA, &Uint::from(extensions as u128)),
            _ => unreachable!(),
        }
        
        Rv64 {
            width,
            reg: RvRegisters::new(64, registers),
            pc: match width {
                32 => Uint::from(pc as u32),
                64 => Uint::from(pc as u64),
                128 => Uint::from(pc as u128),
                _ => unreachable!(),
            },
            csr,
        }
    }
}

impl ArchInterface for Rv64 {
    fn step(&mut self, bus: &mut Bus) -> Option<RvException> {
        // fetch next instruction
        match bus.fetch(4, u64::from(self.pc.clone())) {
            Ok(instr) => {
                let raw: u32 = u32::from_le_bytes(instr.try_into().unwrap());
                match Instr::new(raw).process(self, bus) {
                    Ok(offset) => self.pc = Uint::from(i64::from(self.pc.clone()) + offset as i64),
                    Err(e) => match e {
                        RvException::Breakpoint => {
                            println!("(ebreak @{})", self.pc);
                        },
                        RvException::EnvironmentCallSMode
                        | RvException::EnvironmentCallUMode => {
                            println!("(ecall @{})", self.pc);
                        },
                        _ => {
                            println!("(invalid (instruction {:#08x}))", &raw);
                            return Some(e);
                        },
                    },
                }

                // Reset register $zero to 0
                match self.width {
                    32 => self.reg.set(0, &Uint::from(0 as u32)),
                    64 => self.reg.set(0, &Uint::from(0 as u64)),
                    128 => self.reg.set(0, &Uint::from(0 as u128)),
                    _ => unreachable!(),
                }
            },
            Err(e) => {
                println!("<invalid>");
                match e {
                    BusException::LoadAccessFault =>
                    return Some(RvException::InstructionAccessFault),
                    BusException::LoadAddressMisaligned =>
                    return Some(RvException::InstructionAddressMisaligned),
                    _ => unreachable!(),
                }
            },
        }
        
        None
    }
}

impl fmt::Display for Rv64 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "(rv64\n    (pc {})\n    {}   )\n", self.pc, self.reg)
    }
}
