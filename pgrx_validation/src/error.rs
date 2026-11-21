//! Submodule defining the error returned by every validation function.

/// Reason a value failed validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The value is empty.
    #[error("must not be empty")]
    Empty,
    /// The value has leading or trailing whitespace.
    #[error("must not be padded")]
    Padded,
    /// The value contains two or more consecutive spaces.
    #[error("must not contain consecutive whitespace")]
    ConsecutiveWhitespace,
    /// The value contains control characters.
    #[error("must not contain control characters")]
    ControlCharacters,
    /// The value is not a valid email address.
    #[error("must be a valid email address")]
    InvalidEmail,
    /// The value is not a Font Awesome class name.
    #[error("`{0}` is not a Font Awesome class name")]
    UnknownFontAwesomeClass(String),
}
