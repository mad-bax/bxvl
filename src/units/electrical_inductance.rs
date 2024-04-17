use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{metric::Metric, BaseUnit, Convert};

/// The unit types for electric inductance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricInductance {
    /// SI unit
    Henry(Metric),
}

impl Display for UnitElectricInductance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}H", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricInductance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricInductance> for UnitElectricInductance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricInductance) -> f64 {
        self.scale() / other.scale()
    }   
}

impl BaseUnit for UnitElectricInductance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Henry(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Henry(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
