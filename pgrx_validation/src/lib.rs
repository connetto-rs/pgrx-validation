#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod error;
mod font_awesome_icons;
mod str;
pub use error::Error;
pub use font_awesome_icons::must_be_font_awesome_class;
pub use str::{
    must_be_email, must_be_paragraph, must_not_be_empty, must_not_be_padded,
    must_not_contain_consecutive_whitespace, must_not_contain_control_characters,
};

/// Hooks `cargo pgrx test` calls before starting the test PostgreSQL instance.
#[cfg(all(test, feature = "pgrx"))]
pub mod pg_test {
    /// Prepares the test environment, which needs nothing.
    pub fn setup(_options: Vec<&str>) {}

    /// Returns the extra `postgresql.conf` settings for the tests, which are
    /// none.
    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        Vec::new()
    }
}
