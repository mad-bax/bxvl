use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitLuminousFlux};

impl Display for UnitLuminousFlux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}lm", self.get_metric().as_str())
    }
}

impl Into<String> for UnitLuminousFlux {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitLuminousFlux> for UnitLuminousFlux {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitLuminousFlux) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitLuminousFlux {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Lumen(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Lumen(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
