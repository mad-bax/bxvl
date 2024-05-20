use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitMagneticFluxDensity};

impl Display for UnitMagneticFluxDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}T", self.get_metric().as_str())
    }
}

impl Into<String> for UnitMagneticFluxDensity {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitMagneticFluxDensity> for UnitMagneticFluxDensity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitMagneticFluxDensity) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitMagneticFluxDensity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Tesla(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Tesla(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod magnetic_flux_density_testing {
    use crate::units::{BaseUnit, Metric, UnitMagneticFluxDensity};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitMagneticFluxDensity::Tesla(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitMagneticFluxDensity::Tesla(Metric::Atto), "aT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Centi), "cT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Deca), "daT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Deci), "dT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Exa), "ET"),
            (UnitMagneticFluxDensity::Tesla(Metric::Femto), "fT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Giga), "GT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Hecto), "hT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Kilo), "kT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Mega), "MT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Micro), "μT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Milli), "mT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Nano), "nT"),
            (UnitMagneticFluxDensity::Tesla(Metric::None), "T"),
            (UnitMagneticFluxDensity::Tesla(Metric::Peta), "PT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Pico), "pT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Tera), "TT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Yocto), "yT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Yotta), "YT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Zepto), "zT"),
            (UnitMagneticFluxDensity::Tesla(Metric::Zetta), "ZT"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitMagneticFluxDensity::Tesla(Metric::Atto), Metric::Atto),
            (UnitMagneticFluxDensity::Tesla(Metric::Centi), Metric::Centi),
            (UnitMagneticFluxDensity::Tesla(Metric::Deca), Metric::Deca),
            (UnitMagneticFluxDensity::Tesla(Metric::Deci), Metric::Deci),
            (UnitMagneticFluxDensity::Tesla(Metric::Exa), Metric::Exa),
            (UnitMagneticFluxDensity::Tesla(Metric::Femto), Metric::Femto),
            (UnitMagneticFluxDensity::Tesla(Metric::Giga), Metric::Giga),
            (UnitMagneticFluxDensity::Tesla(Metric::Hecto), Metric::Hecto),
            (UnitMagneticFluxDensity::Tesla(Metric::Kilo), Metric::Kilo),
            (UnitMagneticFluxDensity::Tesla(Metric::Mega), Metric::Mega),
            (UnitMagneticFluxDensity::Tesla(Metric::Micro), Metric::Micro),
            (UnitMagneticFluxDensity::Tesla(Metric::Milli), Metric::Milli),
            (UnitMagneticFluxDensity::Tesla(Metric::Nano), Metric::Nano),
            (UnitMagneticFluxDensity::Tesla(Metric::None), Metric::None),
            (UnitMagneticFluxDensity::Tesla(Metric::Peta), Metric::Peta),
            (UnitMagneticFluxDensity::Tesla(Metric::Pico), Metric::Pico),
            (UnitMagneticFluxDensity::Tesla(Metric::Tera), Metric::Tera),
            (UnitMagneticFluxDensity::Tesla(Metric::Yocto), Metric::Yocto),
            (UnitMagneticFluxDensity::Tesla(Metric::Yotta), Metric::Yotta),
            (UnitMagneticFluxDensity::Tesla(Metric::Zepto), Metric::Zepto),
            (UnitMagneticFluxDensity::Tesla(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitMagneticFluxDensity::Tesla(Metric::Atto), 1.0e-18),
            (UnitMagneticFluxDensity::Tesla(Metric::Centi), 0.01),
            (UnitMagneticFluxDensity::Tesla(Metric::Deca), 10.0),
            (UnitMagneticFluxDensity::Tesla(Metric::Deci), 0.1),
            (UnitMagneticFluxDensity::Tesla(Metric::Exa), 1.0e18),
            (UnitMagneticFluxDensity::Tesla(Metric::Femto), 1.0e-15),
            (UnitMagneticFluxDensity::Tesla(Metric::Giga), 1.0e9),
            (UnitMagneticFluxDensity::Tesla(Metric::Hecto), 100.0),
            (UnitMagneticFluxDensity::Tesla(Metric::Kilo), 1.0e3),
            (UnitMagneticFluxDensity::Tesla(Metric::Mega), 1.0e6),
            (UnitMagneticFluxDensity::Tesla(Metric::Micro), 1.0e-6),
            (UnitMagneticFluxDensity::Tesla(Metric::Milli), 0.001),
            (UnitMagneticFluxDensity::Tesla(Metric::Nano), 1.0e-9),
            (UnitMagneticFluxDensity::Tesla(Metric::None), 1.0),
            (UnitMagneticFluxDensity::Tesla(Metric::Peta), 1.0e15),
            (UnitMagneticFluxDensity::Tesla(Metric::Pico), 1.0e-12),
            (UnitMagneticFluxDensity::Tesla(Metric::Tera), 1.0e12),
            (UnitMagneticFluxDensity::Tesla(Metric::Yocto), 1.0e-24),
            (UnitMagneticFluxDensity::Tesla(Metric::Yotta), 1.0e24),
            (UnitMagneticFluxDensity::Tesla(Metric::Zepto), 1.0e-21),
            (UnitMagneticFluxDensity::Tesla(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
