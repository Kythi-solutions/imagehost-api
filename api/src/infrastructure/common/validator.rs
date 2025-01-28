use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum UsernameError {
    InvalidLength,
    InvalidFormat,
}

impl UsernameError {
    pub fn as_string(&self) -> String {
        match self {
            UsernameError::InvalidLength => "Invalid Length".to_string(),
            UsernameError::InvalidFormat => "Invalid Format".to_string(),
        }
    }
}

pub fn validate_username(username: String) -> Result<bool, UsernameError> {
    if !username.chars().all(char::is_alphanumeric) {
        return Err(UsernameError::InvalidFormat);
    };

    if 3 > username.len() || 32 < username.len() {
        return Err(UsernameError::InvalidLength);
    }

    return Ok(true);
}

pub fn validate_email(email: String) -> Result<bool, email_address::Error> {
    match email_address::EmailAddress::from_str(&email) {
        Ok(_) => return Ok(true),
        Err(err) => return Err(err),
    }
}
