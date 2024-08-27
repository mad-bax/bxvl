use std::fmt::Display;

use crate::consts;

use super::{BaseUnit, Convert, Metric, UnitAbsorbedDose};

impl Display for UnitAbsorbedDose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Gray(m) => format!("{}Gy", m.as_str()),
                Self::Roentgen => "R".into(),
                Self::Rad => "rads".into(),
            }
        )
    }
}

impl From<UnitAbsorbedDose> for String {
    fn from(val: UnitAbsorbedDose) -> Self {
        val.to_string()
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
            Self::Roentgen => consts::AB_ROE_TO_GY,
            Self::Rad => consts::AB_RAD_TO_GY,
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
    use crate::units::{radiation_absorbed_dose::UnitAbsorbedDose, BaseUnit, Metric};

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
        assert!(UnitAbsorbedDose::Roentgen.base() < 0.00877000658);
    }

    #[test]
    fn unit_length_to_string() {
        for i in [
            (UnitAbsorbedDose::Gray(Metric::Ronto), "rGy"),
            (UnitAbsorbedDose::Gray(Metric::Ronna), "RGy"),
            (UnitAbsorbedDose::Gray(Metric::Quetta), "QGy"),
            (UnitAbsorbedDose::Gray(Metric::Quecto), "qGy"),
            (UnitAbsorbedDose::Gray(Metric::Atto), "aGy"),
            (UnitAbsorbedDose::Gray(Metric::Centi), "cGy"),
            (UnitAbsorbedDose::Gray(Metric::Deca), "daGy"),
            (UnitAbsorbedDose::Gray(Metric::Deci), "dGy"),
            (UnitAbsorbedDose::Gray(Metric::Exa), "EGy"),
            (UnitAbsorbedDose::Gray(Metric::Femto), "fGy"),
            (UnitAbsorbedDose::Gray(Metric::Giga), "GGy"),
            (UnitAbsorbedDose::Gray(Metric::Hecto), "hGy"),
            (UnitAbsorbedDose::Gray(Metric::Kilo), "kGy"),
            (UnitAbsorbedDose::Gray(Metric::Mega), "MGy"),
            (UnitAbsorbedDose::Gray(Metric::Micro), "μGy"),
            (UnitAbsorbedDose::Gray(Metric::Milli), "mGy"),
            (UnitAbsorbedDose::Gray(Metric::Nano), "nGy"),
            (UnitAbsorbedDose::Gray(Metric::None), "Gy"),
            (UnitAbsorbedDose::Gray(Metric::Peta), "PGy"),
            (UnitAbsorbedDose::Gray(Metric::Pico), "pGy"),
            (UnitAbsorbedDose::Gray(Metric::Tera), "TGy"),
            (UnitAbsorbedDose::Gray(Metric::Yocto), "yGy"),
            (UnitAbsorbedDose::Gray(Metric::Yotta), "YGy"),
            (UnitAbsorbedDose::Gray(Metric::Zepto), "zGy"),
            (UnitAbsorbedDose::Gray(Metric::Zetta), "ZGy"),
            (UnitAbsorbedDose::Roentgen, "R"),
            (UnitAbsorbedDose::Rad, "rads"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_length_scale() {
        for i in [
            (UnitAbsorbedDose::Gray(Metric::Ronto), Metric::Ronto),
            (UnitAbsorbedDose::Gray(Metric::Ronna), Metric::Ronna),
            (UnitAbsorbedDose::Gray(Metric::Quetta), Metric::Quetta),
            (UnitAbsorbedDose::Gray(Metric::Quecto), Metric::Quecto),
            (UnitAbsorbedDose::Gray(Metric::Atto), Metric::Atto),
            (UnitAbsorbedDose::Gray(Metric::Centi), Metric::Centi),
            (UnitAbsorbedDose::Gray(Metric::Deca), Metric::Deca),
            (UnitAbsorbedDose::Gray(Metric::Deci), Metric::Deci),
            (UnitAbsorbedDose::Gray(Metric::Exa), Metric::Exa),
            (UnitAbsorbedDose::Gray(Metric::Femto), Metric::Femto),
            (UnitAbsorbedDose::Gray(Metric::Giga), Metric::Giga),
            (UnitAbsorbedDose::Gray(Metric::Hecto), Metric::Hecto),
            (UnitAbsorbedDose::Gray(Metric::Kilo), Metric::Kilo),
            (UnitAbsorbedDose::Gray(Metric::Mega), Metric::Mega),
            (UnitAbsorbedDose::Gray(Metric::Micro), Metric::Micro),
            (UnitAbsorbedDose::Gray(Metric::Milli), Metric::Milli),
            (UnitAbsorbedDose::Gray(Metric::Nano), Metric::Nano),
            (UnitAbsorbedDose::Gray(Metric::None), Metric::None),
            (UnitAbsorbedDose::Gray(Metric::Peta), Metric::Peta),
            (UnitAbsorbedDose::Gray(Metric::Pico), Metric::Pico),
            (UnitAbsorbedDose::Gray(Metric::Tera), Metric::Tera),
            (UnitAbsorbedDose::Gray(Metric::Yocto), Metric::Yocto),
            (UnitAbsorbedDose::Gray(Metric::Yotta), Metric::Yotta),
            (UnitAbsorbedDose::Gray(Metric::Zepto), Metric::Zepto),
            (UnitAbsorbedDose::Gray(Metric::Zetta), Metric::Zetta),
            (UnitAbsorbedDose::Rad, Metric::None),
            (UnitAbsorbedDose::Roentgen, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
        for i in [
            (UnitAbsorbedDose::Gray(Metric::Ronto), 1.0e-27),
            (UnitAbsorbedDose::Gray(Metric::Ronna), 1.0e27),
            (UnitAbsorbedDose::Gray(Metric::Quetta), 1.0e30),
            (UnitAbsorbedDose::Gray(Metric::Quecto), 1.0e-30),
            (UnitAbsorbedDose::Gray(Metric::Atto), 1.0e-18),
            (UnitAbsorbedDose::Gray(Metric::Centi), 0.01),
            (UnitAbsorbedDose::Gray(Metric::Deca), 10.0),
            (UnitAbsorbedDose::Gray(Metric::Deci), 0.1),
            (UnitAbsorbedDose::Gray(Metric::Exa), 1.0e18),
            (UnitAbsorbedDose::Gray(Metric::Femto), 1.0e-15),
            (UnitAbsorbedDose::Gray(Metric::Giga), 1.0e9),
            (UnitAbsorbedDose::Gray(Metric::Hecto), 100.0),
            (UnitAbsorbedDose::Gray(Metric::Kilo), 1.0e3),
            (UnitAbsorbedDose::Gray(Metric::Mega), 1.0e6),
            (UnitAbsorbedDose::Gray(Metric::Micro), 1.0e-6),
            (UnitAbsorbedDose::Gray(Metric::Milli), 0.001),
            (UnitAbsorbedDose::Gray(Metric::Nano), 1.0e-9),
            (UnitAbsorbedDose::Gray(Metric::None), 1.0),
            (UnitAbsorbedDose::Gray(Metric::Peta), 1.0e15),
            (UnitAbsorbedDose::Gray(Metric::Pico), 1.0e-12),
            (UnitAbsorbedDose::Gray(Metric::Tera), 1.0e12),
            (UnitAbsorbedDose::Gray(Metric::Yocto), 1.0e-24),
            (UnitAbsorbedDose::Gray(Metric::Yotta), 1.0e24),
            (UnitAbsorbedDose::Gray(Metric::Zepto), 1.0e-21),
            (UnitAbsorbedDose::Gray(Metric::Zetta), 1.0e21),
            (UnitAbsorbedDose::Rad, 1.0),
            (UnitAbsorbedDose::Roentgen, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
