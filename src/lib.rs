use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::{FromStr, Utf8Error};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    ParseUtf8(Utf8Error),
    General(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::ParseUtf8(e)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_e: std::convert::Infallible) -> Self {
        panic!("Infallible error should never occur")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => f.write_fmt(format_args!("Io: {}", e)),
            Error::ParseInt(e) => f.write_fmt(format_args!("Parse: {}", e)),
            Error::ParseFloat(e) => f.write_fmt(format_args!("Parse: {}", e)),
            Error::ParseUtf8(e) => f.write_fmt(format_args!("Parse: {}", e)),
            Error::General(s) => f.write_fmt(format_args!("General: {}", s)),
        }
    }
}

impl std::error::Error for Error {}

pub fn read_file<E, T>(file: &str) -> Result<Vec<T>>
where
    E: 'static + std::error::Error + Into<Error> + Send + Sync,
    T: FromStr<Err = E>,
{
    let path = Path::new(file);
    let io = File::open(path)?;
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line?.parse::<T>().map_err(|e| e.into()))
        .collect()
}

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub mod prelude {
    pub use super::read_file;
    pub use super::regex;
    pub use super::Error;
    pub use super::Result;

    pub use regex::Regex;
}
