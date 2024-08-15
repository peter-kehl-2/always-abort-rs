# always-abort

## GOAL

Allow library crates to invoke `std::panic::always_abort()` automatically on start (of the compiled
binary). And, ensure this even at the beginning of execution, when the binary may not have yet executed
any function from the library crate (the library that uses `always-abort`).

This is needed, because a library's `Cargo.toml` containing

```toml
[profile.dev]
panic = "abort"
[profile.release]
panic = "abort"
```

doesn't affect the result binary.

## TARGETS

Limited. See [`ctor`](https://crates.io/crates/ctor).

## Rust version

Currently this requires `nightly`. Please,

1. give thumbs up to [rust-lang/rust#84438](https://github.com/rust-lang/rust/issues/84438), and
2. subscribe to
   [peter-lyons-kehl/always-abort-rs#1](https://github.com/peter-lyons-kehl/always-abort-rs/issues/1).

## CONTRIBUTIONS ARE DUAL LICENSED

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
