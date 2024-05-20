use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricCharge};

impl Display for UnitElectricCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}C", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricCharge {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricCharge> for UnitElectricCharge {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricCharge) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricCharge {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Coulomb(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Coulomb(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod electrical_charge_testing {
    use crate::units::{BaseUnit, Metric, UnitElectricCharge};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitElectricCharge::Coulomb(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitElectricCharge::Coulomb(Metric::Atto), "aC"),
            (UnitElectricCharge::Coulomb(Metric::Centi), "cC"),
            (UnitElectricCharge::Coulomb(Metric::Deca), "daC"),
            (UnitElectricCharge::Coulomb(Metric::Deci), "dC"),
            (UnitElectricCharge::Coulomb(Metric::Exa), "EC"),
            (UnitElectricCharge::Coulomb(Metric::Femto), "fC"),
            (UnitElectricCharge::Coulomb(Metric::Giga), "GC"),
            (UnitElectricCharge::Coulomb(Metric::Hecto), "hC"),
            (UnitElectricCharge::Coulomb(Metric::Kilo), "kC"),
            (UnitElectricCharge::Coulomb(Metric::Mega), "MC"),
            (UnitElectricCharge::Coulomb(Metric::Micro), "μC"),
            (UnitElectricCharge::Coulomb(Metric::Milli), "mC"),
            (UnitElectricCharge::Coulomb(Metric::Nano), "nC"),
            (UnitElectricCharge::Coulomb(Metric::None), "C"),
            (UnitElectricCharge::Coulomb(Metric::Peta), "PC"),
            (UnitElectricCharge::Coulomb(Metric::Pico), "pC"),
            (UnitElectricCharge::Coulomb(Metric::Tera), "TC"),
            (UnitElectricCharge::Coulomb(Metric::Yocto), "yC"),
            (UnitElectricCharge::Coulomb(Metric::Yotta), "YC"),
            (UnitElectricCharge::Coulomb(Metric::Zepto), "zC"),
            (UnitElectricCharge::Coulomb(Metric::Zetta), "ZC"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitElectricCharge::Coulomb(Metric::Atto), Metric::Atto),
            (UnitElectricCharge::Coulomb(Metric::Centi), Metric::Centi),
            (UnitElectricCharge::Coulomb(Metric::Deca), Metric::Deca),
            (UnitElectricCharge::Coulomb(Metric::Deci), Metric::Deci),
            (UnitElectricCharge::Coulomb(Metric::Exa), Metric::Exa),
            (UnitElectricCharge::Coulomb(Metric::Femto), Metric::Femto),
            (UnitElectricCharge::Coulomb(Metric::Giga), Metric::Giga),
            (UnitElectricCharge::Coulomb(Metric::Hecto), Metric::Hecto),
            (UnitElectricCharge::Coulomb(Metric::Kilo), Metric::Kilo),
            (UnitElectricCharge::Coulomb(Metric::Mega), Metric::Mega),
            (UnitElectricCharge::Coulomb(Metric::Micro), Metric::Micro),
            (UnitElectricCharge::Coulomb(Metric::Milli), Metric::Milli),
            (UnitElectricCharge::Coulomb(Metric::Nano), Metric::Nano),
            (UnitElectricCharge::Coulomb(Metric::None), Metric::None),
            (UnitElectricCharge::Coulomb(Metric::Peta), Metric::Peta),
            (UnitElectricCharge::Coulomb(Metric::Pico), Metric::Pico),
            (UnitElectricCharge::Coulomb(Metric::Tera), Metric::Tera),
            (UnitElectricCharge::Coulomb(Metric::Yocto), Metric::Yocto),
            (UnitElectricCharge::Coulomb(Metric::Yotta), Metric::Yotta),
            (UnitElectricCharge::Coulomb(Metric::Zepto), Metric::Zepto),
            (UnitElectricCharge::Coulomb(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitElectricCharge::Coulomb(Metric::Atto), 1.0e-18),
            (UnitElectricCharge::Coulomb(Metric::Centi), 0.01),
            (UnitElectricCharge::Coulomb(Metric::Deca), 10.0),
            (UnitElectricCharge::Coulomb(Metric::Deci), 0.1),
            (UnitElectricCharge::Coulomb(Metric::Exa), 1.0e18),
            (UnitElectricCharge::Coulomb(Metric::Femto), 1.0e-15),
            (UnitElectricCharge::Coulomb(Metric::Giga), 1.0e9),
            (UnitElectricCharge::Coulomb(Metric::Hecto), 100.0),
            (UnitElectricCharge::Coulomb(Metric::Kilo), 1.0e3),
            (UnitElectricCharge::Coulomb(Metric::Mega), 1.0e6),
            (UnitElectricCharge::Coulomb(Metric::Micro), 1.0e-6),
            (UnitElectricCharge::Coulomb(Metric::Milli), 0.001),
            (UnitElectricCharge::Coulomb(Metric::Nano), 1.0e-9),
            (UnitElectricCharge::Coulomb(Metric::None), 1.0),
            (UnitElectricCharge::Coulomb(Metric::Peta), 1.0e15),
            (UnitElectricCharge::Coulomb(Metric::Pico), 1.0e-12),
            (UnitElectricCharge::Coulomb(Metric::Tera), 1.0e12),
            (UnitElectricCharge::Coulomb(Metric::Yocto), 1.0e-24),
            (UnitElectricCharge::Coulomb(Metric::Yotta), 1.0e24),
            (UnitElectricCharge::Coulomb(Metric::Zepto), 1.0e-21),
            (UnitElectricCharge::Coulomb(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
