use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitIlluminance};

impl Display for UnitIlluminance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}lx", self.get_metric().as_str())
    }
}

impl From<UnitIlluminance> for String {
    fn from(val: UnitIlluminance) -> Self {
        val.to_string()
    }
}

impl Convert<UnitIlluminance> for UnitIlluminance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitIlluminance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitIlluminance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Lux(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Lux(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod illuminance_testing {
    use crate::units::{BaseUnit, Metric, UnitIlluminance};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitIlluminance::Lux(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitIlluminance::Lux(Metric::Ronto), "rlx"),
            (UnitIlluminance::Lux(Metric::Ronna), "Rlx"),
            (UnitIlluminance::Lux(Metric::Quetta), "Qlx"),
            (UnitIlluminance::Lux(Metric::Quecto), "qlx"),
            (UnitIlluminance::Lux(Metric::Atto), "alx"),
            (UnitIlluminance::Lux(Metric::Centi), "clx"),
            (UnitIlluminance::Lux(Metric::Deca), "dalx"),
            (UnitIlluminance::Lux(Metric::Deci), "dlx"),
            (UnitIlluminance::Lux(Metric::Exa), "Elx"),
            (UnitIlluminance::Lux(Metric::Femto), "flx"),
            (UnitIlluminance::Lux(Metric::Giga), "Glx"),
            (UnitIlluminance::Lux(Metric::Hecto), "hlx"),
            (UnitIlluminance::Lux(Metric::Kilo), "klx"),
            (UnitIlluminance::Lux(Metric::Mega), "Mlx"),
            (UnitIlluminance::Lux(Metric::Micro), "μlx"),
            (UnitIlluminance::Lux(Metric::Milli), "mlx"),
            (UnitIlluminance::Lux(Metric::Nano), "nlx"),
            (UnitIlluminance::Lux(Metric::None), "lx"),
            (UnitIlluminance::Lux(Metric::Peta), "Plx"),
            (UnitIlluminance::Lux(Metric::Pico), "plx"),
            (UnitIlluminance::Lux(Metric::Tera), "Tlx"),
            (UnitIlluminance::Lux(Metric::Yocto), "ylx"),
            (UnitIlluminance::Lux(Metric::Yotta), "Ylx"),
            (UnitIlluminance::Lux(Metric::Zepto), "zlx"),
            (UnitIlluminance::Lux(Metric::Zetta), "Zlx"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitIlluminance::Lux(Metric::Ronto), Metric::Ronto),
            (UnitIlluminance::Lux(Metric::Ronna), Metric::Ronna),
            (UnitIlluminance::Lux(Metric::Quetta), Metric::Quetta),
            (UnitIlluminance::Lux(Metric::Quecto), Metric::Quecto),
            (UnitIlluminance::Lux(Metric::Atto), Metric::Atto),
            (UnitIlluminance::Lux(Metric::Centi), Metric::Centi),
            (UnitIlluminance::Lux(Metric::Deca), Metric::Deca),
            (UnitIlluminance::Lux(Metric::Deci), Metric::Deci),
            (UnitIlluminance::Lux(Metric::Exa), Metric::Exa),
            (UnitIlluminance::Lux(Metric::Femto), Metric::Femto),
            (UnitIlluminance::Lux(Metric::Giga), Metric::Giga),
            (UnitIlluminance::Lux(Metric::Hecto), Metric::Hecto),
            (UnitIlluminance::Lux(Metric::Kilo), Metric::Kilo),
            (UnitIlluminance::Lux(Metric::Mega), Metric::Mega),
            (UnitIlluminance::Lux(Metric::Micro), Metric::Micro),
            (UnitIlluminance::Lux(Metric::Milli), Metric::Milli),
            (UnitIlluminance::Lux(Metric::Nano), Metric::Nano),
            (UnitIlluminance::Lux(Metric::None), Metric::None),
            (UnitIlluminance::Lux(Metric::Peta), Metric::Peta),
            (UnitIlluminance::Lux(Metric::Pico), Metric::Pico),
            (UnitIlluminance::Lux(Metric::Tera), Metric::Tera),
            (UnitIlluminance::Lux(Metric::Yocto), Metric::Yocto),
            (UnitIlluminance::Lux(Metric::Yotta), Metric::Yotta),
            (UnitIlluminance::Lux(Metric::Zepto), Metric::Zepto),
            (UnitIlluminance::Lux(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitIlluminance::Lux(Metric::Ronto), 1.0e-27),
            (UnitIlluminance::Lux(Metric::Ronna), 1.0e27),
            (UnitIlluminance::Lux(Metric::Quetta), 1.0e30),
            (UnitIlluminance::Lux(Metric::Quecto), 1.0e-30),
            (UnitIlluminance::Lux(Metric::Atto), 1.0e-18),
            (UnitIlluminance::Lux(Metric::Centi), 0.01),
            (UnitIlluminance::Lux(Metric::Deca), 10.0),
            (UnitIlluminance::Lux(Metric::Deci), 0.1),
            (UnitIlluminance::Lux(Metric::Exa), 1.0e18),
            (UnitIlluminance::Lux(Metric::Femto), 1.0e-15),
            (UnitIlluminance::Lux(Metric::Giga), 1.0e9),
            (UnitIlluminance::Lux(Metric::Hecto), 100.0),
            (UnitIlluminance::Lux(Metric::Kilo), 1.0e3),
            (UnitIlluminance::Lux(Metric::Mega), 1.0e6),
            (UnitIlluminance::Lux(Metric::Micro), 1.0e-6),
            (UnitIlluminance::Lux(Metric::Milli), 0.001),
            (UnitIlluminance::Lux(Metric::Nano), 1.0e-9),
            (UnitIlluminance::Lux(Metric::None), 1.0),
            (UnitIlluminance::Lux(Metric::Peta), 1.0e15),
            (UnitIlluminance::Lux(Metric::Pico), 1.0e-12),
            (UnitIlluminance::Lux(Metric::Tera), 1.0e12),
            (UnitIlluminance::Lux(Metric::Yocto), 1.0e-24),
            (UnitIlluminance::Lux(Metric::Yotta), 1.0e24),
            (UnitIlluminance::Lux(Metric::Zepto), 1.0e-21),
            (UnitIlluminance::Lux(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
