use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{metric::Metric, BaseUnit, Convert};


/// 'Empty' units
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitNone {
    /// To describe a `Value` representing a percentage
    Percentage,

    /// Literally just a number
    None,
}

impl Display for UnitNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::None => "",
            Self::Percentage => "%"
        })
    }
}

impl Into<String> for UnitNone {
    fn into(self) -> String {
        self.to_string()
    }
}

impl<T> Convert<T> for UnitNone where T:BaseUnit {
    fn convert(&self, other: &T) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitNone {
    fn scale(&self) -> f64 {
        1.0
    }

    fn base(&self) -> f64 {
        1.0
    }

    fn get_metric(&self) -> Metric {
        Metric::None
    }
}