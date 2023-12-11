use std::fmt;

use super::state::State;
use crate::vsoc::{arch::interface::ArchInterface, bus::Bus, VsocException};
//use super::riscv::hart::rv32::Rv32;
use super::riscv::hart::Rv;
use super::types::Uint;

#[derive(Debug)]
enum CpuCore {
    CoreRv(Rv),
}

#[derive(Debug)]
pub struct Cpu<'a> {
    desc: &'a String,
    state: State,
    core: CpuCore,
}

impl<'a> fmt::Display for Cpu<'a> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "(cpu\n  (desc {})\n  (state {})\n  (core\n   {}  )\n )",
            self.desc, self.state, self.core
        )
    }
}

impl fmt::Display for CpuCore {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            CpuCore::CoreRv(core) => write!(f, "{}", core),
        }
    }
}

impl<'a> Cpu<'a> {
    pub fn new(desc: &'a String, pc: u128) -> Cpu<'a> {
        let core: CpuCore;

        if desc.starts_with("RV") {
            if desc.starts_with("RV32") {
                core = CpuCore::CoreRv(Rv::new(32, desc, &Uint::from(pc as u32)));
            } else if desc.starts_with("RV64") {
                core = CpuCore::CoreRv(Rv::new(64, desc, &Uint::from(pc as u64)));
            } else if desc.starts_with("RV128") {
                core = CpuCore::CoreRv(Rv::new(128, desc, &Uint::from(pc)));
            } else {
                panic!("Unsupported: {}", desc);
            }
        } else {
            panic!("Unsupported: {}", desc);
        }

        Cpu {
            desc,
            state: State::Initialised,
            core,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) -> Option<VsocException> {
        match &mut self.core {
            CpuCore::CoreRv(core) => core.step(bus).map(VsocException::from),
        }
    }

    pub fn halt(&mut self) -> &mut Self {
        println!("vemu: halt...");
        self.state = State::Halted;

        self
    }

    fn get_state(&self) -> State {
        self.state
    }

    fn set_state(&mut self, s: State) {
        self.state = s;
    }

    pub fn run<'l: 'a>(&'a mut self, bus: &'l mut Bus) {
        println!("vemu: run...");

        self.set_state(State::Running);

        loop {
            match self.get_state() {
                State::Running => self.step(bus),
                State::Shutdown | State::Halted => return,
                _ => unreachable!(),
            };
        }
    }

    pub fn r#continue(&mut self) -> &mut Self {
        println!("vemu: continuing...");
        self.state = State::Running;

        self
    }

    pub fn shutdown(&mut self) -> &mut Self {
        println!("vemu: shutdown...");
        self.state = State::Shutdown;

        self
    }
}
