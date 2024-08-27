use std::fmt::Display;

use crate::consts::PW_HPWR_TO_W;

use super::{BaseUnit, Convert, Metric, UnitPower};

impl Display for UnitPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Watt(m) => format!("{}W", m.as_str()),
                Self::Horsepower => "hp".into(),
            }
        )
    }
}

impl From<UnitPower> for String {
    fn from(val: UnitPower) -> Self {
        val.to_string()
    }
}

impl Convert<UnitPower> for UnitPower {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitPower) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitPower {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Watt(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Watt(m) => *m,
            _ => Metric::None,
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Watt(_) => 1.0,
            Self::Horsepower => PW_HPWR_TO_W,
        }
    }
}

#[cfg(test)]
mod power_testing {
    use crate::units::{BaseUnit, Metric, UnitPower};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitPower::Watt(Metric::None).base() == 1.0);
        assert!(UnitPower::Horsepower.base() == 745.699872);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitPower::Watt(Metric::Ronto), "rW"),
            (UnitPower::Watt(Metric::Ronna), "RW"),
            (UnitPower::Watt(Metric::Quetta), "QW"),
            (UnitPower::Watt(Metric::Quecto), "qW"),
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

        assert_eq!(UnitPower::Horsepower.to_string(), "hp");
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitPower::Watt(Metric::Ronto), Metric::Ronto),
            (UnitPower::Watt(Metric::Ronna), Metric::Ronna),
            (UnitPower::Watt(Metric::Quetta), Metric::Quetta),
            (UnitPower::Watt(Metric::Quecto), Metric::Quecto),
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

        assert_eq!(UnitPower::Horsepower.get_metric(), Metric::None);

        for i in [
            (UnitPower::Watt(Metric::Ronto), 1.0e-27),
            (UnitPower::Watt(Metric::Ronna), 1.0e27),
            (UnitPower::Watt(Metric::Quetta), 1.0e30),
            (UnitPower::Watt(Metric::Quecto), 1.0e-30),
            (UnitPower::Watt(Metric::Atto), 1.0e-18),
            (UnitPower::Watt(Metric::Centi), 0.01),
            (UnitPower::Watt(Metric::Deca), 10.0),
            (UnitPower::Watt(Metric::Deci), 0.1),
            (UnitPower::Watt(Metric::Exa), 1.0e18),
            (UnitPower::Watt(Metric::Femto), 1.0e-15),
            (UnitPower::Watt(Metric::Giga), 1.0e9),
            (UnitPower::Watt(Metric::Hecto), 100.0),
            (UnitPower::Watt(Metric::Kilo), 1.0e3),
            (UnitPower::Watt(Metric::Mega), 1.0e6),
            (UnitPower::Watt(Metric::Micro), 1.0e-6),
            (UnitPower::Watt(Metric::Milli), 0.001),
            (UnitPower::Watt(Metric::Nano), 1.0e-9),
            (UnitPower::Watt(Metric::None), 1.0),
            (UnitPower::Watt(Metric::Peta), 1.0e15),
            (UnitPower::Watt(Metric::Pico), 1.0e-12),
            (UnitPower::Watt(Metric::Tera), 1.0e12),
            (UnitPower::Watt(Metric::Yocto), 1.0e-24),
            (UnitPower::Watt(Metric::Yotta), 1.0e24),
            (UnitPower::Watt(Metric::Zepto), 1.0e-21),
            (UnitPower::Watt(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }

        assert_eq!(UnitPower::Horsepower.scale(), 1.0);
    }
}
