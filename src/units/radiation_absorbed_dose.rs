use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{metric::Metric, BaseUnit, Convert};

/// The unit types of absorbed dose of ionizing radiation
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitAbsorbedDose {
    /// SI unit
    Gray(Metric),
    /// Legacy
    Roentgen,
    /// Legacy
    Rad,
}

impl Display for UnitAbsorbedDose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret: String = String::new();
        write!(f, "{}", match self {
            Self::Gray(m) => format!("{}Gy", m.as_str()),
            Self::Roentgen => "R".into(),
            Self::Rad => "rads".into(),
        })
    }
}

impl Into<String> for UnitAbsorbedDose {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitAbsorbedDose> for UnitAbsorbedDose {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitAbsorbedDose) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitAbsorbedDose {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Gray(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Gray(_) => 1.0,
            Self::Roentgen => constants::AB_ROE_TO_GY,
            Self::Rad => constants::AB_RAD_TO_GY,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Gray(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod absorbed_dose_testing {
    use crate::units::{metric::Metric, radiation_absorbed_dose::UnitAbsorbedDose};

    /// Unit Absorbed Dose of Ionizing Radiation Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_absorbed_base_comparison() {
        // Grays are the base SI unit
        assert!(UnitAbsorbedDose::Gray(Metric::None).base() == 1.0);
        // Rads
        assert!(UnitAbsorbedDose::Rad.base() == 0.01);
        // Roentgens
        assert!(UnitAbsorbedDose::Roentgen.base() >= 0.00877000656);
    }
}