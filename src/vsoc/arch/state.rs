use std::fmt;

#[derive(Debug, Clone,  Copy, PartialEq)]
pub enum State {
    Unknown,
    Initialised,
    Loaded,
    Running,
    Halted,
    Shutdown,
}

impl fmt::Display for State {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            State::Unknown => write!(f, "{}", "Unknown"),
            State::Initialised => write!(f, "{}", "Initialised"),
            State::Loaded => write!(f, "{}", "Loaded"),
            State::Running => write!(f, "{}", "Runnning"),
            State::Halted => write!(f, "{}", "Halted"),
            State::Shutdown => write!(f, "{}", "Shutdown"),
        }
    }
}
