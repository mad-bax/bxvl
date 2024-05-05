use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitLuminousIntensity};

impl Display for UnitLuminousIntensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}cd", self.get_metric().as_str())
    }
}

impl Into<String> for UnitLuminousIntensity {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitLuminousIntensity> for UnitLuminousIntensity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitLuminousIntensity) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitLuminousIntensity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Candela(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Candela(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
