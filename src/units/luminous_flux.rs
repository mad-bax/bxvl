use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitLuminousFlux};

impl Display for UnitLuminousFlux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}lm", self.get_metric().as_str())
    }
}

impl From<UnitLuminousFlux> for String {
    fn from(val: UnitLuminousFlux) -> Self {
        val.to_string()
    }
}

impl Convert<UnitLuminousFlux> for UnitLuminousFlux {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitLuminousFlux) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitLuminousFlux {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Lumen(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Lumen(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod luminous_flux_testing {
    use crate::units::{BaseUnit, Metric, UnitLuminousFlux};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitLuminousFlux::Lumen(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitLuminousFlux::Lumen(Metric::Ronto), "rlm"),
            (UnitLuminousFlux::Lumen(Metric::Ronna), "Rlm"),
            (UnitLuminousFlux::Lumen(Metric::Quetta), "Qlm"),
            (UnitLuminousFlux::Lumen(Metric::Quecto), "qlm"),
            (UnitLuminousFlux::Lumen(Metric::Atto), "alm"),
            (UnitLuminousFlux::Lumen(Metric::Centi), "clm"),
            (UnitLuminousFlux::Lumen(Metric::Deca), "dalm"),
            (UnitLuminousFlux::Lumen(Metric::Deci), "dlm"),
            (UnitLuminousFlux::Lumen(Metric::Exa), "Elm"),
            (UnitLuminousFlux::Lumen(Metric::Femto), "flm"),
            (UnitLuminousFlux::Lumen(Metric::Giga), "Glm"),
            (UnitLuminousFlux::Lumen(Metric::Hecto), "hlm"),
            (UnitLuminousFlux::Lumen(Metric::Kilo), "klm"),
            (UnitLuminousFlux::Lumen(Metric::Mega), "Mlm"),
            (UnitLuminousFlux::Lumen(Metric::Micro), "μlm"),
            (UnitLuminousFlux::Lumen(Metric::Milli), "mlm"),
            (UnitLuminousFlux::Lumen(Metric::Nano), "nlm"),
            (UnitLuminousFlux::Lumen(Metric::None), "lm"),
            (UnitLuminousFlux::Lumen(Metric::Peta), "Plm"),
            (UnitLuminousFlux::Lumen(Metric::Pico), "plm"),
            (UnitLuminousFlux::Lumen(Metric::Tera), "Tlm"),
            (UnitLuminousFlux::Lumen(Metric::Yocto), "ylm"),
            (UnitLuminousFlux::Lumen(Metric::Yotta), "Ylm"),
            (UnitLuminousFlux::Lumen(Metric::Zepto), "zlm"),
            (UnitLuminousFlux::Lumen(Metric::Zetta), "Zlm"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitLuminousFlux::Lumen(Metric::Ronto), Metric::Ronto),
            (UnitLuminousFlux::Lumen(Metric::Ronna), Metric::Ronna),
            (UnitLuminousFlux::Lumen(Metric::Quetta), Metric::Quetta),
            (UnitLuminousFlux::Lumen(Metric::Quecto), Metric::Quecto),
            (UnitLuminousFlux::Lumen(Metric::Atto), Metric::Atto),
            (UnitLuminousFlux::Lumen(Metric::Centi), Metric::Centi),
            (UnitLuminousFlux::Lumen(Metric::Deca), Metric::Deca),
            (UnitLuminousFlux::Lumen(Metric::Deci), Metric::Deci),
            (UnitLuminousFlux::Lumen(Metric::Exa), Metric::Exa),
            (UnitLuminousFlux::Lumen(Metric::Femto), Metric::Femto),
            (UnitLuminousFlux::Lumen(Metric::Giga), Metric::Giga),
            (UnitLuminousFlux::Lumen(Metric::Hecto), Metric::Hecto),
            (UnitLuminousFlux::Lumen(Metric::Kilo), Metric::Kilo),
            (UnitLuminousFlux::Lumen(Metric::Mega), Metric::Mega),
            (UnitLuminousFlux::Lumen(Metric::Micro), Metric::Micro),
            (UnitLuminousFlux::Lumen(Metric::Milli), Metric::Milli),
            (UnitLuminousFlux::Lumen(Metric::Nano), Metric::Nano),
            (UnitLuminousFlux::Lumen(Metric::None), Metric::None),
            (UnitLuminousFlux::Lumen(Metric::Peta), Metric::Peta),
            (UnitLuminousFlux::Lumen(Metric::Pico), Metric::Pico),
            (UnitLuminousFlux::Lumen(Metric::Tera), Metric::Tera),
            (UnitLuminousFlux::Lumen(Metric::Yocto), Metric::Yocto),
            (UnitLuminousFlux::Lumen(Metric::Yotta), Metric::Yotta),
            (UnitLuminousFlux::Lumen(Metric::Zepto), Metric::Zepto),
            (UnitLuminousFlux::Lumen(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitLuminousFlux::Lumen(Metric::Ronto), 1.0e-27),
            (UnitLuminousFlux::Lumen(Metric::Ronna), 1.0e27),
            (UnitLuminousFlux::Lumen(Metric::Quetta), 1.0e30),
            (UnitLuminousFlux::Lumen(Metric::Quecto), 1.0e-30),
            (UnitLuminousFlux::Lumen(Metric::Atto), 1.0e-18),
            (UnitLuminousFlux::Lumen(Metric::Centi), 0.01),
            (UnitLuminousFlux::Lumen(Metric::Deca), 10.0),
            (UnitLuminousFlux::Lumen(Metric::Deci), 0.1),
            (UnitLuminousFlux::Lumen(Metric::Exa), 1.0e18),
            (UnitLuminousFlux::Lumen(Metric::Femto), 1.0e-15),
            (UnitLuminousFlux::Lumen(Metric::Giga), 1.0e9),
            (UnitLuminousFlux::Lumen(Metric::Hecto), 100.0),
            (UnitLuminousFlux::Lumen(Metric::Kilo), 1.0e3),
            (UnitLuminousFlux::Lumen(Metric::Mega), 1.0e6),
            (UnitLuminousFlux::Lumen(Metric::Micro), 1.0e-6),
            (UnitLuminousFlux::Lumen(Metric::Milli), 0.001),
            (UnitLuminousFlux::Lumen(Metric::Nano), 1.0e-9),
            (UnitLuminousFlux::Lumen(Metric::None), 1.0),
            (UnitLuminousFlux::Lumen(Metric::Peta), 1.0e15),
            (UnitLuminousFlux::Lumen(Metric::Pico), 1.0e-12),
            (UnitLuminousFlux::Lumen(Metric::Tera), 1.0e12),
            (UnitLuminousFlux::Lumen(Metric::Yocto), 1.0e-24),
            (UnitLuminousFlux::Lumen(Metric::Yotta), 1.0e24),
            (UnitLuminousFlux::Lumen(Metric::Zepto), 1.0e-21),
            (UnitLuminousFlux::Lumen(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
