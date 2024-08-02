use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitSolidAngle};

impl Display for UnitSolidAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}sr", self.get_metric().as_str())
    }
}

impl From<UnitSolidAngle> for String {
    fn from(val: UnitSolidAngle) -> Self {
        val.to_string()
    }
}

impl Convert<UnitSolidAngle> for UnitSolidAngle {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitSolidAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitSolidAngle {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Steradian(m) => m.scale(),
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Steradian(_) => 1.0,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Steradian(m) => *m,
        }
    }
}

#[cfg(test)]
mod solid_angle_testing {
    use crate::units::{BaseUnit, Metric, UnitSolidAngle};

    #[test]
    fn unit_angle_base_comparison() {
        // Steradians are the base SI unit
        assert!(UnitSolidAngle::Steradian(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitSolidAngle::Steradian(Metric::Quecto), "qsr"),
            (UnitSolidAngle::Steradian(Metric::Ronto), "rsr"),
            (UnitSolidAngle::Steradian(Metric::Ronna), "Rsr"),
            (UnitSolidAngle::Steradian(Metric::Quetta), "Qsr"),
            (UnitSolidAngle::Steradian(Metric::Atto), "asr"),
            (UnitSolidAngle::Steradian(Metric::Centi), "csr"),
            (UnitSolidAngle::Steradian(Metric::Deca), "dasr"),
            (UnitSolidAngle::Steradian(Metric::Deci), "dsr"),
            (UnitSolidAngle::Steradian(Metric::Exa), "Esr"),
            (UnitSolidAngle::Steradian(Metric::Femto), "fsr"),
            (UnitSolidAngle::Steradian(Metric::Giga), "Gsr"),
            (UnitSolidAngle::Steradian(Metric::Hecto), "hsr"),
            (UnitSolidAngle::Steradian(Metric::Kilo), "ksr"),
            (UnitSolidAngle::Steradian(Metric::Mega), "Msr"),
            (UnitSolidAngle::Steradian(Metric::Micro), "μsr"),
            (UnitSolidAngle::Steradian(Metric::Milli), "msr"),
            (UnitSolidAngle::Steradian(Metric::Nano), "nsr"),
            (UnitSolidAngle::Steradian(Metric::None), "sr"),
            (UnitSolidAngle::Steradian(Metric::Peta), "Psr"),
            (UnitSolidAngle::Steradian(Metric::Pico), "psr"),
            (UnitSolidAngle::Steradian(Metric::Tera), "Tsr"),
            (UnitSolidAngle::Steradian(Metric::Yocto), "ysr"),
            (UnitSolidAngle::Steradian(Metric::Yotta), "Ysr"),
            (UnitSolidAngle::Steradian(Metric::Zepto), "zsr"),
            (UnitSolidAngle::Steradian(Metric::Zetta), "Zsr"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitSolidAngle::Steradian(Metric::Quecto), Metric::Quecto),
            (UnitSolidAngle::Steradian(Metric::Ronto), Metric::Ronto),
            (UnitSolidAngle::Steradian(Metric::Ronna), Metric::Ronna),
            (UnitSolidAngle::Steradian(Metric::Quetta), Metric::Quetta),
            (UnitSolidAngle::Steradian(Metric::Atto), Metric::Atto),
            (UnitSolidAngle::Steradian(Metric::Centi), Metric::Centi),
            (UnitSolidAngle::Steradian(Metric::Deca), Metric::Deca),
            (UnitSolidAngle::Steradian(Metric::Deci), Metric::Deci),
            (UnitSolidAngle::Steradian(Metric::Exa), Metric::Exa),
            (UnitSolidAngle::Steradian(Metric::Femto), Metric::Femto),
            (UnitSolidAngle::Steradian(Metric::Giga), Metric::Giga),
            (UnitSolidAngle::Steradian(Metric::Hecto), Metric::Hecto),
            (UnitSolidAngle::Steradian(Metric::Kilo), Metric::Kilo),
            (UnitSolidAngle::Steradian(Metric::Mega), Metric::Mega),
            (UnitSolidAngle::Steradian(Metric::Micro), Metric::Micro),
            (UnitSolidAngle::Steradian(Metric::Milli), Metric::Milli),
            (UnitSolidAngle::Steradian(Metric::Nano), Metric::Nano),
            (UnitSolidAngle::Steradian(Metric::None), Metric::None),
            (UnitSolidAngle::Steradian(Metric::Peta), Metric::Peta),
            (UnitSolidAngle::Steradian(Metric::Pico), Metric::Pico),
            (UnitSolidAngle::Steradian(Metric::Tera), Metric::Tera),
            (UnitSolidAngle::Steradian(Metric::Yocto), Metric::Yocto),
            (UnitSolidAngle::Steradian(Metric::Yotta), Metric::Yotta),
            (UnitSolidAngle::Steradian(Metric::Zepto), Metric::Zepto),
            (UnitSolidAngle::Steradian(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitSolidAngle::Steradian(Metric::Ronna), 1.0e27),
            (UnitSolidAngle::Steradian(Metric::Ronto), 1.0e-27),
            (UnitSolidAngle::Steradian(Metric::Quecto), 1.0e-30),
            (UnitSolidAngle::Steradian(Metric::Quetta), 1.0e30),
            (UnitSolidAngle::Steradian(Metric::Atto), 1.0e-18),
            (UnitSolidAngle::Steradian(Metric::Centi), 0.01),
            (UnitSolidAngle::Steradian(Metric::Deca), 10.0),
            (UnitSolidAngle::Steradian(Metric::Deci), 0.1),
            (UnitSolidAngle::Steradian(Metric::Exa), 1.0e18),
            (UnitSolidAngle::Steradian(Metric::Femto), 1.0e-15),
            (UnitSolidAngle::Steradian(Metric::Giga), 1.0e9),
            (UnitSolidAngle::Steradian(Metric::Hecto), 100.0),
            (UnitSolidAngle::Steradian(Metric::Kilo), 1.0e3),
            (UnitSolidAngle::Steradian(Metric::Mega), 1.0e6),
            (UnitSolidAngle::Steradian(Metric::Micro), 1.0e-6),
            (UnitSolidAngle::Steradian(Metric::Milli), 0.001),
            (UnitSolidAngle::Steradian(Metric::Nano), 1.0e-9),
            (UnitSolidAngle::Steradian(Metric::None), 1.0),
            (UnitSolidAngle::Steradian(Metric::Peta), 1.0e15),
            (UnitSolidAngle::Steradian(Metric::Pico), 1.0e-12),
            (UnitSolidAngle::Steradian(Metric::Tera), 1.0e12),
            (UnitSolidAngle::Steradian(Metric::Yocto), 1.0e-24),
            (UnitSolidAngle::Steradian(Metric::Yotta), 1.0e24),
            (UnitSolidAngle::Steradian(Metric::Zepto), 1.0e-21),
            (UnitSolidAngle::Steradian(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
