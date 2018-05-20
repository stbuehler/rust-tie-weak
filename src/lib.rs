//! Wrap a weak pointer for safe knot tying
//!
//! Can be set later through interior mutability.  This is unsafe in the generic
//! case, but the [`tie!`] macro offers a safe abstraction.
//!
//! [`tie!`]: macro.tie.html

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[macro_use]
mod macros;

#[cfg(feature = "std")]
mod arc_impl;
mod generic;
#[cfg(feature = "std")]
mod rc_impl;
pub mod traits;

pub use generic::TieWeak;

#[cfg(feature = "std")]
#[cfg(test)]
mod test;
