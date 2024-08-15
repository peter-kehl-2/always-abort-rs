//! Empty. The only intended purpose of this library is to attempt to "apply" the following
//! `profile` settings in `Cargo.toml` to a binary where it's being used - and in
//! demo-lib-cargo-toml-doesnt-apply/bin we show that these `Cargo.toml` settings from library
//! crates don't apply.
//! 
//! ```
//! [profile.dev]
//! panic = "abort"
//!
//! [profile.release]
//! panic = "abort"
//! ```
