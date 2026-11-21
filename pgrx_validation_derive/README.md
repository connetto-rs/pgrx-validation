# PGRX Validation Derive

Procedural macro that checks validation functions against a fixed set of rules and, with the `pgrx` feature, exposes each one to PostgreSQL as a SQL function of the same name.

## Validation

The macro made available by this crate is `validation`. It leaves the function unchanged, and with the `pgrx` feature it adds a `pg_extern` wrapper that returns `true` or raises the validation error.

The function must follow these rules, or compilation fails.

1. The function must be [public](https://doc.rust-lang.org/reference/visibility-and-privacy.html).
2. The function must not be [async](https://rust-lang.github.io/async-book/), as this is not supported by [`pgrx`](https://github.com/pgcentralfoundation/pgrx).
3. The function must not have [generic parameters](https://doc.rust-lang.org/reference/items/generics.html), as this would not be supported by [PostgreSQL](https://www.postgresql.org/).
4. The function must have at least one argument and must not take `self`.
5. The function must return `Result<(), crate::Error>`.
6. The function name must be in [snake_case](https://en.wikipedia.org/wiki/Snake_case).
7. The function name must start with `must_be_`, `must_not_be_`, `must_contain_` or `must_not_contain_`.
8. The function must have a doc comment.
