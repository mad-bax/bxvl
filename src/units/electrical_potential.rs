use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{metric::Metric, BaseUnit, Convert};

/// The unit types for electric potential
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricPotential {
    /// SI unit
    Volt(Metric),
}

impl Display for UnitElectricPotential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}V", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricPotential {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricPotential> for UnitElectricPotential {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricPotential) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricPotential {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Volt(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Volt(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}