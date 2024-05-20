use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitLuminousIntensity};

impl Display for UnitLuminousIntensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}cd", self.get_metric().as_str())
    }
}

impl Into<String> for UnitLuminousIntensity {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitLuminousIntensity> for UnitLuminousIntensity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitLuminousIntensity) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitLuminousIntensity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Candela(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Candela(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod luminous_intensity_testing {
    use crate::units::{BaseUnit, Metric, UnitLuminousIntensity};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitLuminousIntensity::Candela(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitLuminousIntensity::Candela(Metric::Atto), "acd"),
            (UnitLuminousIntensity::Candela(Metric::Centi), "ccd"),
            (UnitLuminousIntensity::Candela(Metric::Deca), "dacd"),
            (UnitLuminousIntensity::Candela(Metric::Deci), "dcd"),
            (UnitLuminousIntensity::Candela(Metric::Exa), "Ecd"),
            (UnitLuminousIntensity::Candela(Metric::Femto), "fcd"),
            (UnitLuminousIntensity::Candela(Metric::Giga), "Gcd"),
            (UnitLuminousIntensity::Candela(Metric::Hecto), "hcd"),
            (UnitLuminousIntensity::Candela(Metric::Kilo), "kcd"),
            (UnitLuminousIntensity::Candela(Metric::Mega), "Mcd"),
            (UnitLuminousIntensity::Candela(Metric::Micro), "μcd"),
            (UnitLuminousIntensity::Candela(Metric::Milli), "mcd"),
            (UnitLuminousIntensity::Candela(Metric::Nano), "ncd"),
            (UnitLuminousIntensity::Candela(Metric::None), "cd"),
            (UnitLuminousIntensity::Candela(Metric::Peta), "Pcd"),
            (UnitLuminousIntensity::Candela(Metric::Pico), "pcd"),
            (UnitLuminousIntensity::Candela(Metric::Tera), "Tcd"),
            (UnitLuminousIntensity::Candela(Metric::Yocto), "ycd"),
            (UnitLuminousIntensity::Candela(Metric::Yotta), "Ycd"),
            (UnitLuminousIntensity::Candela(Metric::Zepto), "zcd"),
            (UnitLuminousIntensity::Candela(Metric::Zetta), "Zcd"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitLuminousIntensity::Candela(Metric::Atto), Metric::Atto),
            (UnitLuminousIntensity::Candela(Metric::Centi), Metric::Centi),
            (UnitLuminousIntensity::Candela(Metric::Deca), Metric::Deca),
            (UnitLuminousIntensity::Candela(Metric::Deci), Metric::Deci),
            (UnitLuminousIntensity::Candela(Metric::Exa), Metric::Exa),
            (UnitLuminousIntensity::Candela(Metric::Femto), Metric::Femto),
            (UnitLuminousIntensity::Candela(Metric::Giga), Metric::Giga),
            (UnitLuminousIntensity::Candela(Metric::Hecto), Metric::Hecto),
            (UnitLuminousIntensity::Candela(Metric::Kilo), Metric::Kilo),
            (UnitLuminousIntensity::Candela(Metric::Mega), Metric::Mega),
            (UnitLuminousIntensity::Candela(Metric::Micro), Metric::Micro),
            (UnitLuminousIntensity::Candela(Metric::Milli), Metric::Milli),
            (UnitLuminousIntensity::Candela(Metric::Nano), Metric::Nano),
            (UnitLuminousIntensity::Candela(Metric::None), Metric::None),
            (UnitLuminousIntensity::Candela(Metric::Peta), Metric::Peta),
            (UnitLuminousIntensity::Candela(Metric::Pico), Metric::Pico),
            (UnitLuminousIntensity::Candela(Metric::Tera), Metric::Tera),
            (UnitLuminousIntensity::Candela(Metric::Yocto), Metric::Yocto),
            (UnitLuminousIntensity::Candela(Metric::Yotta), Metric::Yotta),
            (UnitLuminousIntensity::Candela(Metric::Zepto), Metric::Zepto),
            (UnitLuminousIntensity::Candela(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitLuminousIntensity::Candela(Metric::Atto), 1.0e-18),
            (UnitLuminousIntensity::Candela(Metric::Centi), 0.01),
            (UnitLuminousIntensity::Candela(Metric::Deca), 10.0),
            (UnitLuminousIntensity::Candela(Metric::Deci), 0.1),
            (UnitLuminousIntensity::Candela(Metric::Exa), 1.0e18),
            (UnitLuminousIntensity::Candela(Metric::Femto), 1.0e-15),
            (UnitLuminousIntensity::Candela(Metric::Giga), 1.0e9),
            (UnitLuminousIntensity::Candela(Metric::Hecto), 100.0),
            (UnitLuminousIntensity::Candela(Metric::Kilo), 1.0e3),
            (UnitLuminousIntensity::Candela(Metric::Mega), 1.0e6),
            (UnitLuminousIntensity::Candela(Metric::Micro), 1.0e-6),
            (UnitLuminousIntensity::Candela(Metric::Milli), 0.001),
            (UnitLuminousIntensity::Candela(Metric::Nano), 1.0e-9),
            (UnitLuminousIntensity::Candela(Metric::None), 1.0),
            (UnitLuminousIntensity::Candela(Metric::Peta), 1.0e15),
            (UnitLuminousIntensity::Candela(Metric::Pico), 1.0e-12),
            (UnitLuminousIntensity::Candela(Metric::Tera), 1.0e12),
            (UnitLuminousIntensity::Candela(Metric::Yocto), 1.0e-24),
            (UnitLuminousIntensity::Candela(Metric::Yotta), 1.0e24),
            (UnitLuminousIntensity::Candela(Metric::Zepto), 1.0e-21),
            (UnitLuminousIntensity::Candela(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
