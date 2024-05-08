use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricInductance};

impl Display for UnitElectricInductance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}H", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricInductance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricInductance> for UnitElectricInductance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricInductance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricInductance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Henry(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Henry(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod electrical_inductance_testing {
    use crate::units::{BaseUnit, Metric, UnitElectricInductance};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitElectricInductance::Henry(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitElectricInductance::Henry(Metric::Atto), "aH"),
            (UnitElectricInductance::Henry(Metric::Centi), "cH"),
            (UnitElectricInductance::Henry(Metric::Deca), "daH"),
            (UnitElectricInductance::Henry(Metric::Deci), "dH"),
            (UnitElectricInductance::Henry(Metric::Exa), "EH"),
            (UnitElectricInductance::Henry(Metric::Femto), "fH"),
            (UnitElectricInductance::Henry(Metric::Giga), "GH"),
            (UnitElectricInductance::Henry(Metric::Hecto), "hH"),
            (UnitElectricInductance::Henry(Metric::Kilo), "kH"),
            (UnitElectricInductance::Henry(Metric::Mega), "MH"),
            (UnitElectricInductance::Henry(Metric::Micro), "μH"),
            (UnitElectricInductance::Henry(Metric::Milli), "mH"),
            (UnitElectricInductance::Henry(Metric::Nano), "nH"),
            (UnitElectricInductance::Henry(Metric::None), "H"),
            (UnitElectricInductance::Henry(Metric::Peta), "PH"),
            (UnitElectricInductance::Henry(Metric::Pico), "pH"),
            (UnitElectricInductance::Henry(Metric::Tera), "TH"),
            (UnitElectricInductance::Henry(Metric::Yocto), "yH"),
            (UnitElectricInductance::Henry(Metric::Yotta), "YH"),
            (UnitElectricInductance::Henry(Metric::Zepto), "zH"),
            (UnitElectricInductance::Henry(Metric::Zetta), "ZH"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitElectricInductance::Henry(Metric::Atto), Metric::Atto),
            (UnitElectricInductance::Henry(Metric::Centi), Metric::Centi),
            (UnitElectricInductance::Henry(Metric::Deca), Metric::Deca),
            (UnitElectricInductance::Henry(Metric::Deci), Metric::Deci),
            (UnitElectricInductance::Henry(Metric::Exa), Metric::Exa),
            (UnitElectricInductance::Henry(Metric::Femto), Metric::Femto),
            (UnitElectricInductance::Henry(Metric::Giga), Metric::Giga),
            (UnitElectricInductance::Henry(Metric::Hecto), Metric::Hecto),
            (UnitElectricInductance::Henry(Metric::Kilo), Metric::Kilo),
            (UnitElectricInductance::Henry(Metric::Mega), Metric::Mega),
            (UnitElectricInductance::Henry(Metric::Micro), Metric::Micro),
            (UnitElectricInductance::Henry(Metric::Milli), Metric::Milli),
            (UnitElectricInductance::Henry(Metric::Nano), Metric::Nano),
            (UnitElectricInductance::Henry(Metric::None), Metric::None),
            (UnitElectricInductance::Henry(Metric::Peta), Metric::Peta),
            (UnitElectricInductance::Henry(Metric::Pico), Metric::Pico),
            (UnitElectricInductance::Henry(Metric::Tera), Metric::Tera),
            (UnitElectricInductance::Henry(Metric::Yocto), Metric::Yocto),
            (UnitElectricInductance::Henry(Metric::Yotta), Metric::Yotta),
            (UnitElectricInductance::Henry(Metric::Zepto), Metric::Zepto),
            (UnitElectricInductance::Henry(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (
                UnitElectricInductance::Henry(Metric::Atto),
                0.000000000000000001,
            ),
            (UnitElectricInductance::Henry(Metric::Centi), 0.01),
            (UnitElectricInductance::Henry(Metric::Deca), 10.0),
            (UnitElectricInductance::Henry(Metric::Deci), 0.1),
            (
                UnitElectricInductance::Henry(Metric::Exa),
                1000000000000000000.0,
            ),
            (
                UnitElectricInductance::Henry(Metric::Femto),
                0.000000000000001,
            ),
            (UnitElectricInductance::Henry(Metric::Giga), 1000000000.0),
            (UnitElectricInductance::Henry(Metric::Hecto), 100.0),
            (UnitElectricInductance::Henry(Metric::Kilo), 1000.0),
            (UnitElectricInductance::Henry(Metric::Mega), 1000000.0),
            (UnitElectricInductance::Henry(Metric::Micro), 0.000001),
            (UnitElectricInductance::Henry(Metric::Milli), 0.001),
            (UnitElectricInductance::Henry(Metric::Nano), 0.000000001),
            (UnitElectricInductance::Henry(Metric::None), 1.0),
            (
                UnitElectricInductance::Henry(Metric::Peta),
                1000000000000000.0,
            ),
            (UnitElectricInductance::Henry(Metric::Pico), 0.000000000001),
            (UnitElectricInductance::Henry(Metric::Tera), 1000000000000.0),
            (
                UnitElectricInductance::Henry(Metric::Yocto),
                0.000000000000000000000001,
            ),
            (
                UnitElectricInductance::Henry(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (
                UnitElectricInductance::Henry(Metric::Zepto),
                0.000000000000000000001,
            ),
            (
                UnitElectricInductance::Henry(Metric::Zetta),
                1000000000000000000000.0,
            ),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
