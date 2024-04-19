use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{Metric, BaseUnit, Convert};

/// The unit types for pressure
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitPressure {
    /// SI unit
    Pascal(Metric),
    /// SI integrated, Common Standard
    Bar(Metric),
    /// Common Standard
    Torr,
    /// SI integrated
    Hgmm,
    /// Imperial
    Hgin,
    /// Common Standard
    Atm,
    /// Imperial
    Psi,
}

impl Display for UnitPressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pascal(m) => format!("{}Pa", m.as_str()),
                Self::Bar(m) => format!("{}bar", m.as_str()),
                Self::Torr => "torr".into(),
                Self::Hgmm => "mmHg".into(),
                Self::Hgin => "inHg".into(),
                Self::Atm => "atm".into(),
                Self::Psi => "psi".into(),
            }
        )
    }
}

impl Into<String> for UnitPressure {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitPressure> for UnitPressure {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitPressure) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitPressure {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Pascal(m) => m.scale(),
            Self::Bar(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Pascal(_) => 1.0,
            Self::Bar(_) => constants::PR_BAR_TO_P,
            Self::Torr => constants::PR_TORR_TO_P,
            Self::Hgmm => constants::PR_MM_TO_P,
            Self::Hgin => constants::PR_IN_TO_P,
            Self::Atm => constants::PR_ATM_TO_P,
            Self::Psi => constants::PR_PSI_TO_P,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Pascal(m) => *m,
            Self::Bar(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod pressure_testing {
    use crate::units::{Metric, pressure::UnitPressure, BaseUnit};

    /// Unit Pressure Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_pressure_base_comparison() {
        // Pascals are the base SI unit
        assert!(UnitPressure::Pascal(Metric::None).base() == 1.0);
        // Atmospheres
        assert!(UnitPressure::Atm.base() == 101325.0);
        // Bar
        assert!(UnitPressure::Bar(Metric::None).base() == 100000.0);
        // inHg
        assert!(UnitPressure::Hgin.base() >= 3386.3886665);
        // mmHg
        assert!(UnitPressure::Hgmm.base() >= 133.322387414);
        // PSI
        assert!(UnitPressure::Psi.base() == 6894.757);
        // Torr
        assert!(UnitPressure::Torr.base() >= 133.322368420);
    }
}
