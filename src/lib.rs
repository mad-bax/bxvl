#![crate_type = "lib"]
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

//! The V3 library crate

#[doc = include_str!("../README.md")]

/// Various `const` definitions that are used throughout V3
mod constants;

/// The error code definitions
pub mod errors;

/// The main module for `Metric` and `Unit` types
pub mod units;

/// The main module for V3 and `Value`s
pub mod values;
