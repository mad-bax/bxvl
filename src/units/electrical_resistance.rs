use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricResistance};

impl Display for UnitElectricResistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Ω", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricResistance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricResistance> for UnitElectricResistance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricResistance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricResistance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Ohm(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Ohm(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod electrical_resistance_testing {
    use crate::units::{BaseUnit, Metric, UnitElectricResistance};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitElectricResistance::Ohm(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitElectricResistance::Ohm(Metric::Atto), "aΩ"),
            (UnitElectricResistance::Ohm(Metric::Centi), "cΩ"),
            (UnitElectricResistance::Ohm(Metric::Deca), "daΩ"),
            (UnitElectricResistance::Ohm(Metric::Deci), "dΩ"),
            (UnitElectricResistance::Ohm(Metric::Exa), "EΩ"),
            (UnitElectricResistance::Ohm(Metric::Femto), "fΩ"),
            (UnitElectricResistance::Ohm(Metric::Giga), "GΩ"),
            (UnitElectricResistance::Ohm(Metric::Hecto), "hΩ"),
            (UnitElectricResistance::Ohm(Metric::Kilo), "kΩ"),
            (UnitElectricResistance::Ohm(Metric::Mega), "MΩ"),
            (UnitElectricResistance::Ohm(Metric::Micro), "μΩ"),
            (UnitElectricResistance::Ohm(Metric::Milli), "mΩ"),
            (UnitElectricResistance::Ohm(Metric::Nano), "nΩ"),
            (UnitElectricResistance::Ohm(Metric::None), "Ω"),
            (UnitElectricResistance::Ohm(Metric::Peta), "PΩ"),
            (UnitElectricResistance::Ohm(Metric::Pico), "pΩ"),
            (UnitElectricResistance::Ohm(Metric::Tera), "TΩ"),
            (UnitElectricResistance::Ohm(Metric::Yocto), "yΩ"),
            (UnitElectricResistance::Ohm(Metric::Yotta), "YΩ"),
            (UnitElectricResistance::Ohm(Metric::Zepto), "zΩ"),
            (UnitElectricResistance::Ohm(Metric::Zetta), "ZΩ"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitElectricResistance::Ohm(Metric::Atto), Metric::Atto),
            (UnitElectricResistance::Ohm(Metric::Centi), Metric::Centi),
            (UnitElectricResistance::Ohm(Metric::Deca), Metric::Deca),
            (UnitElectricResistance::Ohm(Metric::Deci), Metric::Deci),
            (UnitElectricResistance::Ohm(Metric::Exa), Metric::Exa),
            (UnitElectricResistance::Ohm(Metric::Femto), Metric::Femto),
            (UnitElectricResistance::Ohm(Metric::Giga), Metric::Giga),
            (UnitElectricResistance::Ohm(Metric::Hecto), Metric::Hecto),
            (UnitElectricResistance::Ohm(Metric::Kilo), Metric::Kilo),
            (UnitElectricResistance::Ohm(Metric::Mega), Metric::Mega),
            (UnitElectricResistance::Ohm(Metric::Micro), Metric::Micro),
            (UnitElectricResistance::Ohm(Metric::Milli), Metric::Milli),
            (UnitElectricResistance::Ohm(Metric::Nano), Metric::Nano),
            (UnitElectricResistance::Ohm(Metric::None), Metric::None),
            (UnitElectricResistance::Ohm(Metric::Peta), Metric::Peta),
            (UnitElectricResistance::Ohm(Metric::Pico), Metric::Pico),
            (UnitElectricResistance::Ohm(Metric::Tera), Metric::Tera),
            (UnitElectricResistance::Ohm(Metric::Yocto), Metric::Yocto),
            (UnitElectricResistance::Ohm(Metric::Yotta), Metric::Yotta),
            (UnitElectricResistance::Ohm(Metric::Zepto), Metric::Zepto),
            (UnitElectricResistance::Ohm(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (
                UnitElectricResistance::Ohm(Metric::Atto),
                0.000000000000000001,
            ),
            (UnitElectricResistance::Ohm(Metric::Centi), 0.01),
            (UnitElectricResistance::Ohm(Metric::Deca), 10.0),
            (UnitElectricResistance::Ohm(Metric::Deci), 0.1),
            (
                UnitElectricResistance::Ohm(Metric::Exa),
                1000000000000000000.0,
            ),
            (
                UnitElectricResistance::Ohm(Metric::Femto),
                0.000000000000001,
            ),
            (UnitElectricResistance::Ohm(Metric::Giga), 1000000000.0),
            (UnitElectricResistance::Ohm(Metric::Hecto), 100.0),
            (UnitElectricResistance::Ohm(Metric::Kilo), 1000.0),
            (UnitElectricResistance::Ohm(Metric::Mega), 1000000.0),
            (UnitElectricResistance::Ohm(Metric::Micro), 0.000001),
            (UnitElectricResistance::Ohm(Metric::Milli), 0.001),
            (UnitElectricResistance::Ohm(Metric::Nano), 0.000000001),
            (UnitElectricResistance::Ohm(Metric::None), 1.0),
            (
                UnitElectricResistance::Ohm(Metric::Peta),
                1000000000000000.0,
            ),
            (UnitElectricResistance::Ohm(Metric::Pico), 0.000000000001),
            (UnitElectricResistance::Ohm(Metric::Tera), 1000000000000.0),
            (
                UnitElectricResistance::Ohm(Metric::Yocto),
                0.000000000000000000000001,
            ),
            (
                UnitElectricResistance::Ohm(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (
                UnitElectricResistance::Ohm(Metric::Zepto),
                0.000000000000000000001,
            ),
            (
                UnitElectricResistance::Ohm(Metric::Zetta),
                1000000000000000000000.0,
            ),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
