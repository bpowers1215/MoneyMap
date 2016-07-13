// src/common/mm_result.rs
//MM Result type

use std::fmt;

pub type MMResult<T> = Result<T, MMError>;

#[derive(Debug)]
pub struct MMError{
    kind:MMErrorKind,
    message: String
}

#[derive(Debug)]
pub enum MMErrorKind{
    Database,
    DAO,
    Validation,
    Other
}

impl MMError{
    pub fn new(msg: String, kind: MMErrorKind) -> MMError{
        MMError{
            kind:kind,
            message:msg
        }
    }
}
impl fmt::Display for MMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
