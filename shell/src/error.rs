use std::fmt;
#[derive(Debug)]

pub enum ShellError {
    Other(String),
    Io(std::io::Error)
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Other(msg) => write!(f, "{}", msg),
            ShellError::Io(err) => write!(f, "Io error: {}", err),
        }
    }
}

impl std::error::Error for ShellError {}

impl From<std::io::Error> for ShellError {
    fn from(err: std::io::Error) -> Self {
        ShellError::Io(err)
    }
}