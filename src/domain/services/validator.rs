use crate::domain::{
    error::{CharsError, DomainError, ValidationError},
    models::{
        permission::{PERMISSION_DESCRIPTION_LENGTH_RANGE, PERMISSION_TITLE_LENGTH_RANGE},
        role::{ROLE_DESCRIPTION_LENGTH_RANGE, ROLE_TITLE_LENGTH_RANGE},
        service::{SERVICE_DESCRIPTION_LENGTH_RANGE, SERVICE_TITLE_LENGTH_RANGE},
        session::{SessionToken, SESSION_TOKEN_LENGTH},
        user::{
            EMAIL_LENGTH_RANGE, 
            EMAIL_REGEX, 
            FIRSTNAME_LENGTH_RANGE, 
            FIRSTNAME_REGEX, 
            LASTNAME_LENGTH_RANGE, 
            LASTNAME_REGEX, 
            PASSWORD_LENGTH_RANGE, 
            USERNAME_LENGTH_RANGE, 
            USERNAME_REGEX
        }
    }
};


pub fn validate_username(username: &str) -> Result<(), DomainError> {
    if !(USERNAME_LENGTH_RANGE.0 < username.len() && username.len() < USERNAME_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "username".into(),
            ValidationError::InvalidRange(USERNAME_LENGTH_RANGE)
        )));
    }

    if !USERNAME_REGEX.is_match(username) {
        return Err(DomainError::Validation((
            "username".into(),
            ValidationError::InvalidRegex(USERNAME_REGEX.to_string())
        )));
    }
    
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), DomainError> {
    if !(EMAIL_LENGTH_RANGE.0 < email.len() && email.len() < EMAIL_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "email".into(),
            ValidationError::InvalidRange(EMAIL_LENGTH_RANGE)
        )));
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err(DomainError::Validation((
            "email".into(),
            ValidationError::InvalidRegex(EMAIL_REGEX.to_string())
        )));
    }
    
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), DomainError> {
    if !(PASSWORD_LENGTH_RANGE.0 < password.len() && password.len() < PASSWORD_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "password".into(),
            ValidationError::InvalidRange(PASSWORD_LENGTH_RANGE)
        )));
    }

    if !password.chars().any(char::is_numeric) {
        return Err(DomainError::Validation((
            "password".into(), 
            ValidationError::InvalidChar(CharsError::NoNumeric)
        )))
    }

    if !password.chars().any(char::is_alphabetic) {
        return Err(DomainError::Validation((
            "password".into(),
            ValidationError::InvalidChar(CharsError::NoAlphabetic)
        )))
    }

    if password.chars().any(char::is_whitespace) {
        return Err(DomainError::Validation((
            "password".into(),
            ValidationError::InvalidChar(CharsError::HasWhitespace)
        )))
    }

    Ok(())
}

pub fn validate_first_name(first_name: &str) -> Result<(), DomainError> {
    if !(FIRSTNAME_LENGTH_RANGE.0 < first_name.len() && first_name.len() < FIRSTNAME_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "first_name".into(),
            ValidationError::InvalidRange(FIRSTNAME_LENGTH_RANGE)
        )));
    }

    if !FIRSTNAME_REGEX.is_match(first_name) {
        return Err(DomainError::Validation((
            "first_name".into(),
            ValidationError::InvalidRegex(FIRSTNAME_LENGTH_RANGE.to_string())
        )));
    }

    Ok(())
}

pub fn validate_last_name(last_name: &str) -> Result<(), DomainError> {
    if !(LASTNAME_LENGTH_RANGE.0 < last_name.len() && last_name.len() < LASTNAME_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "last_name".into(),
            ValidationError::InvalidRange(LASTNAME_LENGTH_RANGE)
        )));
    }
    
    if !LASTNAME_REGEX.is_match(last_name) {
        return Err(DomainError::Validation((
            "last_name".into(),
            ValidationError::InvalidRegex(LASTNAME_REGEX.to_string())
        )));
    }

    Ok(())
}

pub fn validate_role_title(title: &str) -> Result<(), DomainError> {
    if !(ROLE_TITLE_LENGTH_RANGE.0 < title.len() && title.len() < ROLE_TITLE_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "title".into(),
            ValidationError::InvalidRange(ROLE_TITLE_LENGTH_RANGE)
        )));
    }
    Ok(())
}

pub fn validate_role_description(description: &str) -> Result<(), DomainError> {
    if !(ROLE_DESCRIPTION_LENGTH_RANGE.0 < description.len() && description.len() < ROLE_DESCRIPTION_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "description".into(),
            ValidationError::InvalidRange(ROLE_DESCRIPTION_LENGTH_RANGE)
        )));
    }
    Ok(())
}

pub fn validate_session_token(session_token: &SessionToken) -> Result<(), DomainError> {
    if session_token.len() != SESSION_TOKEN_LENGTH {
        return Err(DomainError::Validation((
            "session_token".into(),
            ValidationError::InvalidRange((SESSION_TOKEN_LENGTH, SESSION_TOKEN_LENGTH))
        )));
    }
    Ok(())
}

pub fn validate_page(page: u32) -> Result<(), DomainError> {
    if !(1 <= page && page <= 1_000_000) {
        return Err(DomainError::Validation(
            ("page".into(), ValidationError::InvalidRange((1, 1_000_000)))
        ));
    }
    Ok(())
}

pub fn validate_per_page(per_page: u8) -> Result<(), DomainError> {
    if !(1 <= per_page && per_page <= 100) {
        return Err(DomainError::Validation(
            ("per_page".into(), ValidationError::InvalidRange((1, 100)))
        ));
    }
    Ok(())
}
pub fn validate_permission_title(title: &str) -> Result<(), DomainError> {
    if !(PERMISSION_TITLE_LENGTH_RANGE.0 < title.len() && title.len() < PERMISSION_TITLE_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "title".into(),
            ValidationError::InvalidRange(PERMISSION_TITLE_LENGTH_RANGE)
        )));
    }
    Ok(())
}

pub fn validate_permission_description(description: &str) -> Result<(), DomainError> {
    if !(PERMISSION_DESCRIPTION_LENGTH_RANGE.0 < description.len() && description.len() < PERMISSION_DESCRIPTION_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "description".into(),
            ValidationError::InvalidRange(PERMISSION_DESCRIPTION_LENGTH_RANGE)
        )));
    }
    Ok(())
}

pub fn validate_service_title(title: &str) -> Result<(), DomainError> {
    if !(SERVICE_TITLE_LENGTH_RANGE.0 < title.len() && title.len() < SERVICE_TITLE_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "title".into(),
            ValidationError::InvalidRange(SERVICE_TITLE_LENGTH_RANGE)
        )));
    }
    Ok(())
}

pub fn validate_service_description(description: &str) -> Result<(), DomainError> {
    if !(SERVICE_DESCRIPTION_LENGTH_RANGE.0 < description.len() && description.len() < SERVICE_DESCRIPTION_LENGTH_RANGE.1) {
        return Err(DomainError::Validation((
            "description".into(),
            ValidationError::InvalidRange(SERVICE_DESCRIPTION_LENGTH_RANGE)
        )));
    }
    Ok(())
}
