use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitPower};

impl Display for UnitPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}W", self.get_metric().as_str())
    }
}

impl Into<String> for UnitPower {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitPower> for UnitPower {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitPower) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitPower {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Watt(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Watt(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
