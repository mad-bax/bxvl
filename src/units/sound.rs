use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitSound};

impl Display for UnitSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}B", self.get_metric().as_str())
    }
}

impl Into<String> for UnitSound {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitSound> for UnitSound {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitSound) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitSound {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Bel(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Bel(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod sound_testing {
    use crate::units::{BaseUnit, Metric, UnitSound};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitSound::Bel(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitSound::Bel(Metric::Atto), "aB"),
            (UnitSound::Bel(Metric::Centi), "cB"),
            (UnitSound::Bel(Metric::Deca), "daB"),
            (UnitSound::Bel(Metric::Deci), "dB"),
            (UnitSound::Bel(Metric::Exa), "EB"),
            (UnitSound::Bel(Metric::Femto), "fB"),
            (UnitSound::Bel(Metric::Giga), "GB"),
            (UnitSound::Bel(Metric::Hecto), "hB"),
            (UnitSound::Bel(Metric::Kilo), "kB"),
            (UnitSound::Bel(Metric::Mega), "MB"),
            (UnitSound::Bel(Metric::Micro), "μB"),
            (UnitSound::Bel(Metric::Milli), "mB"),
            (UnitSound::Bel(Metric::Nano), "nB"),
            (UnitSound::Bel(Metric::None), "B"),
            (UnitSound::Bel(Metric::Peta), "PB"),
            (UnitSound::Bel(Metric::Pico), "pB"),
            (UnitSound::Bel(Metric::Tera), "TB"),
            (UnitSound::Bel(Metric::Yocto), "yB"),
            (UnitSound::Bel(Metric::Yotta), "YB"),
            (UnitSound::Bel(Metric::Zepto), "zB"),
            (UnitSound::Bel(Metric::Zetta), "ZB"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitSound::Bel(Metric::Atto), Metric::Atto),
            (UnitSound::Bel(Metric::Centi), Metric::Centi),
            (UnitSound::Bel(Metric::Deca), Metric::Deca),
            (UnitSound::Bel(Metric::Deci), Metric::Deci),
            (UnitSound::Bel(Metric::Exa), Metric::Exa),
            (UnitSound::Bel(Metric::Femto), Metric::Femto),
            (UnitSound::Bel(Metric::Giga), Metric::Giga),
            (UnitSound::Bel(Metric::Hecto), Metric::Hecto),
            (UnitSound::Bel(Metric::Kilo), Metric::Kilo),
            (UnitSound::Bel(Metric::Mega), Metric::Mega),
            (UnitSound::Bel(Metric::Micro), Metric::Micro),
            (UnitSound::Bel(Metric::Milli), Metric::Milli),
            (UnitSound::Bel(Metric::Nano), Metric::Nano),
            (UnitSound::Bel(Metric::None), Metric::None),
            (UnitSound::Bel(Metric::Peta), Metric::Peta),
            (UnitSound::Bel(Metric::Pico), Metric::Pico),
            (UnitSound::Bel(Metric::Tera), Metric::Tera),
            (UnitSound::Bel(Metric::Yocto), Metric::Yocto),
            (UnitSound::Bel(Metric::Yotta), Metric::Yotta),
            (UnitSound::Bel(Metric::Zepto), Metric::Zepto),
            (UnitSound::Bel(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitSound::Bel(Metric::Atto), 0.000000000000000001),
            (UnitSound::Bel(Metric::Centi), 0.01),
            (UnitSound::Bel(Metric::Deca), 10.0),
            (UnitSound::Bel(Metric::Deci), 0.1),
            (UnitSound::Bel(Metric::Exa), 1000000000000000000.0),
            (UnitSound::Bel(Metric::Femto), 0.000000000000001),
            (UnitSound::Bel(Metric::Giga), 1000000000.0),
            (UnitSound::Bel(Metric::Hecto), 100.0),
            (UnitSound::Bel(Metric::Kilo), 1000.0),
            (UnitSound::Bel(Metric::Mega), 1000000.0),
            (UnitSound::Bel(Metric::Micro), 0.000001),
            (UnitSound::Bel(Metric::Milli), 0.001),
            (UnitSound::Bel(Metric::Nano), 0.000000001),
            (UnitSound::Bel(Metric::None), 1.0),
            (UnitSound::Bel(Metric::Peta), 1000000000000000.0),
            (UnitSound::Bel(Metric::Pico), 0.000000000001),
            (UnitSound::Bel(Metric::Tera), 1000000000000.0),
            (UnitSound::Bel(Metric::Yocto), 0.000000000000000000000001),
            (UnitSound::Bel(Metric::Yotta), 1000000000000000000000000.0),
            (UnitSound::Bel(Metric::Zepto), 0.000000000000000000001),
            (UnitSound::Bel(Metric::Zetta), 1000000000000000000000.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
