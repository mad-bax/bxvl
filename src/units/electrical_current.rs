use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricCurrent};

impl Display for UnitElectricCurrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}A", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricCurrent {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricCurrent> for UnitElectricCurrent {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricCurrent) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricCurrent {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Ampere(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Ampere(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod electrical_current_testing {
    use crate::units::{BaseUnit, Metric, UnitElectricCurrent};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitElectricCurrent::Ampere(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitElectricCurrent::Ampere(Metric::Atto), "aA"),
            (UnitElectricCurrent::Ampere(Metric::Centi), "cA"),
            (UnitElectricCurrent::Ampere(Metric::Deca), "daA"),
            (UnitElectricCurrent::Ampere(Metric::Deci), "dA"),
            (UnitElectricCurrent::Ampere(Metric::Exa), "EA"),
            (UnitElectricCurrent::Ampere(Metric::Femto), "fA"),
            (UnitElectricCurrent::Ampere(Metric::Giga), "GA"),
            (UnitElectricCurrent::Ampere(Metric::Hecto), "hA"),
            (UnitElectricCurrent::Ampere(Metric::Kilo), "kA"),
            (UnitElectricCurrent::Ampere(Metric::Mega), "MA"),
            (UnitElectricCurrent::Ampere(Metric::Micro), "μA"),
            (UnitElectricCurrent::Ampere(Metric::Milli), "mA"),
            (UnitElectricCurrent::Ampere(Metric::Nano), "nA"),
            (UnitElectricCurrent::Ampere(Metric::None), "A"),
            (UnitElectricCurrent::Ampere(Metric::Peta), "PA"),
            (UnitElectricCurrent::Ampere(Metric::Pico), "pA"),
            (UnitElectricCurrent::Ampere(Metric::Tera), "TA"),
            (UnitElectricCurrent::Ampere(Metric::Yocto), "yA"),
            (UnitElectricCurrent::Ampere(Metric::Yotta), "YA"),
            (UnitElectricCurrent::Ampere(Metric::Zepto), "zA"),
            (UnitElectricCurrent::Ampere(Metric::Zetta), "ZA"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitElectricCurrent::Ampere(Metric::Atto), Metric::Atto),
            (UnitElectricCurrent::Ampere(Metric::Centi), Metric::Centi),
            (UnitElectricCurrent::Ampere(Metric::Deca), Metric::Deca),
            (UnitElectricCurrent::Ampere(Metric::Deci), Metric::Deci),
            (UnitElectricCurrent::Ampere(Metric::Exa), Metric::Exa),
            (UnitElectricCurrent::Ampere(Metric::Femto), Metric::Femto),
            (UnitElectricCurrent::Ampere(Metric::Giga), Metric::Giga),
            (UnitElectricCurrent::Ampere(Metric::Hecto), Metric::Hecto),
            (UnitElectricCurrent::Ampere(Metric::Kilo), Metric::Kilo),
            (UnitElectricCurrent::Ampere(Metric::Mega), Metric::Mega),
            (UnitElectricCurrent::Ampere(Metric::Micro), Metric::Micro),
            (UnitElectricCurrent::Ampere(Metric::Milli), Metric::Milli),
            (UnitElectricCurrent::Ampere(Metric::Nano), Metric::Nano),
            (UnitElectricCurrent::Ampere(Metric::None), Metric::None),
            (UnitElectricCurrent::Ampere(Metric::Peta), Metric::Peta),
            (UnitElectricCurrent::Ampere(Metric::Pico), Metric::Pico),
            (UnitElectricCurrent::Ampere(Metric::Tera), Metric::Tera),
            (UnitElectricCurrent::Ampere(Metric::Yocto), Metric::Yocto),
            (UnitElectricCurrent::Ampere(Metric::Yotta), Metric::Yotta),
            (UnitElectricCurrent::Ampere(Metric::Zepto), Metric::Zepto),
            (UnitElectricCurrent::Ampere(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (
                UnitElectricCurrent::Ampere(Metric::Atto),
                0.000000000000000001,
            ),
            (UnitElectricCurrent::Ampere(Metric::Centi), 0.01),
            (UnitElectricCurrent::Ampere(Metric::Deca), 10.0),
            (UnitElectricCurrent::Ampere(Metric::Deci), 0.1),
            (
                UnitElectricCurrent::Ampere(Metric::Exa),
                1000000000000000000.0,
            ),
            (
                UnitElectricCurrent::Ampere(Metric::Femto),
                0.000000000000001,
            ),
            (UnitElectricCurrent::Ampere(Metric::Giga), 1000000000.0),
            (UnitElectricCurrent::Ampere(Metric::Hecto), 100.0),
            (UnitElectricCurrent::Ampere(Metric::Kilo), 1000.0),
            (UnitElectricCurrent::Ampere(Metric::Mega), 1000000.0),
            (UnitElectricCurrent::Ampere(Metric::Micro), 0.000001),
            (UnitElectricCurrent::Ampere(Metric::Milli), 0.001),
            (UnitElectricCurrent::Ampere(Metric::Nano), 0.000000001),
            (UnitElectricCurrent::Ampere(Metric::None), 1.0),
            (
                UnitElectricCurrent::Ampere(Metric::Peta),
                1000000000000000.0,
            ),
            (UnitElectricCurrent::Ampere(Metric::Pico), 0.000000000001),
            (UnitElectricCurrent::Ampere(Metric::Tera), 1000000000000.0),
            (
                UnitElectricCurrent::Ampere(Metric::Yocto),
                0.000000000000000000000001,
            ),
            (
                UnitElectricCurrent::Ampere(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (
                UnitElectricCurrent::Ampere(Metric::Zepto),
                0.000000000000000000001,
            ),
            (
                UnitElectricCurrent::Ampere(Metric::Zetta),
                1000000000000000000000.0,
            ),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
