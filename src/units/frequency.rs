use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitFrequency, UnitTime};

impl Display for UnitFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Hz", self.get_metric().as_str())
    }
}

impl Into<String> for UnitFrequency {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitFrequency> for UnitFrequency {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitFrequency) -> f64 {
        self.scale() / other.scale()
    }
}

impl Convert<UnitTime> for UnitFrequency {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitTime) -> f64 {
        (self.scale() / other.scale()) * (other.convert(&UnitTime::Second(Metric::None)))
    }
}

impl BaseUnit for UnitFrequency {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Hertz(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Hertz(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod frequency_testing {
    use crate::units::{BaseUnit, Metric, UnitFrequency};

    #[test]
    fn unit_angle_base_comparison() {
        assert!(UnitFrequency::Hertz(Metric::None).base() == 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitFrequency::Hertz(Metric::Atto), "aHz"),
            (UnitFrequency::Hertz(Metric::Centi), "cHz"),
            (UnitFrequency::Hertz(Metric::Deca), "daHz"),
            (UnitFrequency::Hertz(Metric::Deci), "dHz"),
            (UnitFrequency::Hertz(Metric::Exa), "EHz"),
            (UnitFrequency::Hertz(Metric::Femto), "fHz"),
            (UnitFrequency::Hertz(Metric::Giga), "GHz"),
            (UnitFrequency::Hertz(Metric::Hecto), "hHz"),
            (UnitFrequency::Hertz(Metric::Kilo), "kHz"),
            (UnitFrequency::Hertz(Metric::Mega), "MHz"),
            (UnitFrequency::Hertz(Metric::Micro), "μHz"),
            (UnitFrequency::Hertz(Metric::Milli), "mHz"),
            (UnitFrequency::Hertz(Metric::Nano), "nHz"),
            (UnitFrequency::Hertz(Metric::None), "Hz"),
            (UnitFrequency::Hertz(Metric::Peta), "PHz"),
            (UnitFrequency::Hertz(Metric::Pico), "pHz"),
            (UnitFrequency::Hertz(Metric::Tera), "THz"),
            (UnitFrequency::Hertz(Metric::Yocto), "yHz"),
            (UnitFrequency::Hertz(Metric::Yotta), "YHz"),
            (UnitFrequency::Hertz(Metric::Zepto), "zHz"),
            (UnitFrequency::Hertz(Metric::Zetta), "ZHz"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitFrequency::Hertz(Metric::Atto), Metric::Atto),
            (UnitFrequency::Hertz(Metric::Centi), Metric::Centi),
            (UnitFrequency::Hertz(Metric::Deca), Metric::Deca),
            (UnitFrequency::Hertz(Metric::Deci), Metric::Deci),
            (UnitFrequency::Hertz(Metric::Exa), Metric::Exa),
            (UnitFrequency::Hertz(Metric::Femto), Metric::Femto),
            (UnitFrequency::Hertz(Metric::Giga), Metric::Giga),
            (UnitFrequency::Hertz(Metric::Hecto), Metric::Hecto),
            (UnitFrequency::Hertz(Metric::Kilo), Metric::Kilo),
            (UnitFrequency::Hertz(Metric::Mega), Metric::Mega),
            (UnitFrequency::Hertz(Metric::Micro), Metric::Micro),
            (UnitFrequency::Hertz(Metric::Milli), Metric::Milli),
            (UnitFrequency::Hertz(Metric::Nano), Metric::Nano),
            (UnitFrequency::Hertz(Metric::None), Metric::None),
            (UnitFrequency::Hertz(Metric::Peta), Metric::Peta),
            (UnitFrequency::Hertz(Metric::Pico), Metric::Pico),
            (UnitFrequency::Hertz(Metric::Tera), Metric::Tera),
            (UnitFrequency::Hertz(Metric::Yocto), Metric::Yocto),
            (UnitFrequency::Hertz(Metric::Yotta), Metric::Yotta),
            (UnitFrequency::Hertz(Metric::Zepto), Metric::Zepto),
            (UnitFrequency::Hertz(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitFrequency::Hertz(Metric::Atto), 0.000000000000000001),
            (UnitFrequency::Hertz(Metric::Centi), 0.01),
            (UnitFrequency::Hertz(Metric::Deca), 10.0),
            (UnitFrequency::Hertz(Metric::Deci), 0.1),
            (UnitFrequency::Hertz(Metric::Exa), 1000000000000000000.0),
            (UnitFrequency::Hertz(Metric::Femto), 0.000000000000001),
            (UnitFrequency::Hertz(Metric::Giga), 1000000000.0),
            (UnitFrequency::Hertz(Metric::Hecto), 100.0),
            (UnitFrequency::Hertz(Metric::Kilo), 1000.0),
            (UnitFrequency::Hertz(Metric::Mega), 1000000.0),
            (UnitFrequency::Hertz(Metric::Micro), 0.000001),
            (UnitFrequency::Hertz(Metric::Milli), 0.001),
            (UnitFrequency::Hertz(Metric::Nano), 0.000000001),
            (UnitFrequency::Hertz(Metric::None), 1.0),
            (UnitFrequency::Hertz(Metric::Peta), 1000000000000000.0),
            (UnitFrequency::Hertz(Metric::Pico), 0.000000000001),
            (UnitFrequency::Hertz(Metric::Tera), 1000000000000.0),
            (
                UnitFrequency::Hertz(Metric::Yocto),
                0.000000000000000000000001,
            ),
            (
                UnitFrequency::Hertz(Metric::Yotta),
                1000000000000000000000000.0,
            ),
            (UnitFrequency::Hertz(Metric::Zepto), 0.000000000000000000001),
            (
                UnitFrequency::Hertz(Metric::Zetta),
                1000000000000000000000.0,
            ),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
