use core::fmt;
use std::{error, io};


#[derive(Debug, Clone)]
pub struct ZError{
    kind: String,
    message: String,
}

impl fmt::Display for ZError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid opearte kind:{}, message:{}", self.kind, self.message)
    }
}

impl error::Error for ZError {
    
}

impl From<io::Error> for ZError {
    fn from(value: io::Error) -> Self {
        ZError { kind: "io::Error".to_string(), message: value.to_string() }
    }
}

impl From<toml::de::Error> for ZError {
    fn from(value: toml::de::Error) -> Self {
        ZError { kind: "toml::de::Error".to_string(), message: value.to_string() }
    }
}

impl From<toml::ser::Error> for ZError {
    fn from(value: toml::ser::Error) -> Self {
        ZError { kind: "toml::ser::Error".to_string(), message: value.to_string() }
    }
}