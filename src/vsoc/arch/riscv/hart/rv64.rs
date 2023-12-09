use std::fmt;

use crate::vsoc::arch::interface::ArchInterface;
use crate::vsoc::bus::Bus;
use crate::vsoc::bus::BusException;
use super::super::csr;
use super::super::ext;
use super::super::exception::RvException;
use super::super::instr::Instr;
use super::super::registers::RvRegisters;

#[derive(Debug)]
pub struct Rv64 {
    pub reg: RvRegisters,
    pub pc: u64,
    
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
        
        csr = csr::Csr::new(64, arch.contains('S'), arch.contains('H'));
        match width {
            32 => match csr.set(csr::MISA, &extensions.to_le_bytes().to_vec()) {
                Some(_) => println!("csr::MISA = {:x}", extensions),
                None => panic!("Can't set csr::MISA"),
            },
            64 => match csr.set(csr::MISA, &(extensions as u64).to_le_bytes().to_vec()) {
                Some(_) => println!("csr::MISA = {:x}", extensions),
                None => panic!("Can't set csr::MISA"),
            },
            128 => match csr.set(csr::MISA, &(extensions as u128).to_le_bytes().to_vec())  {
                Some(_) => println!("csr::MISA = {:x}", extensions),
                None => panic!("Can't set csr::MISA"),
            },
            _ => unreachable!(),
        }
        
        Rv64 {
            reg: RvRegisters::new(64, registers),
            pc,
            csr,
        }
    }
}

impl ArchInterface for Rv64 {
    fn step(&mut self, bus: &mut Bus) -> Option<RvException> {
        // fetch next instruction
        match bus.fetch(4, self.pc) {
            Ok(instr) => {
                let raw: u32 = u32::from_le_bytes(instr.try_into().unwrap());
                match Instr::new(raw).process(self, bus) {
                    Ok(offset) => self.pc = (self.pc as i64 + offset as i64) as u64,
                    Err(e) => match e {
                        RvException::Breakpoint => {
                            println!("(ebreak @{:#0x})", self.pc);
                        },
                        RvException::EnvironmentCallSMode
                        | RvException::EnvironmentCallUMode => {
                            println!("(ecall @{:#0x})", self.pc);
                        },
                        _ => {
                            println!("(invalid (instruction {:#08x}))", &raw);
                            return Some(e);
                        },
                    },
                }

                self.reg.set(0, &[0; 8]);
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
        write!(f, "(rv64\n    (pc {:#0x})\n    {}   )\n", self.pc, self.reg)
    }
}
