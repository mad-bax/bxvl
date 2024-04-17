use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{metric::Metric, BaseUnit, Convert};

/// The unit types for catalytic activity
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitCatalyticActivity {
    /// SI unit
    Katal(Metric),
}

impl Display for UnitCatalyticActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kat", self.get_metric().as_str())
    }
}

impl Into<String> for UnitCatalyticActivity {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitCatalyticActivity> for UnitCatalyticActivity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitCatalyticActivity) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitCatalyticActivity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Katal(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Katal(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}