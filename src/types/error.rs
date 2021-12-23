use super::response::ResponseError;
use reqwest::Error as ReqwestError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Error {
    ReqwestError(ReqwestError),
    ResponseError(ResponseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::ReqwestError(e) => write!(f, "{:?}", e),
            Error::ResponseError(e) => write!(f, "{:?}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::ResponseError(ref e) => Some(e),
            Error::ReqwestError(ref e) => Some(e),
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Error::ReqwestError(err)
    }
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Error {
        Error::ResponseError(err)
    }
}
