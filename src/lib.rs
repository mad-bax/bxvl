#![crate_type = "lib"]
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

//!

#[doc = include_str!("../README.md")]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Various *const* definitions that are used throughout [`bxvl`]
pub mod consts;

/// The module for [`bxvl`] specific error definitions
pub mod errors;

/// The main module for [`Metric`] and [`Unit`] types
pub mod units;

/// The module used for [`Value`] implementations
pub mod value;
