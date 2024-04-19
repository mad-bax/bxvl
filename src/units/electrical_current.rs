use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{Metric, BaseUnit, Convert};

/// The unit types for electric current
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricCurrent {
    /// SI unit
    Ampere(Metric),
}

impl Display for UnitElectricCurrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}A", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricCurrent {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricCurrent> for UnitElectricCurrent {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricCurrent) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricCurrent {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Ampere(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Ampere(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
