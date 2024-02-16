use std::fmt;

use super::atomic::AtomicCtx;
use super::exception::RvException;
use super::instr::Instr;
use super::registers::RvFpuRegisters;
use super::registers::RvRegisters;
use crate::vsoc::arch::interface::ArchInterface;
use crate::vsoc::arch::riscv::csr;
use crate::vsoc::arch::riscv::ext;
use crate::vsoc::arch::types::Uint;
use crate::vsoc::bus::Bus;
use crate::vsoc::bus::BusException;

#[derive(Debug)]
pub struct Rv {
    xlen: usize,
    flen: usize,

    pub pc: Uint,
    pub x: RvRegisters,
    pub f: Option<RvFpuRegisters>,

    pub csr: Option<csr::Csr>,

    pub extensions: ext::RvExtensions,

    pub atomic_ctx: Option<AtomicCtx>,
}

impl Rv {
    pub fn new(arch: &str) -> Rv {
        let mut extensions: u32 = 0;
        let mut registers: usize = 32;
        let xlen: usize;
        let mut flen: usize = 0;
        let mut ext: ext::RvExtensions = ext::RvExtensions::default();
        let argv: Vec<&str> = arch.trim().split('_').collect();
        let mut atomic_ctx: Option<AtomicCtx> = None;

        if argv[0].starts_with("rv") {
            if argv[0].starts_with("rv32") {
                xlen = 32;
            } else if argv[0].starts_with("rv64") {
                xlen = 64;
            } else if argv[0].starts_with("rv128") {
                xlen = 128;
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
            ext.zalrsc = true;
            ext.zamo = true;
            ext.zacas = true;
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

        if arch.contains("zalrsc") {
            println!("Extension: zalrsc");
            ext.zalrsc = true;
        }

        if arch.contains("zamo") {
            println!("Extension: zamo");
            ext.zamo = true;
        }

        if arch.contains("zacas") {
            println!("Extension: zacas");
            ext.zacas = true;
        }

        if ext.zalrsc || ext.zamo || ext.zacas {
            atomic_ctx = Some(AtomicCtx::new(xlen));
        }

        if arch.contains('f') {
            println!("Extension: f");
            extensions |= ext::EXT_F;
            ext.f = true;
            flen = 32;

            if !ext.zicsr {
                panic!("Missing zicsr extension");
            }

            if arch.contains('d') {
                println!("Extension: d");
                extensions |= ext::EXT_D;
                ext.d = true;
                flen = 64;

                if arch.contains('q') {
                    println!("Extension: q");
                    extensions |= ext::EXT_Q;
                    ext.q = true;
                    flen = 128;
                }
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

        let csr = if ext.zicsr {
            let mut c = csr::Csr::new(xlen, &ext);
            c.set(csr::MISA, Uint::from(extensions).extend(xlen));
            
            Some(c)
        } else {
            None
        };

        Rv {
            xlen,
            flen,
            pc: Uint::zero(xlen),
            x: RvRegisters::new(xlen, registers),
            f: if ext.f {
                Some(RvFpuRegisters::new(flen))
            } else {
                None
            },
            csr,
            extensions: ext,
            atomic_ctx,
        }
    }

    pub fn set_pc(&mut self, addr: u128) {
        self.pc = Uint::from(addr);
        self.pc.truncate(self.xlen);
    }
}

impl ArchInterface for Rv {
    fn step(&mut self, bus: &mut Bus) -> Option<RvException> {
        let pc = match self.xlen {
            32 => u32::from(self.pc.clone()) as u64,
            64 => u64::from(self.pc.clone()),
            128 => u128::from(self.pc.clone()) as u64,
            _ => unreachable!(),
        };
        match bus.fetch(4, pc as u64) {
            Ok(instr) => {
                match Instr::from(instr).process(self, bus) {
                    Ok(offset) => match self.xlen {
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
                self.x.set(0, &Uint::zero(self.xlen));
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
        if self.f.is_none() {
            write!(
                f,
                "(Rv{}\n    {}\n    (pc {})\n    {}   )\n",
                self. xlen, self.extensions, self.pc, self.x
            )
        } else {
            write!(
                f,
                "(Rv{}\n    (flen {})\n    {}\n    (pc {})\n    {}\n    {}   )\n",
                self.xlen, self.flen,
                self.extensions, self.pc, self.x, self.f.as_ref().unwrap()
            )
        }
    }
}
