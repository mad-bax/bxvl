use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitEnergy};

impl Display for UnitEnergy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Joule(m) => format!("{}J", m.as_str()),
                Self::GramCalorie(Metric::Kilo) => "Cal".into(),
                Self::GramCalorie(m) => format!("{}cal", m.as_str()),
                Self::FootPound => "ftlb".into(),
                Self::ElectronVolt(m) => format!("{}eV", m.as_str()),
            }
        )
    }
}

impl Into<String> for UnitEnergy {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitEnergy> for UnitEnergy {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitEnergy) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitEnergy {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Joule(m) => m.scale(),
            Self::GramCalorie(m) => m.scale(),
            Self::ElectronVolt(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Joule(_) => 1.0,
            Self::GramCalorie(_) => constants::EN_CAL_TO_J,
            Self::FootPound => constants::EN_FTLB_TO_J,
            Self::ElectronVolt(_) => constants::EN_EV_TO_J,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Joule(m) => *m,
            Self::GramCalorie(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod energy_testing {
    use crate::units::{energy::UnitEnergy, BaseUnit, Metric};

    /// Unit Energy Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_energy_base_comparison() {
        // Joules are the base SI unit
        assert!(UnitEnergy::Joule(Metric::None).base() == 1.0);
        // Calories
        assert!(UnitEnergy::GramCalorie(Metric::None).base() == 4.184);
        // Footpounds
        assert!(UnitEnergy::FootPound.base() == 1.355818);
        // Electron Volts
        assert!(UnitEnergy::ElectronVolt(Metric::None).base() >= 1.602_176_633e-19);
    }
}
