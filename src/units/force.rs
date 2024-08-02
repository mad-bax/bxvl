use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitForce};

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

impl From<UnitForce> for String {
    fn from(val: UnitForce) -> Self {
        val.to_string()
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
    use crate::units::{force::UnitForce, BaseUnit, Metric};

    /// Unit Force Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_force_base_comparison() {
        // Newtons are the base SI unit
        assert!(UnitForce::Newton(Metric::None).base() == 1.0);
        // Poundforce
        assert!(UnitForce::PoundForce.base() >= 4.448_221_615_260_5);
        assert!(UnitForce::PoundForce.base() < 4.448_221_615_260_6);
    }

    #[test]
    fn unit_force_to_string() {
        for i in [
            (UnitForce::Newton(Metric::Ronto), "rN"),
            (UnitForce::Newton(Metric::Ronna), "RN"),
            (UnitForce::Newton(Metric::Quetta), "QN"),
            (UnitForce::Newton(Metric::Quecto), "qN"),
            (UnitForce::Newton(Metric::Atto), "aN"),
            (UnitForce::Newton(Metric::Centi), "cN"),
            (UnitForce::Newton(Metric::Deca), "daN"),
            (UnitForce::Newton(Metric::Deci), "dN"),
            (UnitForce::Newton(Metric::Exa), "EN"),
            (UnitForce::Newton(Metric::Femto), "fN"),
            (UnitForce::Newton(Metric::Giga), "GN"),
            (UnitForce::Newton(Metric::Hecto), "hN"),
            (UnitForce::Newton(Metric::Kilo), "kN"),
            (UnitForce::Newton(Metric::Mega), "MN"),
            (UnitForce::Newton(Metric::Micro), "μN"),
            (UnitForce::Newton(Metric::Milli), "mN"),
            (UnitForce::Newton(Metric::Nano), "nN"),
            (UnitForce::Newton(Metric::None), "N"),
            (UnitForce::Newton(Metric::Peta), "PN"),
            (UnitForce::Newton(Metric::Pico), "pN"),
            (UnitForce::Newton(Metric::Tera), "TN"),
            (UnitForce::Newton(Metric::Yocto), "yN"),
            (UnitForce::Newton(Metric::Yotta), "YN"),
            (UnitForce::Newton(Metric::Zepto), "zN"),
            (UnitForce::Newton(Metric::Zetta), "ZN"),
            (UnitForce::PoundForce, "lbfr"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitForce::Newton(Metric::Ronto), Metric::Ronto),
            (UnitForce::Newton(Metric::Ronna), Metric::Ronna),
            (UnitForce::Newton(Metric::Quetta), Metric::Quetta),
            (UnitForce::Newton(Metric::Quecto), Metric::Quecto),
            (UnitForce::Newton(Metric::Atto), Metric::Atto),
            (UnitForce::Newton(Metric::Centi), Metric::Centi),
            (UnitForce::Newton(Metric::Deca), Metric::Deca),
            (UnitForce::Newton(Metric::Deci), Metric::Deci),
            (UnitForce::Newton(Metric::Exa), Metric::Exa),
            (UnitForce::Newton(Metric::Femto), Metric::Femto),
            (UnitForce::Newton(Metric::Giga), Metric::Giga),
            (UnitForce::Newton(Metric::Hecto), Metric::Hecto),
            (UnitForce::Newton(Metric::Kilo), Metric::Kilo),
            (UnitForce::Newton(Metric::Mega), Metric::Mega),
            (UnitForce::Newton(Metric::Micro), Metric::Micro),
            (UnitForce::Newton(Metric::Milli), Metric::Milli),
            (UnitForce::Newton(Metric::Nano), Metric::Nano),
            (UnitForce::Newton(Metric::None), Metric::None),
            (UnitForce::Newton(Metric::Peta), Metric::Peta),
            (UnitForce::Newton(Metric::Pico), Metric::Pico),
            (UnitForce::Newton(Metric::Tera), Metric::Tera),
            (UnitForce::Newton(Metric::Yocto), Metric::Yocto),
            (UnitForce::Newton(Metric::Yotta), Metric::Yotta),
            (UnitForce::Newton(Metric::Zepto), Metric::Zepto),
            (UnitForce::Newton(Metric::Zetta), Metric::Zetta),
            (UnitForce::PoundForce, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
        for i in [
            (UnitForce::Newton(Metric::Ronto), 1.0e-27),
            (UnitForce::Newton(Metric::Ronna), 1.0e27),
            (UnitForce::Newton(Metric::Quetta), 1.0e30),
            (UnitForce::Newton(Metric::Quecto), 1.0e-30),
            (UnitForce::Newton(Metric::Atto), 1.0e-18),
            (UnitForce::Newton(Metric::Centi), 0.01),
            (UnitForce::Newton(Metric::Deca), 10.0),
            (UnitForce::Newton(Metric::Deci), 0.1),
            (UnitForce::Newton(Metric::Exa), 1.0e18),
            (UnitForce::Newton(Metric::Femto), 1.0e-15),
            (UnitForce::Newton(Metric::Giga), 1.0e9),
            (UnitForce::Newton(Metric::Hecto), 100.0),
            (UnitForce::Newton(Metric::Kilo), 1.0e3),
            (UnitForce::Newton(Metric::Mega), 1.0e6),
            (UnitForce::Newton(Metric::Micro), 1.0e-6),
            (UnitForce::Newton(Metric::Milli), 0.001),
            (UnitForce::Newton(Metric::Nano), 1.0e-9),
            (UnitForce::Newton(Metric::None), 1.0),
            (UnitForce::Newton(Metric::Peta), 1.0e15),
            (UnitForce::Newton(Metric::Pico), 1.0e-12),
            (UnitForce::Newton(Metric::Tera), 1.0e12),
            (UnitForce::Newton(Metric::Yocto), 1.0e-24),
            (UnitForce::Newton(Metric::Yotta), 1.0e24),
            (UnitForce::Newton(Metric::Zepto), 1.0e-21),
            (UnitForce::Newton(Metric::Zetta), 1.0e21),
            (UnitForce::PoundForce, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
