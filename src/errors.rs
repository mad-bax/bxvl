use std::fmt;

#[derive(Debug)]
pub enum V3Error {
    UnitConversionError,
    ValueConversionError(&'static str),
    UnsupportedUnit(String),
    UnsupportedMetric(String),
    ParsingError(&'static str),
    UnitReductionError(&'static str),
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
            V3Error::UnitReductionError(s) => write!(
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