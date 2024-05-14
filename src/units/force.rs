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
            (UnitForce::Newton(Metric::Atto), 0.000000000000000001),
            (UnitForce::Newton(Metric::Centi), 0.01),
            (UnitForce::Newton(Metric::Deca), 10.0),
            (UnitForce::Newton(Metric::Deci), 0.1),
            (UnitForce::Newton(Metric::Exa), 1000000000000000000.0),
            (UnitForce::Newton(Metric::Femto), 0.000000000000001),
            (UnitForce::Newton(Metric::Giga), 1000000000.0),
            (UnitForce::Newton(Metric::Hecto), 100.0),
            (UnitForce::Newton(Metric::Kilo), 1000.0),
            (UnitForce::Newton(Metric::Mega), 1000000.0),
            (UnitForce::Newton(Metric::Micro), 0.000001),
            (UnitForce::Newton(Metric::Milli), 0.001),
            (UnitForce::Newton(Metric::Nano), 0.000000001),
            (UnitForce::Newton(Metric::None), 1.0),
            (UnitForce::Newton(Metric::Peta), 1000000000000000.0),
            (UnitForce::Newton(Metric::Pico), 0.000000000001),
            (UnitForce::Newton(Metric::Tera), 1000000000000.0),
            (UnitForce::Newton(Metric::Yocto), 0.000000000000000000000001),
            (
                UnitForce::Newton(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (UnitForce::Newton(Metric::Zepto), 0.000000000000000000001),
            (UnitForce::Newton(Metric::Zetta), 1000000000000000000000.0),
            (UnitForce::PoundForce, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
