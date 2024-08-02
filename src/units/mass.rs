use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitMass};

impl Display for UnitMass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Gram(m) => format!("{}{}", m.as_str(), "g"),
                Self::Grain => "gr".into(),
                Self::Ounce => "oz".into(),
                Self::Pound => "lb".into(),
            }
        )
    }
}

impl From<UnitMass> for String {
    fn from(val: UnitMass) -> Self {
        val.to_string()
    }
}

impl Convert<UnitMass> for UnitMass {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitMass) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitMass {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Gram(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Gram(_) => 1.0,
            Self::Grain => constants::MASS_GR_TO_G,
            Self::Ounce => constants::MASS_OZ_TO_G,
            Self::Pound => constants::MASS_LB_TO_G,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Gram(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod mass_testing {
    use crate::units::{BaseUnit, Convert, Metric, UnitMass};

    #[test]
    fn unit_mass_to_string() {
        for i in [
            (UnitMass::Grain, "gr"),
            (UnitMass::Ounce, "oz"),
            (UnitMass::Pound, "lb"),
            (UnitMass::Gram(Metric::Ronto), "rg"),
            (UnitMass::Gram(Metric::Ronna), "Rg"),
            (UnitMass::Gram(Metric::Quetta), "Qg"),
            (UnitMass::Gram(Metric::Quecto), "qg"),
            (UnitMass::Gram(Metric::Atto), "ag"),
            (UnitMass::Gram(Metric::Centi), "cg"),
            (UnitMass::Gram(Metric::Deca), "dag"),
            (UnitMass::Gram(Metric::Deci), "dg"),
            (UnitMass::Gram(Metric::Exa), "Eg"),
            (UnitMass::Gram(Metric::Femto), "fg"),
            (UnitMass::Gram(Metric::Giga), "Gg"),
            (UnitMass::Gram(Metric::Hecto), "hg"),
            (UnitMass::Gram(Metric::Kilo), "kg"),
            (UnitMass::Gram(Metric::Mega), "Mg"),
            (UnitMass::Gram(Metric::Micro), "μg"),
            (UnitMass::Gram(Metric::Milli), "mg"),
            (UnitMass::Gram(Metric::Nano), "ng"),
            (UnitMass::Gram(Metric::None), "g"),
            (UnitMass::Gram(Metric::Peta), "Pg"),
            (UnitMass::Gram(Metric::Pico), "pg"),
            (UnitMass::Gram(Metric::Tera), "Tg"),
            (UnitMass::Gram(Metric::Yocto), "yg"),
            (UnitMass::Gram(Metric::Yotta), "Yg"),
            (UnitMass::Gram(Metric::Zepto), "zg"),
            (UnitMass::Gram(Metric::Zetta), "Zg"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_mass_get_metric() {
        for i in [
            (UnitMass::Gram(Metric::Ronto), Metric::Ronto),
            (UnitMass::Gram(Metric::Ronna), Metric::Ronna),
            (UnitMass::Gram(Metric::Quetta), Metric::Quetta),
            (UnitMass::Gram(Metric::Quecto), Metric::Quecto),
            (UnitMass::Gram(Metric::Atto), Metric::Atto),
            (UnitMass::Gram(Metric::Centi), Metric::Centi),
            (UnitMass::Gram(Metric::Deca), Metric::Deca),
            (UnitMass::Gram(Metric::Deci), Metric::Deci),
            (UnitMass::Gram(Metric::Exa), Metric::Exa),
            (UnitMass::Gram(Metric::Femto), Metric::Femto),
            (UnitMass::Gram(Metric::Giga), Metric::Giga),
            (UnitMass::Gram(Metric::Hecto), Metric::Hecto),
            (UnitMass::Gram(Metric::Kilo), Metric::Kilo),
            (UnitMass::Gram(Metric::Mega), Metric::Mega),
            (UnitMass::Gram(Metric::Micro), Metric::Micro),
            (UnitMass::Gram(Metric::Milli), Metric::Milli),
            (UnitMass::Gram(Metric::Nano), Metric::Nano),
            (UnitMass::Gram(Metric::None), Metric::None),
            (UnitMass::Gram(Metric::Peta), Metric::Peta),
            (UnitMass::Gram(Metric::Pico), Metric::Pico),
            (UnitMass::Gram(Metric::Tera), Metric::Tera),
            (UnitMass::Gram(Metric::Yocto), Metric::Yocto),
            (UnitMass::Gram(Metric::Yotta), Metric::Yotta),
            (UnitMass::Gram(Metric::Zepto), Metric::Zepto),
            (UnitMass::Gram(Metric::Zetta), Metric::Zetta),
            (UnitMass::Grain, Metric::None),
            (UnitMass::Ounce, Metric::None),
            (UnitMass::Pound, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
    }

    #[test]
    fn unit_mass_convert() {
        assert_eq!(
            0.1,
            UnitMass::Gram(Metric::None).convert(&UnitMass::Gram(Metric::Deca))
        );
    }

    #[test]
    fn unit_mass_metric_scale() {
        for i in [
            (UnitMass::Gram(Metric::Ronto), 1.0e-27),
            (UnitMass::Gram(Metric::Ronna), 1.0e27),
            (UnitMass::Gram(Metric::Quetta), 1.0e30),
            (UnitMass::Gram(Metric::Quecto), 1.0e-30),
            (UnitMass::Gram(Metric::Atto), 1.0e-18),
            (UnitMass::Gram(Metric::Centi), 0.01),
            (UnitMass::Gram(Metric::Deca), 10.0),
            (UnitMass::Gram(Metric::Deci), 0.1),
            (UnitMass::Gram(Metric::Exa), 1.0e18),
            (UnitMass::Gram(Metric::Femto), 1.0e-15),
            (UnitMass::Gram(Metric::Giga), 1.0e9),
            (UnitMass::Gram(Metric::Hecto), 100.0),
            (UnitMass::Gram(Metric::Kilo), 1.0e3),
            (UnitMass::Gram(Metric::Mega), 1.0e6),
            (UnitMass::Gram(Metric::Micro), 1.0e-6),
            (UnitMass::Gram(Metric::Milli), 0.001),
            (UnitMass::Gram(Metric::Nano), 1.0e-9),
            (UnitMass::Gram(Metric::None), 1.0),
            (UnitMass::Gram(Metric::Peta), 1.0e15),
            (UnitMass::Gram(Metric::Pico), 1.0e-12),
            (UnitMass::Gram(Metric::Tera), 1.0e12),
            (UnitMass::Gram(Metric::Yocto), 1.0e-24),
            (UnitMass::Gram(Metric::Yotta), 1.0e24),
            (UnitMass::Gram(Metric::Zepto), 1.0e-21),
            (UnitMass::Gram(Metric::Zetta), 1.0e21),
            (UnitMass::Grain, 1.0),
            (UnitMass::Ounce, 1.0),
            (UnitMass::Pound, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }

    /// Unit Mass Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_mass_base_comparison() {
        // Grams are the base SI unit
        assert!(UnitMass::Gram(Metric::None).base() == 1.0);
        // Pounds
        assert!(UnitMass::Pound.base() == 453.592_37);
        // Grains
        assert!(UnitMass::Grain.base() >= 0.06479890);
        // Ounces
        assert!(UnitMass::Ounce.base() >= 28.349_523_124);
    }
}
