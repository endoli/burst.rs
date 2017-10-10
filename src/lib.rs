// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

//! # Burst
//!
//! Burst is a library supporting decomposing binary code
//! into instructions, while maintaining detailed information
//! about the instructions, their flags, and the operands. The
//! result is a structure rather than textual strings.
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
//! burst = "0.0.2"
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

/// An instruction.
///
/// An instruction represents the full amount of information that
/// we have about the instruction that has been disassembled from
/// the binary opcode data.
pub trait Instruction {
    /// The type of the operation for this instruction.
    type Operation;

    /// The type of the operands for this instruction.
    type Operand;

    /// The operation carried out by this instruction.
    fn operation(&self) -> Self::Operation;

    /// The mnemonic for this instruction.
    fn mnemonic(&self) -> &str;

    /// The operands for this instruction.
    fn operands(&self) -> &[Self::Operand];

    /// How many bytes in the binary opcode data are used by this
    /// instruction.
    ///
    /// This can be used to continue disassembling at the next
    /// instruction. An invalid instruction may have a value of
    /// `0` here.
    fn length(&self) -> usize;
}
