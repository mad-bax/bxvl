use std::fmt;

/// V3 defined errors
#[derive(Debug)]
pub enum V3Error {
    /// Declared when a `Unit` conversion cannot be completed as expected
    UnitConversionError,
    /// Declared when a `Value` conversion cannot be completed as expected
    ValueConversionError(&'static str),
    /// Declared when a unit type is not supported 
    UnsupportedUnit(String),
    /// Declared when a metric prefix is not supported
    UnsupportedMetric(String),
    /// Declared when a given string cannot be parsed as expected 
    ParsingError(&'static str),
    /// Declared when there is an error when reducing a `Value`
    UnitReductionError(String),
    /// For any other error case
    UnknownError(&'static str)
}

impl fmt::Display for V3Error {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            V3Error::UnitConversionError => write!(
                f,
                "Unit Conversion Error"
            ),
            V3Error::ValueConversionError(s) => write!(
                f,
                "Value Conversion Error: {}",
                s
            ),
            V3Error::UnsupportedUnit(ref s) => write!(
                f,
                "Unsupported unit: {}",
                s
            ),
            V3Error::UnsupportedMetric(ref s) => write!(
                f,
                "Unsupported metric: {}",
                s
            ),
            V3Error::ParsingError(s) => write!(
                f,
                "Parsing error: {}",
                s
            ),
            V3Error::UnitReductionError(ref s) => write!(
                f, 
                "Unit Reduction error: {}",
                s
            ),
            V3Error::UnknownError(s) => write!(
                f,
                "Unknown Error: {}",
                s
            )
        }
    }
}