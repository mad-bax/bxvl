use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitSolidAngle};

impl Display for UnitSolidAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}sr", self.get_metric().as_str())
    }
}

impl Into<String> for UnitSolidAngle {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitSolidAngle> for UnitSolidAngle {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitSolidAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitSolidAngle {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Steradian(m) => m.scale(),
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Steradian(_) => 1.0,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Steradian(m) => *m,
        }
    }
}
