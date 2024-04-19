use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{Metric, BaseUnit, Convert};

/// The unit types for electric resistance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricResistance {
    /// SI unit
    Ohm(Metric),
}

impl Display for UnitElectricResistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Ω", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricResistance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricResistance> for UnitElectricResistance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricResistance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricResistance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Ohm(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Ohm(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
