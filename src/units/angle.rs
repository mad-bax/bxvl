use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitAngle};

impl Display for UnitAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Radian(Metric::Milli) => "mil".into(),
                Self::Radian(m) => format!("{}rad", m.as_str()),
                Self::Degree => "°".into(),
                Self::Moa => "moa".into(),
            }
        )
    }
}

impl Into<String> for UnitAngle {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitAngle> for UnitAngle {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitAngle {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Radian(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Radian(_) => 1.0,
            Self::Degree => constants::ANGLE_DEG_TO_RAD,
            Self::Moa => constants::ANGLE_MOA_TO_RAD,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Radian(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod angle_testing {
    use crate::units::{BaseUnit, Metric, UnitAngle};

    /// Unit Angle Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_angle_base_comparison() {
        // Radians are the base SI unit
        assert!(UnitAngle::Radian(Metric::None).base() == 1.0);
        // Degrees
        assert!(UnitAngle::Degree.base() >= 0.017_453_292_51);
        assert!(UnitAngle::Degree.base() <= 0.017_453_292_52);
        // Minute of Angle
        assert!(UnitAngle::Moa.base() >= 0.000_290_888_208_665);
        assert!(UnitAngle::Moa.base() < 0.000_290_888_208_666);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitAngle::Radian(Metric::Atto), "arad"),
            (UnitAngle::Radian(Metric::Centi), "crad"),
            (UnitAngle::Radian(Metric::Deca), "darad"),
            (UnitAngle::Radian(Metric::Deci), "drad"),
            (UnitAngle::Radian(Metric::Exa), "Erad"),
            (UnitAngle::Radian(Metric::Femto), "frad"),
            (UnitAngle::Radian(Metric::Giga), "Grad"),
            (UnitAngle::Radian(Metric::Hecto), "hrad"),
            (UnitAngle::Radian(Metric::Kilo), "krad"),
            (UnitAngle::Radian(Metric::Mega), "Mrad"),
            (UnitAngle::Radian(Metric::Micro), "μrad"),
            (UnitAngle::Radian(Metric::Milli), "mil"),
            (UnitAngle::Radian(Metric::Nano), "nrad"),
            (UnitAngle::Radian(Metric::None), "rad"),
            (UnitAngle::Radian(Metric::Peta), "Prad"),
            (UnitAngle::Radian(Metric::Pico), "prad"),
            (UnitAngle::Radian(Metric::Tera), "Trad"),
            (UnitAngle::Radian(Metric::Yocto), "yrad"),
            (UnitAngle::Radian(Metric::Yotta), "Yrad"),
            (UnitAngle::Radian(Metric::Zepto), "zrad"),
            (UnitAngle::Radian(Metric::Zetta), "Zrad"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitAngle::Radian(Metric::Atto), Metric::Atto),
            (UnitAngle::Radian(Metric::Centi), Metric::Centi),
            (UnitAngle::Radian(Metric::Deca), Metric::Deca),
            (UnitAngle::Radian(Metric::Deci), Metric::Deci),
            (UnitAngle::Radian(Metric::Exa), Metric::Exa),
            (UnitAngle::Radian(Metric::Femto), Metric::Femto),
            (UnitAngle::Radian(Metric::Giga), Metric::Giga),
            (UnitAngle::Radian(Metric::Hecto), Metric::Hecto),
            (UnitAngle::Radian(Metric::Kilo), Metric::Kilo),
            (UnitAngle::Radian(Metric::Mega), Metric::Mega),
            (UnitAngle::Radian(Metric::Micro), Metric::Micro),
            (UnitAngle::Radian(Metric::Milli), Metric::Milli),
            (UnitAngle::Radian(Metric::Nano), Metric::Nano),
            (UnitAngle::Radian(Metric::None), Metric::None),
            (UnitAngle::Radian(Metric::Peta), Metric::Peta),
            (UnitAngle::Radian(Metric::Pico), Metric::Pico),
            (UnitAngle::Radian(Metric::Tera), Metric::Tera),
            (UnitAngle::Radian(Metric::Yocto), Metric::Yocto),
            (UnitAngle::Radian(Metric::Yotta), Metric::Yotta),
            (UnitAngle::Radian(Metric::Zepto), Metric::Zepto),
            (UnitAngle::Radian(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitAngle::Radian(Metric::Atto), 0.000000000000000001),
            (UnitAngle::Radian(Metric::Centi), 0.01),
            (UnitAngle::Radian(Metric::Deca), 10.0),
            (UnitAngle::Radian(Metric::Deci), 0.1),
            (UnitAngle::Radian(Metric::Exa), 1000000000000000000.0),
            (UnitAngle::Radian(Metric::Femto), 0.000000000000001),
            (UnitAngle::Radian(Metric::Giga), 1000000000.0),
            (UnitAngle::Radian(Metric::Hecto), 100.0),
            (UnitAngle::Radian(Metric::Kilo), 1000.0),
            (UnitAngle::Radian(Metric::Mega), 1000000.0),
            (UnitAngle::Radian(Metric::Micro), 0.000001),
            (UnitAngle::Radian(Metric::Milli), 0.001),
            (UnitAngle::Radian(Metric::Nano), 0.000000001),
            (UnitAngle::Radian(Metric::None), 1.0),
            (UnitAngle::Radian(Metric::Peta), 1000000000000000.0),
            (UnitAngle::Radian(Metric::Pico), 0.000000000001),
            (UnitAngle::Radian(Metric::Tera), 1000000000000.0),
            (UnitAngle::Radian(Metric::Yocto), 0.000000000000000000000001),
            (
                UnitAngle::Radian(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (UnitAngle::Radian(Metric::Zepto), 0.000000000000000000001),
            (UnitAngle::Radian(Metric::Zetta), 1000000000000000000000.0),
            (UnitAngle::Degree, 1.0),
            (UnitAngle::Moa, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
