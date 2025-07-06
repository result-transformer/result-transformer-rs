//! Procedural macros that implement the various transformer traits.
//!
//! This crate exposes helper macros for reducing the amount of boilerplate
//! required when creating custom transformers.  The macros are grouped into the
//! [`core`] module, which covers the basic traits, and the [`flow`] module for
//! the optional pipeline utilities.

mod core;
mod flow;
