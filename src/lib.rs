/*  BXVL
 *  Copyright (C) 2026 Bax Bradley
 *
 *  This library is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! BXVL Library

//#![feature(coverage_attribute)]

#![crate_type = "lib"]
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

/// The BXVL Library Version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Various *const* definitions that are used throughout [`bxvl`]
pub mod consts;

/// The module for [`bxvl`] specific error definitions
pub mod errors;

/// The main module for [`Metric`] and [`Unit`] types
pub mod units;

/// The module used for [`Value`] implementations
pub mod value;
