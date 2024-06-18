use std::fs;
use std::io;
use std::process::Stdio;

#[derive(Debug)]
pub enum Input {
    File(fs::File),
    Stdin(io::Stdin),
}

impl Default for Input {
    fn default() -> Self {
        Self::Stdin(io::stdin())
    }
}

impl From<io::Stdin> for Input {
    fn from(input: io::Stdin) -> Self {
        Self::Stdin(input)
    }
}

impl From<fs::File> for Input {
    fn from(input: fs::File) -> Self {
        Self::File(input)
    }
}

impl From<Input> for Stdio {
    fn from(input: Input) -> Stdio {
        match input {
            Input::File(file) => file.into(),
            Input::Stdin(_) => Stdio::inherit(),
        }
    }
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Self::File(ref mut file) => file.read(buf),
            Self::Stdin(ref mut stream) => stream.read(buf),
        }
    }
}

impl Clone for Input {
    fn clone(&self) -> Self {
        match *self {
            Self::File(ref file) => Self::File(file.try_clone().unwrap()),
            Self::Stdin(_) => Self::Stdin(io::stdin()),
        }
    }
}