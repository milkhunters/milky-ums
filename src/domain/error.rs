use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum CharsError {
    NoNumeric,
    NoAlphabetic,
    HasWhitespace,
}

#[derive(Debug, Serialize, Clone)]
pub enum ValidationError {
    InvalidChar(CharsError),
    InvalidRange((usize, usize)),
    InvalidEmpty,
    InvalidRegex(String),
}


pub enum DomainError {
    Access,
    Validation((String, ValidationError))
}
