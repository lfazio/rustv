use std::fmt;

pub enum RvInterrupt {
    SupervisorSwInt = 0x1,
    SupervisorTimerInt = 0x5,
    SupervisorExternalInt = 0x9,
}

impl fmt::Display for RvInterrupt {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String;
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match *self {
            Self::SupervisorSwInt => s = String::from("SupervisorSw"),
            Self::SupervisorTimerInt => s = String::from("SupervisorTimer"),
            Self::SupervisorExternalInt => s = String::from("SupervisorExternal"),
        }
        write!(f, "RvInterrupt::{}", s)
    }
}
