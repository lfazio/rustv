use std::fmt;

use crate::vsoc::VsocException;

#[derive(Debug)]
pub enum RvException {
    InstructionAddressMisaligned = 0x00,
    InstructionAccessFault = 0x01,
    InstructionIllegal = 0x02,
    Breakpoint = 0x03,
    LoadAddressMisaligned = 0x04,
    LoadAccessFault = 0x05,
    StoreAddressMisaligned = 0x06,
    StoreAccessFault = 0x07,
    EnvironmentCallUMode = 0x08,
    EnvironmentCallSMode = 0x09,
    InstructionPageFault = 0x0c,
    LoadPageFault = 0x0d,
    StorePageFault = 0x0f,
}

impl fmt::Display for RvException {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String;
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match *self {
            Self::InstructionAddressMisaligned => s = String::from("InstructionAddressMisaligned"),
            Self::InstructionAccessFault => s = String::from("InstructionAccessFault"),
            Self::InstructionIllegal => s = String::from("InstructionIllegal"),
            Self::Breakpoint => s = String::from("Breakpoint"),
            Self::LoadAddressMisaligned => s = String::from("LoadAddressMisaligned"),
            Self::LoadAccessFault => s = String::from("LoadAccessFault"),
            Self::StoreAddressMisaligned => s = String::from("StoreAddressMisaligned"),
            Self::StoreAccessFault => s = String::from("StoreAccessFault"),
            Self::EnvironmentCallUMode => s = String::from("EnvironmentCallUMode"),
            Self::EnvironmentCallSMode => s = String::from("EnvironmentCallSMode"),
            Self::InstructionPageFault => s = String::from("InstructionPageFault"),
            Self::LoadPageFault => s = String::from("LoadPageFault"),
            Self::StorePageFault => s = String::from("StorePageFault"),
        }
        write!(f, "RvException::{}", s)
    }
}

impl RvException {
    pub fn convert(&self) -> VsocException {
	match self {
	    RvException::InstructionAddressMisaligned => VsocException::InstructionAddressMisaligned,
	    RvException::InstructionAccessFault => VsocException::InstructionAccessFault,
	    RvException::InstructionIllegal => VsocException::InstructionIllegal,
	    RvException::Breakpoint => VsocException::Breakpoint,
	    RvException::LoadAddressMisaligned => VsocException::LoadAddressMisaligned,
	    RvException::LoadAccessFault => VsocException::LoadAccessFault,
	    RvException::StoreAddressMisaligned => VsocException::StoreAddressMisaligned,
	    RvException::StoreAccessFault => VsocException::StoreAccessFault,
	    RvException::EnvironmentCallUMode => VsocException::EnvironmentCallUMode,
	    RvException::EnvironmentCallSMode => VsocException::EnvironmentCallSMode,
	    RvException::InstructionPageFault => VsocException::InstructionPageFault,
	    RvException::LoadPageFault => VsocException::LoadPageFault,
	    RvException::StorePageFault => VsocException::StorePageFault,
	}
    }
}
