use std::io::Error as IOError;
use hyper::error::Error as HyperError;
use rustc_serialize::json::DecoderError as JSONDecoderError;
use rustc_serialize::json::EncoderError as JSONEncoderError;


#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    IO(IOError),
    JSONDecode(JSONDecoderError),
    JSONEncode(JSONEncoderError),
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

pub type Result<T> = ::std::result::Result<T, Error>;
