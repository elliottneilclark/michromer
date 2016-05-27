use std::io::Error as IOError;
use std::error::Error as StdError;
use hyper::error::Error as HyperError;
use rustc_serialize::json::DecoderError as JSONDecoderError;
use rustc_serialize::json::EncoderError as JSONEncoderError;
use rustc_serialize::json::ParserError as JSONParseError;
use std::fmt;


#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    IO(IOError),
    JSONDecode(JSONDecoderError),
    JSONEncode(JSONEncoderError),
    JSONParse(JSONParseError),
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
impl From<JSONDecoderError> for Error {
    fn from(e: JSONDecoderError) -> Error {
        Error::JSONDecode(e)
    }
}
impl From<JSONEncoderError> for Error {
    fn from(e: JSONEncoderError) -> Error {
        Error::JSONEncode(e)
    }
}
impl From<JSONParseError> for Error {
    fn from(e: JSONParseError) -> Error {
        Error::JSONParse(e)
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
            Error::JSONDecode(ref e) => e.description(),
            Error::JSONEncode(ref e) => e.description(),
            Error::JSONParse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Hyper(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::JSONDecode(ref e) => Some(e),
            Error::JSONEncode(ref e) => Some(e),
            Error::JSONParse(ref e) => Some(e),

        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
