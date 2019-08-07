use std::{
    fmt,
    io,
};

use failure::Fail;


#[derive(Debug)]
pub enum SendgridError {
    Io(io::Error),
    JSONDecode(serde_json::Error),
    HttpError(http::Error),
    HyperError(hyper::Error),
    InvalidHeader(hyper::header::InvalidHeaderValue),
    InvalidFilename,
    UTF8Decode(std::string::FromUtf8Error),
}

impl fmt::Display for SendgridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Fail for SendgridError {}

impl From<http::Error> for SendgridError {
    fn from(error: http::Error) -> Self {
        SendgridError::HttpError(error)
    }
}

impl From<hyper::Error> for SendgridError {
    fn from(error: hyper::Error) -> Self {
        SendgridError::HyperError(error)
    }
}

impl From<hyper::header::InvalidHeaderValue> for SendgridError {
    fn from(error: hyper::header::InvalidHeaderValue) -> Self {
        SendgridError::InvalidHeader(error)
    }
}

impl From<io::Error> for SendgridError {
    fn from(error: io::Error) -> Self {
        SendgridError::Io(error)
    }
}

impl From<serde_json::Error> for SendgridError {
    fn from(error: serde_json::Error) -> Self {
        SendgridError::JSONDecode(error)
    }
}

impl From<std::string::FromUtf8Error> for SendgridError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        SendgridError::UTF8Decode(error)
    }
}

pub type SendgridResult<T> = Result<T, SendgridError>;
