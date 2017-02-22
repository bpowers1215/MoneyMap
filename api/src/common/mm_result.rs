// src/common/mm_result.rs

/// MM Result type
/// Expands Result type for error handling in the API

use std::fmt;

pub type MMResult<T> = Result<T, MMError>;

#[derive(Debug)]
pub struct MMError{
    kind:MMErrorKind,
    message: &'static str
}

#[derive(Debug)]
pub enum MMErrorKind{
    Database,
    DAO,
    Model,
    Controller,
    Other
}

impl MMError{
    pub fn new(msg: &'static str, kind: MMErrorKind) -> MMError{
        MMError{
            kind:kind,
            message:msg
        }
    }
    pub fn get_message(self) -> &'static str{
        self.message
    }
}
impl fmt::Display for MMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
