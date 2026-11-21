//! Code to embed the Postgres SQL files into the binary
#![cfg_attr(
    not(pgrx_embed),
    expect(missing_docs, reason = "`pgrx_embed!` emits an undocumented placeholder `main`")
)]

::pgrx::pgrx_embed!();
