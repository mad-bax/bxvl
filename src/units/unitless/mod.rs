use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{metric::Metric, BaseUnit};


/// 'Empty' units
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitNone {
    /// To describe a `Value` representing a percentage
    Percentage,
}

impl Display for UnitNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret: String = String::new();
        match self {
            Self::Percentage => {
                ret.push_str("%");
            }
        }

        write!(f, "{}", ret)
    }
}

impl<T> BaseUnit<UnitNone, T> for UnitNone {
    fn scale(&self) -> T {
        1.0
    }

    fn base(&self) -> T {
        1.0
    }

    fn convert(&self, other: &UnitNone) -> T {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    fn get_metric(&self) -> Metric {
        Metric::None
    }
}