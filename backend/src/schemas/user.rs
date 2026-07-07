use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut contains_uppercase = false;
    let mut contains_lowercase = false;
    let mut contains_number = false;
    let mut contains_special = false;

    let special_chars = vec!['!', '@', '#', '$', '%', '^', '&', '*'];

    for char in password.chars() {
        if special_chars.contains(&char) {
            contains_special = true;
        } else if char.is_ascii_digit() {
            contains_number = true;
        } else if char.is_ascii_lowercase() {
            contains_lowercase = true
        } else if char.is_ascii_uppercase() {
            contains_uppercase = true
        }

        if contains_lowercase && contains_uppercase && contains_number && contains_special {
            return Ok(());
        }
    }

    return Err(ValidationError::new(
        "password should contain at least one lowercase and uppercase character, one number and one special character",
    ));
}

#[derive(Validate, Deserialize)]
pub struct RegisterUserPayload {
    #[validate(length(min = 1, max = 255))]
    pub full_name: String,
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 64), custom(function = "validate_password"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: i32,
}
