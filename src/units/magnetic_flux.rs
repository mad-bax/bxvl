use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitMagneticFlux};

impl Display for UnitMagneticFlux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Wb", self.get_metric().as_str())
    }
}

impl From<UnitMagneticFlux> for String {
    fn from(val: UnitMagneticFlux) -> Self {
        val.to_string()
    }
}

impl Convert<UnitMagneticFlux> for UnitMagneticFlux {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitMagneticFlux) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitMagneticFlux {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Weber(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Weber(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod magnetic_flux_testing {
    use crate::units::{BaseUnit, Metric, UnitMagneticFlux};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitMagneticFlux::Weber(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitMagneticFlux::Weber(Metric::Ronto), "rWb"),
            (UnitMagneticFlux::Weber(Metric::Ronna), "RWb"),
            (UnitMagneticFlux::Weber(Metric::Quetta), "QWb"),
            (UnitMagneticFlux::Weber(Metric::Quecto), "qWb"),
            (UnitMagneticFlux::Weber(Metric::Atto), "aWb"),
            (UnitMagneticFlux::Weber(Metric::Centi), "cWb"),
            (UnitMagneticFlux::Weber(Metric::Deca), "daWb"),
            (UnitMagneticFlux::Weber(Metric::Deci), "dWb"),
            (UnitMagneticFlux::Weber(Metric::Exa), "EWb"),
            (UnitMagneticFlux::Weber(Metric::Femto), "fWb"),
            (UnitMagneticFlux::Weber(Metric::Giga), "GWb"),
            (UnitMagneticFlux::Weber(Metric::Hecto), "hWb"),
            (UnitMagneticFlux::Weber(Metric::Kilo), "kWb"),
            (UnitMagneticFlux::Weber(Metric::Mega), "MWb"),
            (UnitMagneticFlux::Weber(Metric::Micro), "μWb"),
            (UnitMagneticFlux::Weber(Metric::Milli), "mWb"),
            (UnitMagneticFlux::Weber(Metric::Nano), "nWb"),
            (UnitMagneticFlux::Weber(Metric::None), "Wb"),
            (UnitMagneticFlux::Weber(Metric::Peta), "PWb"),
            (UnitMagneticFlux::Weber(Metric::Pico), "pWb"),
            (UnitMagneticFlux::Weber(Metric::Tera), "TWb"),
            (UnitMagneticFlux::Weber(Metric::Yocto), "yWb"),
            (UnitMagneticFlux::Weber(Metric::Yotta), "YWb"),
            (UnitMagneticFlux::Weber(Metric::Zepto), "zWb"),
            (UnitMagneticFlux::Weber(Metric::Zetta), "ZWb"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitMagneticFlux::Weber(Metric::Ronto), Metric::Ronto),
            (UnitMagneticFlux::Weber(Metric::Ronna), Metric::Ronna),
            (UnitMagneticFlux::Weber(Metric::Quetta), Metric::Quetta),
            (UnitMagneticFlux::Weber(Metric::Quecto), Metric::Quecto),
            (UnitMagneticFlux::Weber(Metric::Atto), Metric::Atto),
            (UnitMagneticFlux::Weber(Metric::Centi), Metric::Centi),
            (UnitMagneticFlux::Weber(Metric::Deca), Metric::Deca),
            (UnitMagneticFlux::Weber(Metric::Deci), Metric::Deci),
            (UnitMagneticFlux::Weber(Metric::Exa), Metric::Exa),
            (UnitMagneticFlux::Weber(Metric::Femto), Metric::Femto),
            (UnitMagneticFlux::Weber(Metric::Giga), Metric::Giga),
            (UnitMagneticFlux::Weber(Metric::Hecto), Metric::Hecto),
            (UnitMagneticFlux::Weber(Metric::Kilo), Metric::Kilo),
            (UnitMagneticFlux::Weber(Metric::Mega), Metric::Mega),
            (UnitMagneticFlux::Weber(Metric::Micro), Metric::Micro),
            (UnitMagneticFlux::Weber(Metric::Milli), Metric::Milli),
            (UnitMagneticFlux::Weber(Metric::Nano), Metric::Nano),
            (UnitMagneticFlux::Weber(Metric::None), Metric::None),
            (UnitMagneticFlux::Weber(Metric::Peta), Metric::Peta),
            (UnitMagneticFlux::Weber(Metric::Pico), Metric::Pico),
            (UnitMagneticFlux::Weber(Metric::Tera), Metric::Tera),
            (UnitMagneticFlux::Weber(Metric::Yocto), Metric::Yocto),
            (UnitMagneticFlux::Weber(Metric::Yotta), Metric::Yotta),
            (UnitMagneticFlux::Weber(Metric::Zepto), Metric::Zepto),
            (UnitMagneticFlux::Weber(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitMagneticFlux::Weber(Metric::Ronto), 1.0e-27),
            (UnitMagneticFlux::Weber(Metric::Ronna), 1.0e27),
            (UnitMagneticFlux::Weber(Metric::Quetta), 1.0e30),
            (UnitMagneticFlux::Weber(Metric::Quecto), 1.0e-30),
            (UnitMagneticFlux::Weber(Metric::Atto), 1.0e-18),
            (UnitMagneticFlux::Weber(Metric::Centi), 0.01),
            (UnitMagneticFlux::Weber(Metric::Deca), 10.0),
            (UnitMagneticFlux::Weber(Metric::Deci), 0.1),
            (UnitMagneticFlux::Weber(Metric::Exa), 1.0e18),
            (UnitMagneticFlux::Weber(Metric::Femto), 1.0e-15),
            (UnitMagneticFlux::Weber(Metric::Giga), 1.0e9),
            (UnitMagneticFlux::Weber(Metric::Hecto), 100.0),
            (UnitMagneticFlux::Weber(Metric::Kilo), 1.0e3),
            (UnitMagneticFlux::Weber(Metric::Mega), 1.0e6),
            (UnitMagneticFlux::Weber(Metric::Micro), 1.0e-6),
            (UnitMagneticFlux::Weber(Metric::Milli), 0.001),
            (UnitMagneticFlux::Weber(Metric::Nano), 1.0e-9),
            (UnitMagneticFlux::Weber(Metric::None), 1.0),
            (UnitMagneticFlux::Weber(Metric::Peta), 1.0e15),
            (UnitMagneticFlux::Weber(Metric::Pico), 1.0e-12),
            (UnitMagneticFlux::Weber(Metric::Tera), 1.0e12),
            (UnitMagneticFlux::Weber(Metric::Yocto), 1.0e-24),
            (UnitMagneticFlux::Weber(Metric::Yotta), 1.0e24),
            (UnitMagneticFlux::Weber(Metric::Zepto), 1.0e-21),
            (UnitMagneticFlux::Weber(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
