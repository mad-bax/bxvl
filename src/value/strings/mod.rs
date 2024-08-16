/// This module is responsible for the string formatting of a [`Value`]
pub(crate) mod display;

/// This module is responsible for parsing [`String`] or [`&str`] into a [`Value`]
pub(crate) mod parse;

/// This module is responsible for implementing the functions [`.into()`] or [`.from()`] to turn a [`&str`] into a [`Value`]
pub(crate) mod value_from_str;
