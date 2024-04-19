use std::fmt::Display;

use crate::constants;

use super::{Metric, BaseUnit, Convert, UnitMass};

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

impl Into<String> for UnitMass {
    fn into(self) -> String {
        self.to_string()
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
    use crate::units::{Metric, BaseUnit, Convert, UnitMass};

    #[test]
    fn unit_mass_to_string() {
        for i in [
            (UnitMass::Grain, "gr"),
            (UnitMass::Ounce, "oz"),
            (UnitMass::Pound, "lb"),
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
            let t:String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_mass_get_metric() {
        for i in [
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
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
    }

    #[test]
    fn unit_mass_convert() {
        assert_eq!(0.1, UnitMass::Gram(Metric::None).convert(&UnitMass::Gram(Metric::Deca)));
    }

    #[test]
    fn unit_mass_metric_scale() {
        for i in [
            (UnitMass::Gram(Metric::Atto), 0.000000000000000001),
            (UnitMass::Gram(Metric::Centi), 0.01),
            (UnitMass::Gram(Metric::Deca), 10.0),
            (UnitMass::Gram(Metric::Deci), 0.1),
            (UnitMass::Gram(Metric::Exa), 1000000000000000000.0),
            (UnitMass::Gram(Metric::Femto), 0.000000000000001),
            (UnitMass::Gram(Metric::Giga), 1000000000.0),
            (UnitMass::Gram(Metric::Hecto), 100.0),
            (UnitMass::Gram(Metric::Kilo), 1000.0),
            (UnitMass::Gram(Metric::Mega), 1000000.0),
            (UnitMass::Gram(Metric::Micro), 0.000001),
            (UnitMass::Gram(Metric::Milli), 0.001),
            (UnitMass::Gram(Metric::Nano), 0.000000001),
            (UnitMass::Gram(Metric::None), 1.0),
            (UnitMass::Gram(Metric::Peta), 1000000000000000.0),
            (UnitMass::Gram(Metric::Pico), 0.000000000001),
            (UnitMass::Gram(Metric::Tera), 1000000000000.0),
            (UnitMass::Gram(Metric::Yocto), 0.000000000000000000000001),
            (UnitMass::Gram(Metric::Yotta), 1000000000000000000000000.0),
            (UnitMass::Gram(Metric::Zepto), 0.000000000000000000001),
            (UnitMass::Gram(Metric::Zetta), 1000000000000000000000.0),
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
