use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitNone};

impl Display for UnitNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Percentage => "%",
            }
        )
    }
}

impl Into<String> for UnitNone {
    fn into(self) -> String {
        self.to_string()
    }
}

impl<T> Convert<T> for UnitNone
where
    T: BaseUnit,
{
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
