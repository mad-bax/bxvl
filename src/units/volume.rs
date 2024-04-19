use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{length::UnitLength, Metric, BaseUnit, Convert};

/// The unit types for volume
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitVolume {
    /// SI unit
    Liter(Metric),
}

impl Display for UnitVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}l", self.get_metric().as_str())
    }
}

impl Into<String> for UnitVolume {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitVolume> for UnitVolume {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitVolume) -> f64 {
        self.scale() / other.scale()
    }
}

impl Convert<UnitLength> for UnitVolume {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitLength) -> f64 {
        self.scale()
            * (f64::powf(UnitLength::Meter(Metric::None).convert(other), 3.0)
                / constants::METER3_TO_LITER)
    }
}

impl BaseUnit for UnitVolume {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Liter(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Liter(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
