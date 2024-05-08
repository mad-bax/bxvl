use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricPotential};

impl Display for UnitElectricPotential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}V", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricPotential {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricPotential> for UnitElectricPotential {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricPotential) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricPotential {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Volt(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Volt(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod electrical_potential_testing {
    use crate::units::{BaseUnit, Metric, UnitElectricPotential};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitElectricPotential::Volt(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitElectricPotential::Volt(Metric::Atto), "aV"),
            (UnitElectricPotential::Volt(Metric::Centi), "cV"),
            (UnitElectricPotential::Volt(Metric::Deca), "daV"),
            (UnitElectricPotential::Volt(Metric::Deci), "dV"),
            (UnitElectricPotential::Volt(Metric::Exa), "EV"),
            (UnitElectricPotential::Volt(Metric::Femto), "fV"),
            (UnitElectricPotential::Volt(Metric::Giga), "GV"),
            (UnitElectricPotential::Volt(Metric::Hecto), "hV"),
            (UnitElectricPotential::Volt(Metric::Kilo), "kV"),
            (UnitElectricPotential::Volt(Metric::Mega), "MV"),
            (UnitElectricPotential::Volt(Metric::Micro), "μV"),
            (UnitElectricPotential::Volt(Metric::Milli), "mV"),
            (UnitElectricPotential::Volt(Metric::Nano), "nV"),
            (UnitElectricPotential::Volt(Metric::None), "V"),
            (UnitElectricPotential::Volt(Metric::Peta), "PV"),
            (UnitElectricPotential::Volt(Metric::Pico), "pV"),
            (UnitElectricPotential::Volt(Metric::Tera), "TV"),
            (UnitElectricPotential::Volt(Metric::Yocto), "yV"),
            (UnitElectricPotential::Volt(Metric::Yotta), "YV"),
            (UnitElectricPotential::Volt(Metric::Zepto), "zV"),
            (UnitElectricPotential::Volt(Metric::Zetta), "ZV"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitElectricPotential::Volt(Metric::Atto), Metric::Atto),
            (UnitElectricPotential::Volt(Metric::Centi), Metric::Centi),
            (UnitElectricPotential::Volt(Metric::Deca), Metric::Deca),
            (UnitElectricPotential::Volt(Metric::Deci), Metric::Deci),
            (UnitElectricPotential::Volt(Metric::Exa), Metric::Exa),
            (UnitElectricPotential::Volt(Metric::Femto), Metric::Femto),
            (UnitElectricPotential::Volt(Metric::Giga), Metric::Giga),
            (UnitElectricPotential::Volt(Metric::Hecto), Metric::Hecto),
            (UnitElectricPotential::Volt(Metric::Kilo), Metric::Kilo),
            (UnitElectricPotential::Volt(Metric::Mega), Metric::Mega),
            (UnitElectricPotential::Volt(Metric::Micro), Metric::Micro),
            (UnitElectricPotential::Volt(Metric::Milli), Metric::Milli),
            (UnitElectricPotential::Volt(Metric::Nano), Metric::Nano),
            (UnitElectricPotential::Volt(Metric::None), Metric::None),
            (UnitElectricPotential::Volt(Metric::Peta), Metric::Peta),
            (UnitElectricPotential::Volt(Metric::Pico), Metric::Pico),
            (UnitElectricPotential::Volt(Metric::Tera), Metric::Tera),
            (UnitElectricPotential::Volt(Metric::Yocto), Metric::Yocto),
            (UnitElectricPotential::Volt(Metric::Yotta), Metric::Yotta),
            (UnitElectricPotential::Volt(Metric::Zepto), Metric::Zepto),
            (UnitElectricPotential::Volt(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (
                UnitElectricPotential::Volt(Metric::Atto),
                0.000000000000000001,
            ),
            (UnitElectricPotential::Volt(Metric::Centi), 0.01),
            (UnitElectricPotential::Volt(Metric::Deca), 10.0),
            (UnitElectricPotential::Volt(Metric::Deci), 0.1),
            (
                UnitElectricPotential::Volt(Metric::Exa),
                1000000000000000000.0,
            ),
            (
                UnitElectricPotential::Volt(Metric::Femto),
                0.000000000000001,
            ),
            (UnitElectricPotential::Volt(Metric::Giga), 1000000000.0),
            (UnitElectricPotential::Volt(Metric::Hecto), 100.0),
            (UnitElectricPotential::Volt(Metric::Kilo), 1000.0),
            (UnitElectricPotential::Volt(Metric::Mega), 1000000.0),
            (UnitElectricPotential::Volt(Metric::Micro), 0.000001),
            (UnitElectricPotential::Volt(Metric::Milli), 0.001),
            (UnitElectricPotential::Volt(Metric::Nano), 0.000000001),
            (UnitElectricPotential::Volt(Metric::None), 1.0),
            (
                UnitElectricPotential::Volt(Metric::Peta),
                1000000000000000.0,
            ),
            (UnitElectricPotential::Volt(Metric::Pico), 0.000000000001),
            (UnitElectricPotential::Volt(Metric::Tera), 1000000000000.0),
            (
                UnitElectricPotential::Volt(Metric::Yocto),
                0.000000000000000000000001,
            ),
            (
                UnitElectricPotential::Volt(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (
                UnitElectricPotential::Volt(Metric::Zepto),
                0.000000000000000000001,
            ),
            (
                UnitElectricPotential::Volt(Metric::Zetta),
                1000000000000000000000.0,
            ),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
