use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{metric::Metric, BaseUnit, Convert};

/// The unit types for mass
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitMass {
    /// SI unit
    Gram(Metric),
    /// Imperial
    Grain,
    /// Imperial
    Ounce,
    /// Imperial
    Pound,
}

impl Display for UnitMass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Gram(m) => format!("{}{}", m.as_str(), "g"),
            Self::Grain => "gr".into(),
            Self::Ounce => "oz".into(),
            Self::Pound => "lb".into(),
        })
    }
}

impl Into<String> for UnitMass {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitMass> for UnitMass {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitMass) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitMass {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Gram(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Gram(_) => 1.0,
            Self::Grain => constants::MASS_GR_TO_G,
            Self::Ounce => constants::MASS_OZ_TO_G,
            Self::Pound => constants::MASS_LB_TO_G,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Gram(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod mass_testing {
    use crate::units::{mass::UnitMass, metric::Metric};

    /// Unit Mass Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_mass_base_comparison() {
        // Grams are the base SI unit
        assert!(UnitMass::Gram(Metric::None).base() == 1.0);
        // Pounds
        assert!(UnitMass::Pound.base() == 453.592_37);
        // Grains
        assert!(UnitMass::Grain.base() >= 0.06479890);
        // Ounces
        assert!(UnitMass::Ounce.base() >= 28.349_523_124);
    }
}