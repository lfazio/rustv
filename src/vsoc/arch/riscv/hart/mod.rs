use std::fmt;

use super::exception::RvException;
use super::instr::Instr;
use super::registers::RvRegisters;
use crate::vsoc::arch::interface::ArchInterface;
use crate::vsoc::arch::riscv::csr;
use crate::vsoc::arch::riscv::ext;
use crate::vsoc::arch::types::Uint;
use crate::vsoc::bus::Bus;
use crate::vsoc::bus::BusException;

#[derive(Debug)]
pub struct Rv {
    width: usize,
    pub reg: RvRegisters,
    pub pc: Uint,

    pub csr: Option<csr::Csr>,

    pub extensions: ext::RvExtensions,
}

impl Rv {
    pub fn new(arch: &str, pc: u128) -> Rv {
        let mut extensions: u32 = 0;
        let mut registers: usize = 32;
        let pcr: Uint;
        let width: usize;
        let mut ext: ext::RvExtensions = ext::RvExtensions::default();
        let argv: Vec<&str> = arch.trim().split('_').collect();

        dbg!(arch, &argv);
        if argv[0].starts_with("rv") {
            if argv[0].starts_with("rv32") {
                width = 32;
                pcr = Uint::from(pc as u32);
            } else if argv[0].starts_with("rv64") {
                width = 64;
                pcr = Uint::from(pc as u64);
            } else if argv[0].starts_with("rv128") {
                width = 128;
                pcr = Uint::from(pc);
            } else {
                panic!("Unsupported architecture width 32/64/128");
            }
        } else {
            panic!("Unsupported architecture: {}", argv[0]);
        }

        if argv[0].contains('i') {
            println!("Extension: i");
            extensions |= ext::EXT_I;
            ext.i = true;
        } else {
            println!("Extension: e");
            extensions |= ext::EXT_E;
            registers = 16;
            ext.e = true;
        }

        if argv[0].contains('m') {
            println!("Extension: m");
            extensions |= ext::EXT_M;
            ext.m = true;

            if arch.contains("zmmul") {
                println!("Extension: zmmul");
                ext.zmmul = true;
            }
        }

        if argv[0].contains('a') {
            println!("Extension: a");
            extensions |= ext::EXT_A;
            ext.a = true;
        }

        if argv[0].contains('h') {
            println!("Extension: h");
            extensions |= ext::EXT_H;
            ext.h = true;
        }

        if argv[0].contains('c') {
            println!("Extension: c");
            extensions |= ext::EXT_C;
            ext.c = true;
        }

        if argv[0].contains('s') {
            println!("Extension: s");
            extensions |= ext::EXT_S;
            ext.s = true;
        }

        if argv[0].contains('u') {
            println!("Extension: u");
            extensions |= ext::EXT_U;
            ext.u = true;
        }

        if arch.contains("zicsr") {
            println!("Extension: zicsr");
            ext.zicsr = true;
        }

        if arch.contains('f') {
            println!("Extension: f");
            extensions |= ext::EXT_F;
            ext.f = true;

            if !ext.zicsr {
                panic!("Missing zicsr extension");
            }

            if arch.contains('d') {
                println!("Extension: d");
                extensions |= ext::EXT_D;
                ext.d = true;
            }
        }

        if arch.contains("zifencei") {
            println!("Extension: zifencei");
            ext.zifencei = true;
        }

        if arch.contains("zicntr") {
            if !ext.zicsr {
                panic!("Missing zicsr extension");
            }

            println!("Extension: zicntr");
            ext.zicntr = true;
        }

        if arch.contains("zihpm") {
            if !ext.zicsr {
                panic!("Missing zicsr extension");
            }

            println!("Extension: zihpm");
            ext.zihpm = true;
        }

        let c = if ext.zicsr {
            let mut csr = csr::Csr::new(width, &ext);
            match width {
                32 => csr.set(csr::MISA, &Uint::from(extensions)),
                64 => csr.set(csr::MISA, &Uint::from(extensions as u64)),
                128 => csr.set(csr::MISA, &Uint::from(extensions as u128)),
                _ => unreachable!(),
            }
            Some(csr)
        } else {
            None
        };

        Rv {
            width,
            reg: RvRegisters::new(width, registers),
            pc: pcr,
            csr: c,
            extensions: ext,
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
                match Instr::from(instr).process(self, bus) {
                    Ok(offset) => match self.width {
                        32 => self.pc = Uint::from(i32::from(self.pc.clone()) + offset as i32),
                        64 => self.pc = Uint::from(i64::from(self.pc.clone()) + offset as i64),
                        128 => self.pc = Uint::from(i128::from(self.pc.clone()) + offset),
                        _ => unreachable!(),
                    },
                    Err(e) => match e {
                        RvException::Breakpoint => {
                            println!("(ebreak @{})", self.pc);
                        }
                        RvException::EnvironmentCallSMode | RvException::EnvironmentCallUMode => {
                            println!("(ecall @{})", self.pc);
                        }
                        _ => {
                            println!("(invalid instruction)");
                            return Some(e);
                        }
                    },
                }

                // Reset register $zero to 0
                self.reg.set(0, &Uint::zero(self.width));
            }
            Err(e) => {
                println!("<invalid>");
                match e {
                    BusException::LoadAccessFault => {
                        return Some(RvException::InstructionAccessFault)
                    }
                    BusException::LoadAddressMisaligned => {
                        return Some(RvException::InstructionAddressMisaligned)
                    }
                    _ => unreachable!(),
                }
            }
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
        write!(
            f,
            "(Rv\n    {}\n    (pc {})\n    {}   )\n",
            self.extensions, self.pc, self.reg
        )
    }
}
