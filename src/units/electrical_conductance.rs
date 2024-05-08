use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricConductance};

impl Display for UnitElectricConductance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}S", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricConductance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricConductance> for UnitElectricConductance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricConductance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricConductance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Siemens(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Siemens(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod electrical_conductance_testing {
    use crate::units::{BaseUnit, Metric, UnitElectricConductance};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitElectricConductance::Siemens(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitElectricConductance::Siemens(Metric::Atto), "aS"),
            (UnitElectricConductance::Siemens(Metric::Centi), "cS"),
            (UnitElectricConductance::Siemens(Metric::Deca), "daS"),
            (UnitElectricConductance::Siemens(Metric::Deci), "dS"),
            (UnitElectricConductance::Siemens(Metric::Exa), "ES"),
            (UnitElectricConductance::Siemens(Metric::Femto), "fS"),
            (UnitElectricConductance::Siemens(Metric::Giga), "GS"),
            (UnitElectricConductance::Siemens(Metric::Hecto), "hS"),
            (UnitElectricConductance::Siemens(Metric::Kilo), "kS"),
            (UnitElectricConductance::Siemens(Metric::Mega), "MS"),
            (UnitElectricConductance::Siemens(Metric::Micro), "μS"),
            (UnitElectricConductance::Siemens(Metric::Milli), "mS"),
            (UnitElectricConductance::Siemens(Metric::Nano), "nS"),
            (UnitElectricConductance::Siemens(Metric::None), "S"),
            (UnitElectricConductance::Siemens(Metric::Peta), "PS"),
            (UnitElectricConductance::Siemens(Metric::Pico), "pS"),
            (UnitElectricConductance::Siemens(Metric::Tera), "TS"),
            (UnitElectricConductance::Siemens(Metric::Yocto), "yS"),
            (UnitElectricConductance::Siemens(Metric::Yotta), "YS"),
            (UnitElectricConductance::Siemens(Metric::Zepto), "zS"),
            (UnitElectricConductance::Siemens(Metric::Zetta), "ZS"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitElectricConductance::Siemens(Metric::Atto), Metric::Atto),
            (
                UnitElectricConductance::Siemens(Metric::Centi),
                Metric::Centi,
            ),
            (UnitElectricConductance::Siemens(Metric::Deca), Metric::Deca),
            (UnitElectricConductance::Siemens(Metric::Deci), Metric::Deci),
            (UnitElectricConductance::Siemens(Metric::Exa), Metric::Exa),
            (
                UnitElectricConductance::Siemens(Metric::Femto),
                Metric::Femto,
            ),
            (UnitElectricConductance::Siemens(Metric::Giga), Metric::Giga),
            (
                UnitElectricConductance::Siemens(Metric::Hecto),
                Metric::Hecto,
            ),
            (UnitElectricConductance::Siemens(Metric::Kilo), Metric::Kilo),
            (UnitElectricConductance::Siemens(Metric::Mega), Metric::Mega),
            (
                UnitElectricConductance::Siemens(Metric::Micro),
                Metric::Micro,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Milli),
                Metric::Milli,
            ),
            (UnitElectricConductance::Siemens(Metric::Nano), Metric::Nano),
            (UnitElectricConductance::Siemens(Metric::None), Metric::None),
            (UnitElectricConductance::Siemens(Metric::Peta), Metric::Peta),
            (UnitElectricConductance::Siemens(Metric::Pico), Metric::Pico),
            (UnitElectricConductance::Siemens(Metric::Tera), Metric::Tera),
            (
                UnitElectricConductance::Siemens(Metric::Yocto),
                Metric::Yocto,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Yotta),
                Metric::Yotta,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Zepto),
                Metric::Zepto,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Zetta),
                Metric::Zetta,
            ),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (
                UnitElectricConductance::Siemens(Metric::Atto),
                0.000000000000000001,
            ),
            (UnitElectricConductance::Siemens(Metric::Centi), 0.01),
            (UnitElectricConductance::Siemens(Metric::Deca), 10.0),
            (UnitElectricConductance::Siemens(Metric::Deci), 0.1),
            (
                UnitElectricConductance::Siemens(Metric::Exa),
                1000000000000000000.0,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Femto),
                0.000000000000001,
            ),
            (UnitElectricConductance::Siemens(Metric::Giga), 1000000000.0),
            (UnitElectricConductance::Siemens(Metric::Hecto), 100.0),
            (UnitElectricConductance::Siemens(Metric::Kilo), 1000.0),
            (UnitElectricConductance::Siemens(Metric::Mega), 1000000.0),
            (UnitElectricConductance::Siemens(Metric::Micro), 0.000001),
            (UnitElectricConductance::Siemens(Metric::Milli), 0.001),
            (UnitElectricConductance::Siemens(Metric::Nano), 0.000000001),
            (UnitElectricConductance::Siemens(Metric::None), 1.0),
            (
                UnitElectricConductance::Siemens(Metric::Peta),
                1000000000000000.0,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Pico),
                0.000000000001,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Tera),
                1000000000000.0,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Yocto),
                0.000000000000000000000001,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Zepto),
                0.000000000000000000001,
            ),
            (
                UnitElectricConductance::Siemens(Metric::Zetta),
                1000000000000000000000.0,
            ),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
