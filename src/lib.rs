// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

//! # Burst
//!
//! Burst is a library supporting disassembling binary code
//! into instructions, while maintaining detailed information
//! about the instructions, their flags, and the operands.
//!
//! While Burst currently only supports x86 and x86_64 code,
//! this will change in the near future and we anticipate adding
//! many additional architectures.
//!
//! ## Goals of Burst:
//!
//! * Regular releases without waiting for long periods of time.
//! * Uses fuzz testing to avoid crashes.
//! * Well tested.
//! * Fast. Few allocations and little data copying should be required.
//!
//! ## Installation
//!
//! This crate works with Cargo and is on
//! [crates.io](https://crates.io/crates/burst).
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! burst = "0.0.1"
//! ```
//!
//! Then, let `rustc` know that you're going to use this crate at the
//! top of your own crate:
//!
//! ```
//! extern crate burst;
//! # fn main() {}
//! ```
//!
//! ## Contributions
//!
//! Contributions are welcome.
//!

#![warn(missing_docs)]
#![deny(trivial_numeric_casts, unstable_features,
        unused_import_braces, unused_qualifications)]

pub mod x86;
