# PGRX Validation

Validation rules defined once and compiled to several targets. Without features, each validation is an ordinary Rust function returning `Result<(), Error>`, usable from a backend or from a frontend compiled to WASM. With the `pgrx` feature and one of `pg13` to `pg17`, the crate builds the `pgrx_validation` PostgreSQL extension, in which each validation is a SQL function returning `true` or raising an error, so a `CHECK` constraint applies the same rule as the application.

```rust
use pgrx_validation::{Error, must_be_email, must_be_paragraph};

assert_eq!(must_be_email("marco.visani@unifr.ch"), Ok(()));
assert_eq!(must_be_email("marco"), Err(Error::InvalidEmail));
assert_eq!(must_be_paragraph("two  spaces"), Err(Error::ConsecutiveWhitespace));
```
