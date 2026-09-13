//! Personal Time Capsule reference implementation -- library target.
//!
//! **Stability note**: this crate's Rust API is an internal implementation
//! detail, not this project's public integration contract. Other tools
//! (including a future GUI) are meant to build on the `ptc` *binary*'s
//! flags, output, and exit codes -- see `cli/README.md`'s "Stability"
//! section. A Rust-native consumer is welcome to depend on this crate
//! directly instead of shelling out, but does so at its own risk with no
//! semver promise, at least for now.

pub mod checksum;
pub mod cipher;
pub mod encrypt;
pub mod import;
pub mod inspect;
pub mod model;
pub mod redact;
pub mod scan;
pub mod seal;
