use anyhow::Error as AnyhowError;
use reqwest::Error as ReqwestError;
use std::{error::Error as StdError, fmt, io};

pub enum Error {
    Io(io::Error),
    Tera(tera::Error),
    Anyhow(AnyhowError),
    Join(tokio::task::JoinError),
    Reqwest(ReqwestError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "Io Error : {}", err),
            Error::Tera(err) => write!(f, "Tera Error : {}", err),
            Error::Anyhow(err) => write!(f, "Anyhow Error : {}", err),
            Error::Join(err) => write!(f, "tokio::task::JoinError : {}", err),
            Error::Reqwest(err) => write!(f, "reqwest::Error : {}", err),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "Io Error : {:?}", err),
            Error::Tera(err) => write!(f, "Tera Error : {:?}", err),
            Error::Anyhow(err) => write!(f, "Anyhow Error : {:?}", err),
            Error::Join(err) => write!(f, "tokio::task::JoinError : {:?}", err),
            Error::Reqwest(err) => write!(f, "reqwest::Error : {:?}", err),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Self::Tera(err)
    }
}

impl From<AnyhowError> for Error {
    fn from(err: AnyhowError) -> Self {
        Self::Anyhow(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Join(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Self::Reqwest(err)
    }
}
