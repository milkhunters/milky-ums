use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum CharsError {
    NoNumeric,
    NoAlphabetic,
    HasWhitespace,
}

#[derive(Debug, Serialize, Clone)]
pub enum ValidationError {
    Invalid,
    InvalidEmpty,
    InvalidChar(CharsError),
    InvalidRange((usize, usize)),
    InvalidRegex(String),
}


pub enum DomainError {
    Access,
    Validation((String, ValidationError))
}

impl DomainError {
    pub fn to_validation(self) -> (String, ValidationError) {
        match self {
            DomainError::Validation(err) => err,
            DomainError::Access => unreachable!("Access error should not be converted to validation error")
        }
    }
}