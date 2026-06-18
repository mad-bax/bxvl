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

/// This module is responsible for the string formatting of a [`Value`]
pub(crate) mod display;

/// This module is responsible for parsing [`String`] or [`&str`] into a [`Value`]
pub(crate) mod parse;

/// This module is responsible for implementing the functions [`.into()`] or [`.from()`] to turn a [`&str`] into a [`Value`]
pub(crate) mod value_from_str;
