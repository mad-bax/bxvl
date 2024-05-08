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

    #[test]
    fn unit_energy_base_comparison() {
        // Joules are the base SI unit
        assert!(UnitEnergy::Joule(Metric::None).base() == 1.0);
        // Calories
        assert!(UnitEnergy::GramCalorie(Metric::None).base() == 4.184);
        // Footpounds
        assert!(UnitEnergy::FootPound.base() == 1.355818);
        // Electron Volts
        assert!(UnitEnergy::ElectronVolt(Metric::None).base() >= 1.602_176_634e-19);
        assert!(UnitEnergy::ElectronVolt(Metric::None).base() < 1.602_176_635e-19);
    }

    #[test]
    fn unit_energy_to_string() {
        for i in [
            (UnitEnergy::Joule(Metric::Atto), "aHz"),
            (UnitEnergy::Joule(Metric::Centi), "cHz"),
            (UnitEnergy::Joule(Metric::Deca), "daHz"),
            (UnitEnergy::Joule(Metric::Deci), "dHz"),
            (UnitEnergy::Joule(Metric::Exa), "EHz"),
            (UnitEnergy::Joule(Metric::Femto), "fHz"),
            (UnitEnergy::Joule(Metric::Giga), "GHz"),
            (UnitEnergy::Joule(Metric::Hecto), "hHz"),
            (UnitEnergy::Joule(Metric::Kilo), "kHz"),
            (UnitEnergy::Joule(Metric::Mega), "MHz"),
            (UnitEnergy::Joule(Metric::Micro), "μHz"),
            (UnitEnergy::Joule(Metric::Milli), "mHz"),
            (UnitEnergy::Joule(Metric::Nano), "nHz"),
            (UnitEnergy::Joule(Metric::None), "Hz"),
            (UnitEnergy::Joule(Metric::Peta), "PHz"),
            (UnitEnergy::Joule(Metric::Pico), "pHz"),
            (UnitEnergy::Joule(Metric::Tera), "THz"),
            (UnitEnergy::Joule(Metric::Yocto), "yHz"),
            (UnitEnergy::Joule(Metric::Yotta), "YHz"),
            (UnitEnergy::Joule(Metric::Zepto), "zHz"),
            (UnitEnergy::Joule(Metric::Zetta), "ZHz"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }
}
