# always-abort

## Goal

Allow library crates to invoke
[`std::panic::always_abort()`](https://doc.rust-lang.org/nightly/std/panic/fn.always_abort.html)
automatically on start (of the compiled binary). And, ensure this at the beginning of execution,
before `main()`, even when the binary may not have yet executed any function from the library crate
(that is, from the library that uses `always-abort`).

This is needed, because a library's `Cargo.toml` containing

```toml
[profile.dev]
panic = "abort"
[profile.release]
panic = "abort"
```

doesn't affect the result binary.

## Motivation

- [Small Cult Following = baby steps > Panics vs cancellation, part
  1](https://smallcultfollowing.com/babysteps/blog/2022/01/27/panics-vs-cancellation-part-1/)
- [Small Cult Following = baby steps > Unwind considered
  harmful?](https://smallcultfollowing.com/babysteps/blog/2024/05/02/unwind-considered-harmful) >
  "virtually every network service I know of ships either with `panic=abort` or without really
  leveraging unwinding to recover, just to take cleanup actions and then exit. This could be done
  with `panic=abort` and exit handlers."

## TARGETS

Supported targets are as per [`ctor`](https://crates.io/crates/ctor).

## Rust version

Currently this requires `nightly`. Please,

1. give thumbs up to [rust-lang/rust#84438](https://github.com/rust-lang/rust/issues/84438), and
2. subscribe to
   [peter-lyons-kehl/always-abort-rs#1](https://github.com/peter-lyons-kehl/always-abort-rs/issues/1).

## How to use

1. In `Cargo.toml` of your library have

```toml
[dependencies]
always-abort = "0.0.2"
```

2. In `lib.rs` of your library have
```rust
#[allow(unused_imports)]
use always_abort;
```

That is required, even though you don't execute/use any item from `always-abort` (which doesn't have
any public items anyway).

## Alternatives

### Binary crates

You don't need to use `always-abort` crate if your crate is a binary. Instead, either call
`std::panic::always_abort()`, or have `panic = "abort"` in the binary's `Cargo.toml` (which you want
anyway, to reduce the binary size).

### Library crates on unsupported targets

On targets not supported by [`ctor`](https://crates.io/crates/ctor), you could have your library
invoke `std::panic::always_abort()` at some kind of initiation.

## CONTRIBUTIONS ARE DUAL LICENSED

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
