//! Submodule for testing compile failure of the `validation` attribute on
//! functions which return a reference.
use pgrx_validation_derive::validation;

#[validation]
/// A validation returning a reference should fail.
pub fn must_not_be_empty(_arg: &str) -> &'static str {
    ""
}

/// The main for this test.
fn main() {}
