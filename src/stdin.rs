use std::{
    error::Error,
    fmt::Display,
    io::{stdin, BufReader, Read},
};

#[derive(Debug)]
pub struct StdinReadError;

impl Error for StdinReadError {}

impl Display for StdinReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to read from stdin. Error: {}", self)
    }
}

/// Read from stdin, and return a standard error if anything goes wrong
pub fn read_from_stdin() -> Result<String, StdinReadError> {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer: Vec<u8> = Vec::new();

    reader
        .read_to_end(&mut buffer)
        .map_err(|_| StdinReadError)?;

    let result = std::str::from_utf8(&buffer).map_err(|_| StdinReadError)?;

    Ok(result.into())
}
