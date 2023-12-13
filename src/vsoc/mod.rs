mod arch;
mod bus;
mod dev;
mod peripheral;

use bus::Bus;
use dev::{flash, sram, uart};
use std::fmt;

pub enum VsocException {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    InstructionIllegal,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallUMode,
    EnvironmentCallSMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
}

impl fmt::Display for VsocException {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match *self {
            Self::InstructionAddressMisaligned => String::from("InstructionAddressMisaligned"),
            Self::InstructionAccessFault => String::from("InstructionAccessFault"),
            Self::InstructionIllegal => String::from("InstructionIllegal"),
            Self::Breakpoint => String::from("Breakpoint"),
            Self::LoadAddressMisaligned => String::from("LoadAddressMisaligned"),
            Self::LoadAccessFault => String::from("LoadAccessFault"),
            Self::StoreAddressMisaligned => String::from("StoreAddressMisaligned"),
            Self::StoreAccessFault => String::from("StoreAccessFault"),
            Self::EnvironmentCallUMode => String::from("EnvironmentCallUMode"),
            Self::EnvironmentCallSMode => String::from("EnvironmentCallSMode"),
            Self::InstructionPageFault => String::from("InstructionPageFault"),
            Self::LoadPageFault => String::from("LoadPageFault"),
            Self::StorePageFault => String::from("StorePageFault"),
        };
        write!(f, "VsocException::{}", s)
    }
}

#[derive(Debug)]
pub struct Vsoc<'a> {
    cpu: arch::cpu::Cpu<'a>,
    bus: Bus,
}

impl<'a> Vsoc<'a> {
    pub fn new(arch: &'a String) -> Vsoc<'a> {
        let mut bus: Bus = Bus::new();
        let flash: Box<flash::Flash> = Box::new(flash::Flash::new(128 * 1024));
        let sram: Box<sram::Sram> = Box::new(sram::Sram::new(128 * 1024));
        let uart: Box<uart::uart16550::Uart16550> =
            Box::new(uart::uart16550::Uart16550::new(0x2000));
        let p_flash = Box::new(peripheral::Peripheral::new(
            String::from("flash"),
            flash.size(),
            flash,
        ));
        let p_sram = Box::new(peripheral::Peripheral::new(
            String::from("sram"),
            sram.size(),
            sram,
        ));
        let p_uart = Box::new(peripheral::Peripheral::new(
            String::from("uart"),
            uart.size(),
            uart,
        ));

        bus.attach(0x2000_0000, p_flash);
        bus.attach(0x8000_0000, p_sram);
        bus.attach(0x4001_3c00, p_uart);
        Vsoc {
            cpu: arch::cpu::Cpu::new(arch, 0x8000_0000),
            bus,
        }
    }

    pub fn load(&mut self, binary: &Vec<u8>) {
        for i in 0..binary.len() {
            self.bus
                .store(1, 0x8000_0000 + i as u64, &[binary[i]].to_vec());
        }
    }

    pub fn step(&mut self) -> Option<VsocException> {
        self.cpu.step(&mut self.bus)
    }
}

impl<'a> fmt::Display for Vsoc<'a> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "(vsoc\n {}\n {})", self.cpu, self.bus)
    }
}
