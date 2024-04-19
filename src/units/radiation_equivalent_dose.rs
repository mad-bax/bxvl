use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{Metric, BaseUnit, Convert};

/// The unit types of equaivalent dose of ionizing radiation
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitRadioactivityExposure {
    /// SI unit
    Sievert(Metric),
    /// Legacy
    Rem,
}

impl Display for UnitRadioactivityExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sievert(m) => format!("{}Sv", m.as_str()),
                Self::Rem => "rem".into(),
            }
        )
    }
}

impl Into<String> for UnitRadioactivityExposure {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitRadioactivityExposure> for UnitRadioactivityExposure {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitRadioactivityExposure) -> f64 {
        (self.scale() / self.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitRadioactivityExposure {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Sievert(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Sievert(_) => 1.0,
            Self::Rem => constants::RADEX_REM_TO_SV,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Sievert(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod equivalent_dose_testing {
    use crate::units::{
        Metric, radiation_equivalent_dose::UnitRadioactivityExposure, BaseUnit,
    };

    /// Unit Equivalent Dose of Ionizing Radiation Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_equivalent_base_comparison() {
        // Sieverts are the base SI unit
        assert!(UnitRadioactivityExposure::Sievert(Metric::None).base() == 1.0);
        // Rems
        assert!(UnitRadioactivityExposure::Rem.base() == 0.01);
    }
}
