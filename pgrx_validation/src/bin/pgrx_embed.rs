//! Code to embed the Postgres SQL files into the binary
#![cfg_attr(
    all(feature = "pgrx", not(pgrx_embed)),
    expect(missing_docs, reason = "`pgrx_embed!` emits an undocumented placeholder `main`")
)]

#[cfg(feature = "pgrx")]
::pgrx::pgrx_embed!();

#[cfg(not(feature = "pgrx"))]
/// Stub function to allow compilation without pgrx
fn main() {}
