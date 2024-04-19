use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{Metric, BaseUnit, Convert};

/// The unit types of force
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitForce {
    /// SI unit
    Newton(Metric),
    /// Imperial
    PoundForce,
}

impl Display for UnitForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Newton(m) => format!("{}N", m.as_str()),
                Self::PoundForce => "lbfr".into(),
            }
        )
    }
}

impl Into<String> for UnitForce {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitForce> for UnitForce {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitForce) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitForce {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Newton(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Newton(_) => 1.0,
            Self::PoundForce => constants::FC_LBF_TO_N,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Newton(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod force_testing {
    use crate::units::{force::UnitForce, Metric, BaseUnit};

    /// Unit Force Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_force_base_comparison() {
        // Newtons are the base SI unit
        assert!(UnitForce::Newton(Metric::None).base() == 1.0);
        // Poundforce
        assert!(UnitForce::PoundForce.base() == 4.448_221_615_260_5);
    }
}
