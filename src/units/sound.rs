use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{metric::Metric, BaseUnit, Convert};

/// The unit types of a measurement of sound
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitSound {
    /// SI unit
    Bel(Metric),
}

impl Display for UnitSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}B", self.get_metric().as_str())
    }
}

impl Into<String> for UnitSound {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitSound> for UnitSound {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitSound) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitSound {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Bel(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Bel(m) => *m,
        }
    }
    
    fn base(&self) -> f64 {
        1.0
    }
}