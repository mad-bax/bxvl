use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{Metric, BaseUnit, Convert};

/// The unit types for substance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitSubstance {
    /// SI unit
    Mole(Metric),
}

impl Display for UnitSubstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mol", self.get_metric().as_str())
    }
}

impl Into<String> for UnitSubstance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitSubstance> for UnitSubstance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitSubstance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitSubstance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Mole(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Mole(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
