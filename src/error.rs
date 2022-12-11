use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Timeout {
    msg: String,
}

impl Timeout {
    pub fn new(msg: String) -> Timeout {
        Timeout{ msg }
    }
}

impl Error for Timeout {}

impl Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "TimeoutError: {}", self.msg)
    }
}
