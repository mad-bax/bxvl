use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{metric::Metric, time::UnitTime, BaseUnit, Convert};

/// The unit types of frequency
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitFrequency {
    /// SI unit
    Hertz(Metric),
}

impl Display for UnitFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Hz", self.get_metric().as_str())
    }
}

impl Into<String> for UnitFrequency {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitFrequency> for UnitFrequency {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitFrequency) -> f64 {
        self.scale() / other.scale()
    }
}

impl Convert<UnitTime> for UnitFrequency {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitTime) -> f64 {
        (self.scale() / other.scale()) * (other.convert(&UnitTime::Second(Metric::None)))
    }
}

impl BaseUnit for UnitFrequency {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Hertz(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Hertz(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}