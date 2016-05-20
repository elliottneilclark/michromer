use std::error::Error as StdError;
use std::io::Error as IOError;
use hyper::error::Error as HyperError;
use rustc_serialize::json::DecoderError as JSONDecoderError;

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    IO(IOError),
    JSON(JSONDecoderError),
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
        Error::JSON(e)
    }
}
