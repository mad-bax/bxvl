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

impl From<UnitEnergy> for String {
    fn from(val: UnitEnergy) -> Self {
        val.to_string()
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
            Self::ElectronVolt(m) => *m,
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
            (UnitEnergy::Joule(Metric::Ronto), "rJ"),
            (UnitEnergy::Joule(Metric::Ronna), "RJ"),
            (UnitEnergy::Joule(Metric::Quetta), "QJ"),
            (UnitEnergy::Joule(Metric::Quecto), "qJ"),
            (UnitEnergy::Joule(Metric::Atto), "aJ"),
            (UnitEnergy::Joule(Metric::Centi), "cJ"),
            (UnitEnergy::Joule(Metric::Deca), "daJ"),
            (UnitEnergy::Joule(Metric::Deci), "dJ"),
            (UnitEnergy::Joule(Metric::Exa), "EJ"),
            (UnitEnergy::Joule(Metric::Femto), "fJ"),
            (UnitEnergy::Joule(Metric::Giga), "GJ"),
            (UnitEnergy::Joule(Metric::Hecto), "hJ"),
            (UnitEnergy::Joule(Metric::Kilo), "kJ"),
            (UnitEnergy::Joule(Metric::Mega), "MJ"),
            (UnitEnergy::Joule(Metric::Micro), "μJ"),
            (UnitEnergy::Joule(Metric::Milli), "mJ"),
            (UnitEnergy::Joule(Metric::Nano), "nJ"),
            (UnitEnergy::Joule(Metric::None), "J"),
            (UnitEnergy::Joule(Metric::Peta), "PJ"),
            (UnitEnergy::Joule(Metric::Pico), "pJ"),
            (UnitEnergy::Joule(Metric::Tera), "TJ"),
            (UnitEnergy::Joule(Metric::Yocto), "yJ"),
            (UnitEnergy::Joule(Metric::Yotta), "YJ"),
            (UnitEnergy::Joule(Metric::Zepto), "zJ"),
            (UnitEnergy::Joule(Metric::Zetta), "ZJ"),
            (UnitEnergy::FootPound, "ftlb"),
            (UnitEnergy::GramCalorie(Metric::Ronto), "rcal"),
            (UnitEnergy::GramCalorie(Metric::Ronna), "Rcal"),
            (UnitEnergy::GramCalorie(Metric::Quetta), "Qcal"),
            (UnitEnergy::GramCalorie(Metric::Quecto), "qcal"),
            (UnitEnergy::GramCalorie(Metric::Atto), "acal"),
            (UnitEnergy::GramCalorie(Metric::Centi), "ccal"),
            (UnitEnergy::GramCalorie(Metric::Deca), "dacal"),
            (UnitEnergy::GramCalorie(Metric::Deci), "dcal"),
            (UnitEnergy::GramCalorie(Metric::Exa), "Ecal"),
            (UnitEnergy::GramCalorie(Metric::Femto), "fcal"),
            (UnitEnergy::GramCalorie(Metric::Giga), "Gcal"),
            (UnitEnergy::GramCalorie(Metric::Hecto), "hcal"),
            (UnitEnergy::GramCalorie(Metric::Kilo), "Cal"),
            (UnitEnergy::GramCalorie(Metric::Mega), "Mcal"),
            (UnitEnergy::GramCalorie(Metric::Micro), "μcal"),
            (UnitEnergy::GramCalorie(Metric::Milli), "mcal"),
            (UnitEnergy::GramCalorie(Metric::Nano), "ncal"),
            (UnitEnergy::GramCalorie(Metric::None), "cal"),
            (UnitEnergy::GramCalorie(Metric::Peta), "Pcal"),
            (UnitEnergy::GramCalorie(Metric::Pico), "pcal"),
            (UnitEnergy::GramCalorie(Metric::Tera), "Tcal"),
            (UnitEnergy::GramCalorie(Metric::Yocto), "ycal"),
            (UnitEnergy::GramCalorie(Metric::Yotta), "Ycal"),
            (UnitEnergy::GramCalorie(Metric::Zepto), "zcal"),
            (UnitEnergy::GramCalorie(Metric::Zetta), "Zcal"),
            (UnitEnergy::ElectronVolt(Metric::Ronto), "reV"),
            (UnitEnergy::ElectronVolt(Metric::Ronna), "ReV"),
            (UnitEnergy::ElectronVolt(Metric::Quetta), "QeV"),
            (UnitEnergy::ElectronVolt(Metric::Quecto), "qeV"),
            (UnitEnergy::ElectronVolt(Metric::Atto), "aeV"),
            (UnitEnergy::ElectronVolt(Metric::Centi), "ceV"),
            (UnitEnergy::ElectronVolt(Metric::Deca), "daeV"),
            (UnitEnergy::ElectronVolt(Metric::Deci), "deV"),
            (UnitEnergy::ElectronVolt(Metric::Exa), "EeV"),
            (UnitEnergy::ElectronVolt(Metric::Femto), "feV"),
            (UnitEnergy::ElectronVolt(Metric::Giga), "GeV"),
            (UnitEnergy::ElectronVolt(Metric::Hecto), "heV"),
            (UnitEnergy::ElectronVolt(Metric::Kilo), "keV"),
            (UnitEnergy::ElectronVolt(Metric::Mega), "MeV"),
            (UnitEnergy::ElectronVolt(Metric::Micro), "μeV"),
            (UnitEnergy::ElectronVolt(Metric::Milli), "meV"),
            (UnitEnergy::ElectronVolt(Metric::Nano), "neV"),
            (UnitEnergy::ElectronVolt(Metric::None), "eV"),
            (UnitEnergy::ElectronVolt(Metric::Peta), "PeV"),
            (UnitEnergy::ElectronVolt(Metric::Pico), "peV"),
            (UnitEnergy::ElectronVolt(Metric::Tera), "TeV"),
            (UnitEnergy::ElectronVolt(Metric::Yocto), "yeV"),
            (UnitEnergy::ElectronVolt(Metric::Yotta), "YeV"),
            (UnitEnergy::ElectronVolt(Metric::Zepto), "zeV"),
            (UnitEnergy::ElectronVolt(Metric::Zetta), "ZeV"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitEnergy::ElectronVolt(Metric::Quetta), Metric::Quetta),
            (UnitEnergy::ElectronVolt(Metric::Quecto), Metric::Quecto),
            (UnitEnergy::ElectronVolt(Metric::Ronto), Metric::Ronto),
            (UnitEnergy::ElectronVolt(Metric::Ronna), Metric::Ronna),
            (UnitEnergy::ElectronVolt(Metric::Atto), Metric::Atto),
            (UnitEnergy::ElectronVolt(Metric::Centi), Metric::Centi),
            (UnitEnergy::ElectronVolt(Metric::Deca), Metric::Deca),
            (UnitEnergy::ElectronVolt(Metric::Deci), Metric::Deci),
            (UnitEnergy::ElectronVolt(Metric::Exa), Metric::Exa),
            (UnitEnergy::ElectronVolt(Metric::Femto), Metric::Femto),
            (UnitEnergy::ElectronVolt(Metric::Giga), Metric::Giga),
            (UnitEnergy::ElectronVolt(Metric::Hecto), Metric::Hecto),
            (UnitEnergy::ElectronVolt(Metric::Kilo), Metric::Kilo),
            (UnitEnergy::ElectronVolt(Metric::Mega), Metric::Mega),
            (UnitEnergy::ElectronVolt(Metric::Micro), Metric::Micro),
            (UnitEnergy::ElectronVolt(Metric::Milli), Metric::Milli),
            (UnitEnergy::ElectronVolt(Metric::Nano), Metric::Nano),
            (UnitEnergy::ElectronVolt(Metric::None), Metric::None),
            (UnitEnergy::ElectronVolt(Metric::Peta), Metric::Peta),
            (UnitEnergy::ElectronVolt(Metric::Pico), Metric::Pico),
            (UnitEnergy::ElectronVolt(Metric::Tera), Metric::Tera),
            (UnitEnergy::ElectronVolt(Metric::Yocto), Metric::Yocto),
            (UnitEnergy::ElectronVolt(Metric::Yotta), Metric::Yotta),
            (UnitEnergy::ElectronVolt(Metric::Zepto), Metric::Zepto),
            (UnitEnergy::ElectronVolt(Metric::Zetta), Metric::Zetta),
            (UnitEnergy::GramCalorie(Metric::Quetta), Metric::Quetta),
            (UnitEnergy::GramCalorie(Metric::Quecto), Metric::Quecto),
            (UnitEnergy::GramCalorie(Metric::Ronto), Metric::Ronto),
            (UnitEnergy::GramCalorie(Metric::Ronna), Metric::Ronna),
            (UnitEnergy::GramCalorie(Metric::Atto), Metric::Atto),
            (UnitEnergy::GramCalorie(Metric::Centi), Metric::Centi),
            (UnitEnergy::GramCalorie(Metric::Deca), Metric::Deca),
            (UnitEnergy::GramCalorie(Metric::Deci), Metric::Deci),
            (UnitEnergy::GramCalorie(Metric::Exa), Metric::Exa),
            (UnitEnergy::GramCalorie(Metric::Femto), Metric::Femto),
            (UnitEnergy::GramCalorie(Metric::Giga), Metric::Giga),
            (UnitEnergy::GramCalorie(Metric::Hecto), Metric::Hecto),
            (UnitEnergy::GramCalorie(Metric::Kilo), Metric::Kilo),
            (UnitEnergy::GramCalorie(Metric::Mega), Metric::Mega),
            (UnitEnergy::GramCalorie(Metric::Micro), Metric::Micro),
            (UnitEnergy::GramCalorie(Metric::Milli), Metric::Milli),
            (UnitEnergy::GramCalorie(Metric::Nano), Metric::Nano),
            (UnitEnergy::GramCalorie(Metric::None), Metric::None),
            (UnitEnergy::GramCalorie(Metric::Peta), Metric::Peta),
            (UnitEnergy::GramCalorie(Metric::Pico), Metric::Pico),
            (UnitEnergy::GramCalorie(Metric::Tera), Metric::Tera),
            (UnitEnergy::GramCalorie(Metric::Yocto), Metric::Yocto),
            (UnitEnergy::GramCalorie(Metric::Yotta), Metric::Yotta),
            (UnitEnergy::GramCalorie(Metric::Zepto), Metric::Zepto),
            (UnitEnergy::GramCalorie(Metric::Zetta), Metric::Zetta),
            (UnitEnergy::Joule(Metric::Quetta), Metric::Quetta),
            (UnitEnergy::Joule(Metric::Quecto), Metric::Quecto),
            (UnitEnergy::Joule(Metric::Ronto), Metric::Ronto),
            (UnitEnergy::Joule(Metric::Ronna), Metric::Ronna),
            (UnitEnergy::Joule(Metric::Atto), Metric::Atto),
            (UnitEnergy::Joule(Metric::Centi), Metric::Centi),
            (UnitEnergy::Joule(Metric::Deca), Metric::Deca),
            (UnitEnergy::Joule(Metric::Deci), Metric::Deci),
            (UnitEnergy::Joule(Metric::Exa), Metric::Exa),
            (UnitEnergy::Joule(Metric::Femto), Metric::Femto),
            (UnitEnergy::Joule(Metric::Giga), Metric::Giga),
            (UnitEnergy::Joule(Metric::Hecto), Metric::Hecto),
            (UnitEnergy::Joule(Metric::Kilo), Metric::Kilo),
            (UnitEnergy::Joule(Metric::Mega), Metric::Mega),
            (UnitEnergy::Joule(Metric::Micro), Metric::Micro),
            (UnitEnergy::Joule(Metric::Milli), Metric::Milli),
            (UnitEnergy::Joule(Metric::Nano), Metric::Nano),
            (UnitEnergy::Joule(Metric::None), Metric::None),
            (UnitEnergy::Joule(Metric::Peta), Metric::Peta),
            (UnitEnergy::Joule(Metric::Pico), Metric::Pico),
            (UnitEnergy::Joule(Metric::Tera), Metric::Tera),
            (UnitEnergy::Joule(Metric::Yocto), Metric::Yocto),
            (UnitEnergy::Joule(Metric::Yotta), Metric::Yotta),
            (UnitEnergy::Joule(Metric::Zepto), Metric::Zepto),
            (UnitEnergy::Joule(Metric::Zetta), Metric::Zetta),
            (UnitEnergy::FootPound, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitEnergy::ElectronVolt(Metric::Ronna), 1.0e27),
            (UnitEnergy::ElectronVolt(Metric::Ronto), 1.0e-27),
            (UnitEnergy::ElectronVolt(Metric::Quetta), 1.0e30),
            (UnitEnergy::ElectronVolt(Metric::Quecto), 1.0e-30),
            (UnitEnergy::ElectronVolt(Metric::Atto), 1.0e-18),
            (UnitEnergy::ElectronVolt(Metric::Centi), 0.01),
            (UnitEnergy::ElectronVolt(Metric::Deca), 10.0),
            (UnitEnergy::ElectronVolt(Metric::Deci), 0.1),
            (UnitEnergy::ElectronVolt(Metric::Exa), 1.0e18),
            (UnitEnergy::ElectronVolt(Metric::Femto), 1.0e-15),
            (UnitEnergy::ElectronVolt(Metric::Giga), 1.0e9),
            (UnitEnergy::ElectronVolt(Metric::Hecto), 100.0),
            (UnitEnergy::ElectronVolt(Metric::Kilo), 1.0e3),
            (UnitEnergy::ElectronVolt(Metric::Mega), 1.0e6),
            (UnitEnergy::ElectronVolt(Metric::Micro), 1.0e-6),
            (UnitEnergy::ElectronVolt(Metric::Milli), 0.001),
            (UnitEnergy::ElectronVolt(Metric::Nano), 1.0e-9),
            (UnitEnergy::ElectronVolt(Metric::None), 1.0),
            (UnitEnergy::ElectronVolt(Metric::Peta), 1.0e15),
            (UnitEnergy::ElectronVolt(Metric::Pico), 1.0e-12),
            (UnitEnergy::ElectronVolt(Metric::Tera), 1.0e12),
            (UnitEnergy::ElectronVolt(Metric::Yocto), 1.0e-24),
            (UnitEnergy::ElectronVolt(Metric::Yotta), 1.0e24),
            (UnitEnergy::ElectronVolt(Metric::Zepto), 1.0e-21),
            (UnitEnergy::ElectronVolt(Metric::Zetta), 1.0e21),
            (UnitEnergy::GramCalorie(Metric::Ronna), 1.0e27),
            (UnitEnergy::GramCalorie(Metric::Ronto), 1.0e-27),
            (UnitEnergy::GramCalorie(Metric::Quetta), 1.0e30),
            (UnitEnergy::GramCalorie(Metric::Quecto), 1.0e-30),
            (UnitEnergy::GramCalorie(Metric::Atto), 1.0e-18),
            (UnitEnergy::GramCalorie(Metric::Centi), 0.01),
            (UnitEnergy::GramCalorie(Metric::Deca), 10.0),
            (UnitEnergy::GramCalorie(Metric::Deci), 0.1),
            (UnitEnergy::GramCalorie(Metric::Exa), 1.0e18),
            (UnitEnergy::GramCalorie(Metric::Femto), 1.0e-15),
            (UnitEnergy::GramCalorie(Metric::Giga), 1.0e9),
            (UnitEnergy::GramCalorie(Metric::Hecto), 100.0),
            (UnitEnergy::GramCalorie(Metric::Kilo), 1.0e3),
            (UnitEnergy::GramCalorie(Metric::Mega), 1.0e6),
            (UnitEnergy::GramCalorie(Metric::Micro), 1.0e-6),
            (UnitEnergy::GramCalorie(Metric::Milli), 0.001),
            (UnitEnergy::GramCalorie(Metric::Nano), 1.0e-9),
            (UnitEnergy::GramCalorie(Metric::None), 1.0),
            (UnitEnergy::GramCalorie(Metric::Peta), 1.0e15),
            (UnitEnergy::GramCalorie(Metric::Pico), 1.0e-12),
            (UnitEnergy::GramCalorie(Metric::Tera), 1.0e12),
            (UnitEnergy::GramCalorie(Metric::Yocto), 1.0e-24),
            (UnitEnergy::GramCalorie(Metric::Yotta), 1.0e24),
            (UnitEnergy::GramCalorie(Metric::Zepto), 1.0e-21),
            (UnitEnergy::GramCalorie(Metric::Zetta), 1.0e21),
            (UnitEnergy::Joule(Metric::Ronna), 1.0e27),
            (UnitEnergy::Joule(Metric::Ronto), 1.0e-27),
            (UnitEnergy::Joule(Metric::Quetta), 1.0e30),
            (UnitEnergy::Joule(Metric::Quecto), 1.0e-30),
            (UnitEnergy::Joule(Metric::Atto), 1.0e-18),
            (UnitEnergy::Joule(Metric::Centi), 0.01),
            (UnitEnergy::Joule(Metric::Deca), 10.0),
            (UnitEnergy::Joule(Metric::Deci), 0.1),
            (UnitEnergy::Joule(Metric::Exa), 1.0e18),
            (UnitEnergy::Joule(Metric::Femto), 1.0e-15),
            (UnitEnergy::Joule(Metric::Giga), 1.0e9),
            (UnitEnergy::Joule(Metric::Hecto), 100.0),
            (UnitEnergy::Joule(Metric::Kilo), 1.0e3),
            (UnitEnergy::Joule(Metric::Mega), 1.0e6),
            (UnitEnergy::Joule(Metric::Micro), 1.0e-6),
            (UnitEnergy::Joule(Metric::Milli), 0.001),
            (UnitEnergy::Joule(Metric::Nano), 1.0e-9),
            (UnitEnergy::Joule(Metric::None), 1.0),
            (UnitEnergy::Joule(Metric::Peta), 1.0e15),
            (UnitEnergy::Joule(Metric::Pico), 1.0e-12),
            (UnitEnergy::Joule(Metric::Tera), 1.0e12),
            (UnitEnergy::Joule(Metric::Yocto), 1.0e-24),
            (UnitEnergy::Joule(Metric::Yotta), 1.0e24),
            (UnitEnergy::Joule(Metric::Zepto), 1.0e-21),
            (UnitEnergy::Joule(Metric::Zetta), 1.0e21),
            (UnitEnergy::FootPound, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
