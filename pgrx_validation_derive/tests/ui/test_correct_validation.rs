//! Submodule for testing compile success of the `validation` attribute on
//! functions which return the correct return type.
use pgrx_validation_derive::validation;

#[validation]
/// A placeholder validation.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_be_another_empty(_arg: &str) -> Result<(), crate::Error> {
    Ok(())
}

#[validation]
/// A simple validation function which returns the correct error type should
/// pass.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_be_empty(arg: &str) -> Result<(), crate::Error> {
    must_not_be_another_empty(arg)?;
    Ok(())
}

#[validation]
/// A placeholder validation.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_contain_another_empty(
    _arg: &str,
) -> Result<(), crate::Error> {
    Ok(())
}

#[validation]
/// A placeholder validation.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_contain_another_empty(_arg: &str) -> Result<(), crate::Error> {
    Ok(())
}

#[validation]
/// A simple validation function which returns the correct error type should
/// pass.
///
/// # Errors
///
/// * `Error::EmptyText` if the given argument is empty.
pub fn must_not_contain_empty(arg: &str) -> Result<(), crate::Error> {
    must_not_be_another_empty(arg)?;
    must_contain_another_empty(arg)?;
    must_not_contain_another_empty(arg)?;
    Ok(())
}

/// The error type the validations above return.
#[derive(Debug)]
pub struct Error;

/// The main for this test.
fn main() {}
