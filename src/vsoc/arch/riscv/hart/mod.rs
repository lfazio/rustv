use std::fmt;

use crate::vsoc::arch::interface::ArchInterface;
use crate::vsoc::arch::types::Uint;
use crate::vsoc::bus::Bus;
use crate::vsoc::bus::BusException;
use crate::vsoc::arch::riscv::csr;
use crate::vsoc::arch::riscv::ext;
use super::exception::RvException;
use super::instr::Instr;
use super::registers::RvRegisters;

#[derive(Debug)]
pub struct Rv {
    width: usize,
    pub reg: RvRegisters,
    pub pc: Uint,
    
    pub csr: csr::Csr,
}

impl Rv {
    pub fn new(width: usize, arch: &str, pc: &Uint) -> Rv {
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
            32 => csr.set(csr::MISA, &Uint::from(extensions)),
            64 => csr.set(csr::MISA, &Uint::from(extensions as u64)),
            128 => csr.set(csr::MISA, &Uint::from(extensions as u128)),
            _ => unreachable!(),
        }
        
        Rv {
            width,
            reg: RvRegisters::new(width, registers),
            pc: pc.clone(),
            csr,
        }
    }
}

impl ArchInterface for Rv {
    fn step(&mut self, bus: &mut Bus) -> Option<RvException> {
        let pc = match self.width {
            32 => u32::from(self.pc.clone()) as u64,
            64 => u64::from(self.pc.clone()),
            128 => u128::from(self.pc.clone()) as u64,
            _ => unreachable!(),
        };
        match bus.fetch(4, pc as u64) {
            Ok(instr) => {
                let raw: u32 = u32::from_le_bytes(instr.try_into().unwrap());
                match Instr::new(raw).process(self, bus) {
                    Ok(offset) => match self.width {
                        32 => self.pc = Uint::from(i32::from(self.pc.clone()) + offset as i32),
                        64 => self.pc = Uint::from(i64::from(self.pc.clone()) + offset as i64),
                        128 => self.pc = Uint::from(i128::from(self.pc.clone()) + offset),
                        _ => unreachable!(),
                    },
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
                    32 => self.reg.set(0, &Uint::from(0u32)),
                    64 => self.reg.set(0, &Uint::from(0u64)),
                    128 => self.reg.set(0, &Uint::from(0u128)),
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

impl fmt::Display for Rv {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "(Rv\n    (pc {})\n    {}   )\n", self.pc, self.reg)
    }
}
