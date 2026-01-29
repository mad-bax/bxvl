use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitCatalyticActivity};

impl Display for UnitCatalyticActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kat", self.get_metric().as_str())
    }
}

impl From<UnitCatalyticActivity> for String {
    fn from(val: UnitCatalyticActivity) -> Self {
        val.to_string()
    }
}

impl Convert<UnitCatalyticActivity> for UnitCatalyticActivity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitCatalyticActivity) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitCatalyticActivity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Katal(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Katal(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod catalytic_activity_testing {
    use crate::units::{BaseUnit, Metric, UnitCatalyticActivity};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitCatalyticActivity::Katal(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitCatalyticActivity::Katal(Metric::Ronto), "rkat"),
            (UnitCatalyticActivity::Katal(Metric::Ronna), "Rkat"),
            (UnitCatalyticActivity::Katal(Metric::Quetta), "Qkat"),
            (UnitCatalyticActivity::Katal(Metric::Quecto), "qkat"),
            (UnitCatalyticActivity::Katal(Metric::Atto), "akat"),
            (UnitCatalyticActivity::Katal(Metric::Centi), "ckat"),
            (UnitCatalyticActivity::Katal(Metric::Deca), "dakat"),
            (UnitCatalyticActivity::Katal(Metric::Deci), "dkat"),
            (UnitCatalyticActivity::Katal(Metric::Exa), "Ekat"),
            (UnitCatalyticActivity::Katal(Metric::Femto), "fkat"),
            (UnitCatalyticActivity::Katal(Metric::Giga), "Gkat"),
            (UnitCatalyticActivity::Katal(Metric::Hecto), "hkat"),
            (UnitCatalyticActivity::Katal(Metric::Kilo), "kkat"),
            (UnitCatalyticActivity::Katal(Metric::Mega), "Mkat"),
            (UnitCatalyticActivity::Katal(Metric::Micro), "μkat"),
            (UnitCatalyticActivity::Katal(Metric::Milli), "mkat"),
            (UnitCatalyticActivity::Katal(Metric::Nano), "nkat"),
            (UnitCatalyticActivity::Katal(Metric::None), "kat"),
            (UnitCatalyticActivity::Katal(Metric::Peta), "Pkat"),
            (UnitCatalyticActivity::Katal(Metric::Pico), "pkat"),
            (UnitCatalyticActivity::Katal(Metric::Tera), "Tkat"),
            (UnitCatalyticActivity::Katal(Metric::Yocto), "ykat"),
            (UnitCatalyticActivity::Katal(Metric::Yotta), "Ykat"),
            (UnitCatalyticActivity::Katal(Metric::Zepto), "zkat"),
            (UnitCatalyticActivity::Katal(Metric::Zetta), "Zkat"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitCatalyticActivity::Katal(Metric::Ronto), Metric::Ronto),
            (UnitCatalyticActivity::Katal(Metric::Ronna), Metric::Ronna),
            (UnitCatalyticActivity::Katal(Metric::Quetta), Metric::Quetta),
            (UnitCatalyticActivity::Katal(Metric::Quecto), Metric::Quecto),
            (UnitCatalyticActivity::Katal(Metric::Atto), Metric::Atto),
            (UnitCatalyticActivity::Katal(Metric::Centi), Metric::Centi),
            (UnitCatalyticActivity::Katal(Metric::Deca), Metric::Deca),
            (UnitCatalyticActivity::Katal(Metric::Deci), Metric::Deci),
            (UnitCatalyticActivity::Katal(Metric::Exa), Metric::Exa),
            (UnitCatalyticActivity::Katal(Metric::Femto), Metric::Femto),
            (UnitCatalyticActivity::Katal(Metric::Giga), Metric::Giga),
            (UnitCatalyticActivity::Katal(Metric::Hecto), Metric::Hecto),
            (UnitCatalyticActivity::Katal(Metric::Kilo), Metric::Kilo),
            (UnitCatalyticActivity::Katal(Metric::Mega), Metric::Mega),
            (UnitCatalyticActivity::Katal(Metric::Micro), Metric::Micro),
            (UnitCatalyticActivity::Katal(Metric::Milli), Metric::Milli),
            (UnitCatalyticActivity::Katal(Metric::Nano), Metric::Nano),
            (UnitCatalyticActivity::Katal(Metric::None), Metric::None),
            (UnitCatalyticActivity::Katal(Metric::Peta), Metric::Peta),
            (UnitCatalyticActivity::Katal(Metric::Pico), Metric::Pico),
            (UnitCatalyticActivity::Katal(Metric::Tera), Metric::Tera),
            (UnitCatalyticActivity::Katal(Metric::Yocto), Metric::Yocto),
            (UnitCatalyticActivity::Katal(Metric::Yotta), Metric::Yotta),
            (UnitCatalyticActivity::Katal(Metric::Zepto), Metric::Zepto),
            (UnitCatalyticActivity::Katal(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitCatalyticActivity::Katal(Metric::Ronto), 1.0e-27),
            (UnitCatalyticActivity::Katal(Metric::Ronna), 1.0e27),
            (UnitCatalyticActivity::Katal(Metric::Quecto), 1.0e-30),
            (UnitCatalyticActivity::Katal(Metric::Quetta), 1.0e30),
            (UnitCatalyticActivity::Katal(Metric::Atto), 1.0e-18),
            (UnitCatalyticActivity::Katal(Metric::Centi), 0.01),
            (UnitCatalyticActivity::Katal(Metric::Deca), 10.0),
            (UnitCatalyticActivity::Katal(Metric::Deci), 0.1),
            (UnitCatalyticActivity::Katal(Metric::Exa), 1.0e18),
            (UnitCatalyticActivity::Katal(Metric::Femto), 1.0e-15),
            (UnitCatalyticActivity::Katal(Metric::Giga), 1.0e9),
            (UnitCatalyticActivity::Katal(Metric::Hecto), 100.0),
            (UnitCatalyticActivity::Katal(Metric::Kilo), 1.0e3),
            (UnitCatalyticActivity::Katal(Metric::Mega), 1.0e6),
            (UnitCatalyticActivity::Katal(Metric::Micro), 1.0e-6),
            (UnitCatalyticActivity::Katal(Metric::Milli), 0.001),
            (UnitCatalyticActivity::Katal(Metric::Nano), 1.0e-9),
            (UnitCatalyticActivity::Katal(Metric::None), 1.0),
            (UnitCatalyticActivity::Katal(Metric::Peta), 1.0e15),
            (UnitCatalyticActivity::Katal(Metric::Pico), 1.0e-12),
            (UnitCatalyticActivity::Katal(Metric::Tera), 1.0e12),
            (UnitCatalyticActivity::Katal(Metric::Yocto), 1.0e-24),
            (UnitCatalyticActivity::Katal(Metric::Yotta), 1.0e24),
            (UnitCatalyticActivity::Katal(Metric::Zepto), 1.0e-21),
            (UnitCatalyticActivity::Katal(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
