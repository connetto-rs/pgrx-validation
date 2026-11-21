//! Submodule for testing compile failure of the `validation` attribute on
//! methods taking `self`.
use pgrx_validation_derive::validation;

/// A value with a validation method.
pub struct Value;

impl Value {
    #[validation]
    /// A validation taking `self` should fail.
    pub fn must_not_be_empty(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// The main for this test.
fn main() {}
