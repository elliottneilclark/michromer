use std::io::Error as IOError;
use std::error::Error as StdError;
use hyper::error::Error as HyperError;
use serde_json::Error as SerdeJsonError;
use std::fmt;


#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    IO(IOError),
    JSON(SerdeJsonError),
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Error {
        Error::IO(e)
    }
}
impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error::Hyper(e)
    }
}
impl From<SerdeJsonError> for Error {
    fn from(e: SerdeJsonError) -> Error {
        Error::JSON(e)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Michromer Error: {}", self.description())
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref e) => e.description(),
            Error::IO(ref e) => e.description(),
            Error::JSON(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Hyper(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::JSON(ref e) => Some(e),
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
