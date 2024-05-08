use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitSubstance};

impl Display for UnitSubstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mol", self.get_metric().as_str())
    }
}

impl Into<String> for UnitSubstance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitSubstance> for UnitSubstance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitSubstance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitSubstance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Mole(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Mole(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod substance_testing {
    use crate::units::{BaseUnit, Metric, UnitSubstance};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitSubstance::Mole(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitSubstance::Mole(Metric::Atto), "amol"),
            (UnitSubstance::Mole(Metric::Centi), "cmol"),
            (UnitSubstance::Mole(Metric::Deca), "damol"),
            (UnitSubstance::Mole(Metric::Deci), "dmol"),
            (UnitSubstance::Mole(Metric::Exa), "Emol"),
            (UnitSubstance::Mole(Metric::Femto), "fmol"),
            (UnitSubstance::Mole(Metric::Giga), "Gmol"),
            (UnitSubstance::Mole(Metric::Hecto), "hmol"),
            (UnitSubstance::Mole(Metric::Kilo), "kmol"),
            (UnitSubstance::Mole(Metric::Mega), "Mmol"),
            (UnitSubstance::Mole(Metric::Micro), "μmol"),
            (UnitSubstance::Mole(Metric::Milli), "mmol"),
            (UnitSubstance::Mole(Metric::Nano), "nmol"),
            (UnitSubstance::Mole(Metric::None), "mol"),
            (UnitSubstance::Mole(Metric::Peta), "Pmol"),
            (UnitSubstance::Mole(Metric::Pico), "pmol"),
            (UnitSubstance::Mole(Metric::Tera), "Tmol"),
            (UnitSubstance::Mole(Metric::Yocto), "ymol"),
            (UnitSubstance::Mole(Metric::Yotta), "Ymol"),
            (UnitSubstance::Mole(Metric::Zepto), "zmol"),
            (UnitSubstance::Mole(Metric::Zetta), "Zmol"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitSubstance::Mole(Metric::Atto), Metric::Atto),
            (UnitSubstance::Mole(Metric::Centi), Metric::Centi),
            (UnitSubstance::Mole(Metric::Deca), Metric::Deca),
            (UnitSubstance::Mole(Metric::Deci), Metric::Deci),
            (UnitSubstance::Mole(Metric::Exa), Metric::Exa),
            (UnitSubstance::Mole(Metric::Femto), Metric::Femto),
            (UnitSubstance::Mole(Metric::Giga), Metric::Giga),
            (UnitSubstance::Mole(Metric::Hecto), Metric::Hecto),
            (UnitSubstance::Mole(Metric::Kilo), Metric::Kilo),
            (UnitSubstance::Mole(Metric::Mega), Metric::Mega),
            (UnitSubstance::Mole(Metric::Micro), Metric::Micro),
            (UnitSubstance::Mole(Metric::Milli), Metric::Milli),
            (UnitSubstance::Mole(Metric::Nano), Metric::Nano),
            (UnitSubstance::Mole(Metric::None), Metric::None),
            (UnitSubstance::Mole(Metric::Peta), Metric::Peta),
            (UnitSubstance::Mole(Metric::Pico), Metric::Pico),
            (UnitSubstance::Mole(Metric::Tera), Metric::Tera),
            (UnitSubstance::Mole(Metric::Yocto), Metric::Yocto),
            (UnitSubstance::Mole(Metric::Yotta), Metric::Yotta),
            (UnitSubstance::Mole(Metric::Zepto), Metric::Zepto),
            (UnitSubstance::Mole(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitSubstance::Mole(Metric::Atto), 0.000000000000000001),
            (UnitSubstance::Mole(Metric::Centi), 0.01),
            (UnitSubstance::Mole(Metric::Deca), 10.0),
            (UnitSubstance::Mole(Metric::Deci), 0.1),
            (UnitSubstance::Mole(Metric::Exa), 1000000000000000000.0),
            (UnitSubstance::Mole(Metric::Femto), 0.000000000000001),
            (UnitSubstance::Mole(Metric::Giga), 1000000000.0),
            (UnitSubstance::Mole(Metric::Hecto), 100.0),
            (UnitSubstance::Mole(Metric::Kilo), 1000.0),
            (UnitSubstance::Mole(Metric::Mega), 1000000.0),
            (UnitSubstance::Mole(Metric::Micro), 0.000001),
            (UnitSubstance::Mole(Metric::Milli), 0.001),
            (UnitSubstance::Mole(Metric::Nano), 0.000000001),
            (UnitSubstance::Mole(Metric::None), 1.0),
            (UnitSubstance::Mole(Metric::Peta), 1000000000000000.0),
            (UnitSubstance::Mole(Metric::Pico), 0.000000000001),
            (UnitSubstance::Mole(Metric::Tera), 1000000000000.0),
            (
                UnitSubstance::Mole(Metric::Yocto),
                0.000000000000000000000001,
            ),
            (
                UnitSubstance::Mole(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (UnitSubstance::Mole(Metric::Zepto), 0.000000000000000000001),
            (UnitSubstance::Mole(Metric::Zetta), 1000000000000000000000.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
