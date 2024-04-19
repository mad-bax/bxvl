use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{Metric, BaseUnit, Convert};

/// The unit types of radioactivity
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitRadioactivity {
    /// SI unit
    Becquerel(Metric),
    /// Legacy
    Curie,
}

impl Display for UnitRadioactivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Becquerel(m) => format!("{}Bq", m.as_str()),
                Self::Curie => "Ci".into(),
            }
        )
    }
}

impl Into<String> for UnitRadioactivity {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitRadioactivity> for UnitRadioactivity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitRadioactivity) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitRadioactivity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Becquerel(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Becquerel(_) => 1.0,
            Self::Curie => constants::RADIO_C_TO_BQ,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Becquerel(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod radioactivity_testing {
    use crate::units::{Metric, radioactivity::UnitRadioactivity, BaseUnit};

    /// Unit Radioactivity Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_radioactivity_base_comparison() {
        // Becquerels are the base SI unit
        assert!(UnitRadioactivity::Becquerel(Metric::None).base() == 1.0);
        // Curies
        assert!(UnitRadioactivity::Curie.base() == 37_000_000_000.0);
    }
}
