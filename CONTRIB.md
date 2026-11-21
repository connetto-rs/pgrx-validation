# Building and testing the extension

The PostgreSQL side is built with [`pgrx`](https://github.com/pgcentralfoundation/pgrx), which compiles its own PostgreSQL and needs `readline`, `bison` and `flex`. On Ubuntu:

```bash
sudo apt update
sudo apt -y install libreadline-dev bison flex libclang-dev
```

`cargo-pgrx` must be the exact version of the `pgrx` crate in the dependency graph:

```bash
pgrx_version=$(cargo metadata --format-version 1 --features pgrx_validation/pg17 | jq -r '.packages[] | select(.name == "pgrx") | .version')
cargo install cargo-pgrx --version "$pgrx_version" --locked
cargo pgrx init --pg17 download
```

Then run the `#[pg_test]` tests inside a temporary PostgreSQL 17, or package the extension:

```bash
cargo pgrx test pg17 -p pgrx_validation
cargo pgrx package -p pgrx_validation --pg-config ~/.pgrx/17.*/pgrx-install/bin/pg_config
```

The Rust-only tests, including the `trybuild` tests of the derive macro, need none of this and run with `cargo test --workspace`.
