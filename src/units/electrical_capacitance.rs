use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricCapacitance};

impl Display for UnitElectricCapacitance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}F", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricCapacitance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricCapacitance> for UnitElectricCapacitance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricCapacitance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricCapacitance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Farad(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Farad(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod electrical_capacitance_testing {
    use crate::units::{BaseUnit, Metric, UnitElectricCapacitance};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitElectricCapacitance::Farad(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitElectricCapacitance::Farad(Metric::Atto), "aF"),
            (UnitElectricCapacitance::Farad(Metric::Centi), "cF"),
            (UnitElectricCapacitance::Farad(Metric::Deca), "daF"),
            (UnitElectricCapacitance::Farad(Metric::Deci), "dF"),
            (UnitElectricCapacitance::Farad(Metric::Exa), "EF"),
            (UnitElectricCapacitance::Farad(Metric::Femto), "fF"),
            (UnitElectricCapacitance::Farad(Metric::Giga), "GF"),
            (UnitElectricCapacitance::Farad(Metric::Hecto), "hF"),
            (UnitElectricCapacitance::Farad(Metric::Kilo), "kF"),
            (UnitElectricCapacitance::Farad(Metric::Mega), "MF"),
            (UnitElectricCapacitance::Farad(Metric::Micro), "μF"),
            (UnitElectricCapacitance::Farad(Metric::Milli), "mF"),
            (UnitElectricCapacitance::Farad(Metric::Nano), "nF"),
            (UnitElectricCapacitance::Farad(Metric::None), "F"),
            (UnitElectricCapacitance::Farad(Metric::Peta), "PF"),
            (UnitElectricCapacitance::Farad(Metric::Pico), "pF"),
            (UnitElectricCapacitance::Farad(Metric::Tera), "TF"),
            (UnitElectricCapacitance::Farad(Metric::Yocto), "yF"),
            (UnitElectricCapacitance::Farad(Metric::Yotta), "YF"),
            (UnitElectricCapacitance::Farad(Metric::Zepto), "zF"),
            (UnitElectricCapacitance::Farad(Metric::Zetta), "ZF"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitElectricCapacitance::Farad(Metric::Atto), Metric::Atto),
            (UnitElectricCapacitance::Farad(Metric::Centi), Metric::Centi),
            (UnitElectricCapacitance::Farad(Metric::Deca), Metric::Deca),
            (UnitElectricCapacitance::Farad(Metric::Deci), Metric::Deci),
            (UnitElectricCapacitance::Farad(Metric::Exa), Metric::Exa),
            (UnitElectricCapacitance::Farad(Metric::Femto), Metric::Femto),
            (UnitElectricCapacitance::Farad(Metric::Giga), Metric::Giga),
            (UnitElectricCapacitance::Farad(Metric::Hecto), Metric::Hecto),
            (UnitElectricCapacitance::Farad(Metric::Kilo), Metric::Kilo),
            (UnitElectricCapacitance::Farad(Metric::Mega), Metric::Mega),
            (UnitElectricCapacitance::Farad(Metric::Micro), Metric::Micro),
            (UnitElectricCapacitance::Farad(Metric::Milli), Metric::Milli),
            (UnitElectricCapacitance::Farad(Metric::Nano), Metric::Nano),
            (UnitElectricCapacitance::Farad(Metric::None), Metric::None),
            (UnitElectricCapacitance::Farad(Metric::Peta), Metric::Peta),
            (UnitElectricCapacitance::Farad(Metric::Pico), Metric::Pico),
            (UnitElectricCapacitance::Farad(Metric::Tera), Metric::Tera),
            (UnitElectricCapacitance::Farad(Metric::Yocto), Metric::Yocto),
            (UnitElectricCapacitance::Farad(Metric::Yotta), Metric::Yotta),
            (UnitElectricCapacitance::Farad(Metric::Zepto), Metric::Zepto),
            (UnitElectricCapacitance::Farad(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitElectricCapacitance::Farad(Metric::Atto), 1.0e-18),
            (UnitElectricCapacitance::Farad(Metric::Centi), 0.01),
            (UnitElectricCapacitance::Farad(Metric::Deca), 10.0),
            (UnitElectricCapacitance::Farad(Metric::Deci), 0.1),
            (UnitElectricCapacitance::Farad(Metric::Exa), 1.0e18),
            (UnitElectricCapacitance::Farad(Metric::Femto), 1.0e-15),
            (UnitElectricCapacitance::Farad(Metric::Giga), 1.0e9),
            (UnitElectricCapacitance::Farad(Metric::Hecto), 100.0),
            (UnitElectricCapacitance::Farad(Metric::Kilo), 1.0e3),
            (UnitElectricCapacitance::Farad(Metric::Mega), 1.0e6),
            (UnitElectricCapacitance::Farad(Metric::Micro), 1.0e-6),
            (UnitElectricCapacitance::Farad(Metric::Milli), 0.001),
            (UnitElectricCapacitance::Farad(Metric::Nano), 1.0e-9),
            (UnitElectricCapacitance::Farad(Metric::None), 1.0),
            (UnitElectricCapacitance::Farad(Metric::Peta), 1.0e15),
            (UnitElectricCapacitance::Farad(Metric::Pico), 1.0e-12),
            (UnitElectricCapacitance::Farad(Metric::Tera), 1.0e12),
            (UnitElectricCapacitance::Farad(Metric::Yocto), 1.0e-24),
            (UnitElectricCapacitance::Farad(Metric::Yotta), 1.0e24),
            (UnitElectricCapacitance::Farad(Metric::Zepto), 1.0e-21),
            (UnitElectricCapacitance::Farad(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
