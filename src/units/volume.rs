use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitLength, UnitVolume};

impl Display for UnitVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}l", self.get_metric().as_str())
    }
}

impl Into<String> for UnitVolume {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitVolume> for UnitVolume {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitVolume) -> f64 {
        self.scale() / other.scale()
    }
}

impl Convert<UnitLength> for UnitVolume {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitLength) -> f64 {
        self.scale()
            * (f64::powf(UnitLength::Meter(Metric::None).convert(other), 3.0)
                / constants::METER3_TO_LITER)
    }
}

impl BaseUnit for UnitVolume {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Liter(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Liter(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod volume_testing {
    use crate::units::{volume::UnitVolume, BaseUnit, Metric};

    /// Unit Time Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_time_base_comparison() {
        assert_eq!(UnitVolume::Liter(Metric::None).base(), 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitVolume::Liter(Metric::Atto), "al"),
            (UnitVolume::Liter(Metric::Centi), "cl"),
            (UnitVolume::Liter(Metric::Deca), "dal"),
            (UnitVolume::Liter(Metric::Deci), "dl"),
            (UnitVolume::Liter(Metric::Exa), "El"),
            (UnitVolume::Liter(Metric::Femto), "fl"),
            (UnitVolume::Liter(Metric::Giga), "Gl"),
            (UnitVolume::Liter(Metric::Hecto), "hl"),
            (UnitVolume::Liter(Metric::Kilo), "kl"),
            (UnitVolume::Liter(Metric::Mega), "Ml"),
            (UnitVolume::Liter(Metric::Micro), "μl"),
            (UnitVolume::Liter(Metric::Milli), "ml"),
            (UnitVolume::Liter(Metric::Nano), "nl"),
            (UnitVolume::Liter(Metric::None), "l"),
            (UnitVolume::Liter(Metric::Peta), "Pl"),
            (UnitVolume::Liter(Metric::Pico), "pl"),
            (UnitVolume::Liter(Metric::Tera), "Tl"),
            (UnitVolume::Liter(Metric::Yocto), "yl"),
            (UnitVolume::Liter(Metric::Yotta), "Yl"),
            (UnitVolume::Liter(Metric::Zepto), "zl"),
            (UnitVolume::Liter(Metric::Zetta), "Zl"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitVolume::Liter(Metric::Atto), Metric::Atto),
            (UnitVolume::Liter(Metric::Centi), Metric::Centi),
            (UnitVolume::Liter(Metric::Deca), Metric::Deca),
            (UnitVolume::Liter(Metric::Deci), Metric::Deci),
            (UnitVolume::Liter(Metric::Exa), Metric::Exa),
            (UnitVolume::Liter(Metric::Femto), Metric::Femto),
            (UnitVolume::Liter(Metric::Giga), Metric::Giga),
            (UnitVolume::Liter(Metric::Hecto), Metric::Hecto),
            (UnitVolume::Liter(Metric::Kilo), Metric::Kilo),
            (UnitVolume::Liter(Metric::Mega), Metric::Mega),
            (UnitVolume::Liter(Metric::Micro), Metric::Micro),
            (UnitVolume::Liter(Metric::Milli), Metric::Milli),
            (UnitVolume::Liter(Metric::Nano), Metric::Nano),
            (UnitVolume::Liter(Metric::None), Metric::None),
            (UnitVolume::Liter(Metric::Peta), Metric::Peta),
            (UnitVolume::Liter(Metric::Pico), Metric::Pico),
            (UnitVolume::Liter(Metric::Tera), Metric::Tera),
            (UnitVolume::Liter(Metric::Yocto), Metric::Yocto),
            (UnitVolume::Liter(Metric::Yotta), Metric::Yotta),
            (UnitVolume::Liter(Metric::Zepto), Metric::Zepto),
            (UnitVolume::Liter(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitVolume::Liter(Metric::Atto), 1.0e-18),
            (UnitVolume::Liter(Metric::Centi), 0.01),
            (UnitVolume::Liter(Metric::Deca), 10.0),
            (UnitVolume::Liter(Metric::Deci), 0.1),
            (UnitVolume::Liter(Metric::Exa), 1.0e18),
            (UnitVolume::Liter(Metric::Femto), 1.0e-15),
            (UnitVolume::Liter(Metric::Giga), 1.0e9),
            (UnitVolume::Liter(Metric::Hecto), 100.0),
            (UnitVolume::Liter(Metric::Kilo), 1.0e3),
            (UnitVolume::Liter(Metric::Mega), 1.0e6),
            (UnitVolume::Liter(Metric::Micro), 1.0e-6),
            (UnitVolume::Liter(Metric::Milli), 0.001),
            (UnitVolume::Liter(Metric::Nano), 1.0e-9),
            (UnitVolume::Liter(Metric::None), 1.0),
            (UnitVolume::Liter(Metric::Peta), 1.0e15),
            (UnitVolume::Liter(Metric::Pico), 1.0e-12),
            (UnitVolume::Liter(Metric::Tera), 1.0e12),
            (UnitVolume::Liter(Metric::Yocto), 1.0e-24),
            (UnitVolume::Liter(Metric::Yotta), 1.0e24),
            (UnitVolume::Liter(Metric::Zepto), 1.0e-21),
            (UnitVolume::Liter(Metric::Zetta), 1.0e21),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
