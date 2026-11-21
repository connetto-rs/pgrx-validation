//! Submodule for string validation

use pgrx_validation_derive::validation;
use validator::ValidateEmail;

#[validation]
/// Validates that the given value is a valid mail address.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * [`crate::Error::Empty`] if the string is empty.
/// * [`crate::Error::InvalidEmail`] if the string is not a valid mail address.
pub fn must_be_email(value: &str) -> Result<(), crate::Error> {
    must_not_be_empty(value)?;
    if value.validate_email() { Ok(()) } else { Err(crate::Error::InvalidEmail) }
}

#[validation]
/// Validates that the given value is not empty.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * [`crate::Error::Empty`] if the string is empty.
pub fn must_not_be_empty(value: &str) -> Result<(), crate::Error> {
    if value.is_empty() { Err(crate::Error::Empty) } else { Ok(()) }
}

#[validation]
/// Validates that a string has no leading or trailing whitespace.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * [`crate::Error::Padded`] if the string has leading or trailing whitespace.
pub fn must_not_be_padded(value: &str) -> Result<(), crate::Error> {
    if value.trim() == value { Ok(()) } else { Err(crate::Error::Padded) }
}

#[validation]
/// Validates that a string does not contain two or more consecutive spaces.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * [`crate::Error::ConsecutiveWhitespace`] if the string contains two or more
///   consecutive spaces.
pub fn must_not_contain_consecutive_whitespace(value: &str) -> Result<(), crate::Error> {
    if value.contains("  ") { Err(crate::Error::ConsecutiveWhitespace) } else { Ok(()) }
}

#[validation]
/// Validates that a string does not contain any control characters.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * [`crate::Error::ControlCharacters`] if the string contains control
///   characters.
pub fn must_not_contain_control_characters(value: &str) -> Result<(), crate::Error> {
    if value.chars().any(char::is_control) { Err(crate::Error::ControlCharacters) } else { Ok(()) }
}

#[validation]
/// Validates that a string is a valid paragraph, which is non-empty, not
/// padded, and free of consecutive spaces and control characters.
///
/// # Arguments
///
/// * `value` a string
///
/// # Errors
///
/// * The error of the first failing check among [`must_not_be_empty`],
///   [`must_not_be_padded`], [`must_not_contain_consecutive_whitespace`] and
///   [`must_not_contain_control_characters`].
pub fn must_be_paragraph(value: &str) -> Result<(), crate::Error> {
    must_not_be_empty(value)?;
    must_not_be_padded(value)?;
    must_not_contain_consecutive_whitespace(value)?;
    must_not_contain_control_characters(value)?;
    Ok(())
}

#[cfg(feature = "pgrx")]
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_must_be_paragraph_accepts_paragraph() {
        let accepted = Spi::get_one::<bool>("SELECT must_be_paragraph('marco visani')").unwrap();
        assert_eq!(accepted, Some(true));
    }

    #[pg_test(error = "Validation failed: must not contain consecutive whitespace")]
    fn test_must_be_paragraph_rejects_double_space() {
        Spi::run("SELECT must_be_paragraph('marco  visani')").unwrap();
    }

    #[pg_test(error = "Validation failed: must be a valid email address")]
    fn test_must_be_email_rejects_in_check_constraint() {
        Spi::run("CREATE TABLE contacts (email TEXT NOT NULL CHECK (must_be_email(email)))")
            .unwrap();
        Spi::run("INSERT INTO contacts (email) VALUES ('marco.visani@unifr.ch')").unwrap();
        Spi::run("INSERT INTO contacts (email) VALUES ('marco')").unwrap();
    }
}

#[cfg(not(feature = "pgrx"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn test_must_be_email() {
        assert_eq!(must_be_email("marco.visani@unifr.ch"), Ok(()));
        assert_eq!(must_be_email("marco"), Err(Error::InvalidEmail));
        assert_eq!(must_be_email(""), Err(Error::Empty));
    }

    #[test]
    fn test_must_not_be_padded() {
        assert_eq!(must_not_be_padded("marco"), Ok(()));
        assert_eq!(must_not_be_padded(" marco"), Err(Error::Padded));
        assert_eq!(must_not_be_padded("marco\t"), Err(Error::Padded));
    }

    #[test]
    fn test_must_not_contain_control_characters() {
        assert_eq!(must_not_contain_control_characters("marco"), Ok(()));
        assert_eq!(must_not_contain_control_characters("marco\n"), Err(Error::ControlCharacters));
    }

    #[test]
    fn test_must_not_contain_consecutive_whitespace() {
        assert_eq!(must_not_contain_consecutive_whitespace("marco visani"), Ok(()));
        assert_eq!(
            must_not_contain_consecutive_whitespace("marco  visani"),
            Err(Error::ConsecutiveWhitespace)
        );
    }

    #[test]
    fn test_must_be_paragraph() {
        assert_eq!(must_be_paragraph("marco visani"), Ok(()));
        assert_eq!(must_be_paragraph(""), Err(Error::Empty));
        assert_eq!(must_be_paragraph(" marco"), Err(Error::Padded));
        assert_eq!(must_be_paragraph("marco  visani"), Err(Error::ConsecutiveWhitespace));
        assert_eq!(must_be_paragraph("marco\nvisani"), Err(Error::ControlCharacters));
    }

    #[test]
    fn test_error_messages() {
        assert_eq!(Error::InvalidEmail.to_string(), "must be a valid email address");
        assert_eq!(
            Error::UnknownFontAwesomeClass("not-an-icon".to_owned()).to_string(),
            "`not-an-icon` is not a Font Awesome class name"
        );
    }
}
