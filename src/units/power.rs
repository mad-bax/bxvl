use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitPower};

impl Display for UnitPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}W", self.get_metric().as_str())
    }
}

impl Into<String> for UnitPower {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitPower> for UnitPower {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitPower) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitPower {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Watt(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Watt(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod power_testing {
    use crate::units::{BaseUnit, Metric, UnitPower};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitPower::Watt(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitPower::Watt(Metric::Atto), "aW"),
            (UnitPower::Watt(Metric::Centi), "cW"),
            (UnitPower::Watt(Metric::Deca), "daW"),
            (UnitPower::Watt(Metric::Deci), "dW"),
            (UnitPower::Watt(Metric::Exa), "EW"),
            (UnitPower::Watt(Metric::Femto), "fW"),
            (UnitPower::Watt(Metric::Giga), "GW"),
            (UnitPower::Watt(Metric::Hecto), "hW"),
            (UnitPower::Watt(Metric::Kilo), "kW"),
            (UnitPower::Watt(Metric::Mega), "MW"),
            (UnitPower::Watt(Metric::Micro), "μW"),
            (UnitPower::Watt(Metric::Milli), "mW"),
            (UnitPower::Watt(Metric::Nano), "nW"),
            (UnitPower::Watt(Metric::None), "W"),
            (UnitPower::Watt(Metric::Peta), "PW"),
            (UnitPower::Watt(Metric::Pico), "pW"),
            (UnitPower::Watt(Metric::Tera), "TW"),
            (UnitPower::Watt(Metric::Yocto), "yW"),
            (UnitPower::Watt(Metric::Yotta), "YW"),
            (UnitPower::Watt(Metric::Zepto), "zW"),
            (UnitPower::Watt(Metric::Zetta), "ZW"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitPower::Watt(Metric::Atto), Metric::Atto),
            (UnitPower::Watt(Metric::Centi), Metric::Centi),
            (UnitPower::Watt(Metric::Deca), Metric::Deca),
            (UnitPower::Watt(Metric::Deci), Metric::Deci),
            (UnitPower::Watt(Metric::Exa), Metric::Exa),
            (UnitPower::Watt(Metric::Femto), Metric::Femto),
            (UnitPower::Watt(Metric::Giga), Metric::Giga),
            (UnitPower::Watt(Metric::Hecto), Metric::Hecto),
            (UnitPower::Watt(Metric::Kilo), Metric::Kilo),
            (UnitPower::Watt(Metric::Mega), Metric::Mega),
            (UnitPower::Watt(Metric::Micro), Metric::Micro),
            (UnitPower::Watt(Metric::Milli), Metric::Milli),
            (UnitPower::Watt(Metric::Nano), Metric::Nano),
            (UnitPower::Watt(Metric::None), Metric::None),
            (UnitPower::Watt(Metric::Peta), Metric::Peta),
            (UnitPower::Watt(Metric::Pico), Metric::Pico),
            (UnitPower::Watt(Metric::Tera), Metric::Tera),
            (UnitPower::Watt(Metric::Yocto), Metric::Yocto),
            (UnitPower::Watt(Metric::Yotta), Metric::Yotta),
            (UnitPower::Watt(Metric::Zepto), Metric::Zepto),
            (UnitPower::Watt(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitPower::Watt(Metric::Atto), 0.000000000000000001),
            (UnitPower::Watt(Metric::Centi), 0.01),
            (UnitPower::Watt(Metric::Deca), 10.0),
            (UnitPower::Watt(Metric::Deci), 0.1),
            (UnitPower::Watt(Metric::Exa), 1000000000000000000.0),
            (UnitPower::Watt(Metric::Femto), 0.000000000000001),
            (UnitPower::Watt(Metric::Giga), 1000000000.0),
            (UnitPower::Watt(Metric::Hecto), 100.0),
            (UnitPower::Watt(Metric::Kilo), 1000.0),
            (UnitPower::Watt(Metric::Mega), 1000000.0),
            (UnitPower::Watt(Metric::Micro), 0.000001),
            (UnitPower::Watt(Metric::Milli), 0.001),
            (UnitPower::Watt(Metric::Nano), 0.000000001),
            (UnitPower::Watt(Metric::None), 1.0),
            (UnitPower::Watt(Metric::Peta), 1000000000000000.0),
            (UnitPower::Watt(Metric::Pico), 0.000000000001),
            (UnitPower::Watt(Metric::Tera), 1000000000000.0),
            (UnitPower::Watt(Metric::Yocto), 0.000000000000000000000001),
            (UnitPower::Watt(Metric::Yotta), 1000000000000000000000000.0),
            (UnitPower::Watt(Metric::Zepto), 0.000000000000000000001),
            (UnitPower::Watt(Metric::Zetta), 1000000000000000000000.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
