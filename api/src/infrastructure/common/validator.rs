use std::str::FromStr;

use derive_more::derive::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum UsernameError {
    InvalidLength,
    InvalidFormat,
}

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum PasswordError {
    InvalidLength,
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

pub fn validate_password(password: String) -> Result<bool, PasswordError> {
    if 5 > password.len() || 64 < password.len() {
        return Err(PasswordError::InvalidLength);
    }

    Ok(true)
}

impl UsernameError {
    pub fn as_string(&self) -> String {
        match self {
            UsernameError::InvalidLength => "Invalid Length".to_string(),
            UsernameError::InvalidFormat => "Invalid Format".to_string(),
        }
    }
}

impl PasswordError {
    pub fn as_string(&self) -> String {
        match self {
            PasswordError::InvalidLength => "Invalid Length".to_string(),
        }
    }
}
