use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitFrequency, UnitTime};

impl Display for UnitTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnitTime::Second(m) => format!("{}s", m.as_str()),
                UnitTime::Minute => "min".into(),
                UnitTime::Hour => "hr".into(),
                UnitTime::Day => "day".into(),
            }
        )
    }
}

impl Into<String> for UnitTime {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitTime> for UnitTime {
    fn convert(&self, other: &UnitTime) -> f64 {
        ((self.scale() / other.scale()) * (self.base() / other.base())).into()
    }
}

impl Convert<UnitFrequency> for UnitTime {
    fn convert(&self, other: &UnitFrequency) -> f64 {
        1.0 / (self.base() * self.scale() * other.scale())
    }
}

impl BaseUnit for UnitTime {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Second(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Second(_) => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3600.0,
            Self::Day => 86400.0,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Second(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod time_testing {
    use crate::units::{time::UnitTime, BaseUnit, Metric};

    /// Unit Time Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_time_base_comparison() {
        assert_eq!(UnitTime::Second(Metric::None).base(), 1.0);
        assert_eq!(UnitTime::Minute.base(), 60.0);
        assert_eq!(UnitTime::Hour.base(), 3600.0);
        assert_eq!(UnitTime::Day.base(), 86400.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [
            (UnitTime::Second(Metric::Atto), "as"),
            (UnitTime::Second(Metric::Centi), "cs"),
            (UnitTime::Second(Metric::Deca), "das"),
            (UnitTime::Second(Metric::Deci), "ds"),
            (UnitTime::Second(Metric::Exa), "Es"),
            (UnitTime::Second(Metric::Femto), "fs"),
            (UnitTime::Second(Metric::Giga), "Gs"),
            (UnitTime::Second(Metric::Hecto), "hs"),
            (UnitTime::Second(Metric::Kilo), "ks"),
            (UnitTime::Second(Metric::Mega), "Ms"),
            (UnitTime::Second(Metric::Micro), "μs"),
            (UnitTime::Second(Metric::Milli), "ms"),
            (UnitTime::Second(Metric::Nano), "ns"),
            (UnitTime::Second(Metric::None), "s"),
            (UnitTime::Second(Metric::Peta), "Ps"),
            (UnitTime::Second(Metric::Pico), "ps"),
            (UnitTime::Second(Metric::Tera), "Ts"),
            (UnitTime::Second(Metric::Yocto), "ys"),
            (UnitTime::Second(Metric::Yotta), "Ys"),
            (UnitTime::Second(Metric::Zepto), "zs"),
            (UnitTime::Second(Metric::Zetta), "Zs"),
            (UnitTime::Day, "day"),
            (UnitTime::Hour, "hr"),
            (UnitTime::Minute, "min"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitTime::Second(Metric::Atto), Metric::Atto),
            (UnitTime::Second(Metric::Centi), Metric::Centi),
            (UnitTime::Second(Metric::Deca), Metric::Deca),
            (UnitTime::Second(Metric::Deci), Metric::Deci),
            (UnitTime::Second(Metric::Exa), Metric::Exa),
            (UnitTime::Second(Metric::Femto), Metric::Femto),
            (UnitTime::Second(Metric::Giga), Metric::Giga),
            (UnitTime::Second(Metric::Hecto), Metric::Hecto),
            (UnitTime::Second(Metric::Kilo), Metric::Kilo),
            (UnitTime::Second(Metric::Mega), Metric::Mega),
            (UnitTime::Second(Metric::Micro), Metric::Micro),
            (UnitTime::Second(Metric::Milli), Metric::Milli),
            (UnitTime::Second(Metric::Nano), Metric::Nano),
            (UnitTime::Second(Metric::None), Metric::None),
            (UnitTime::Second(Metric::Peta), Metric::Peta),
            (UnitTime::Second(Metric::Pico), Metric::Pico),
            (UnitTime::Second(Metric::Tera), Metric::Tera),
            (UnitTime::Second(Metric::Yocto), Metric::Yocto),
            (UnitTime::Second(Metric::Yotta), Metric::Yotta),
            (UnitTime::Second(Metric::Zepto), Metric::Zepto),
            (UnitTime::Second(Metric::Zetta), Metric::Zetta),
            (UnitTime::Hour, Metric::None),
            (UnitTime::Minute, Metric::None),
            (UnitTime::Day, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitTime::Second(Metric::Atto), 0.000000000000000001),
            (UnitTime::Second(Metric::Centi), 0.01),
            (UnitTime::Second(Metric::Deca), 10.0),
            (UnitTime::Second(Metric::Deci), 0.1),
            (UnitTime::Second(Metric::Exa), 1000000000000000000.0),
            (UnitTime::Second(Metric::Femto), 0.000000000000001),
            (UnitTime::Second(Metric::Giga), 1000000000.0),
            (UnitTime::Second(Metric::Hecto), 100.0),
            (UnitTime::Second(Metric::Kilo), 1000.0),
            (UnitTime::Second(Metric::Mega), 1000000.0),
            (UnitTime::Second(Metric::Micro), 0.000001),
            (UnitTime::Second(Metric::Milli), 0.001),
            (UnitTime::Second(Metric::Nano), 0.000000001),
            (UnitTime::Second(Metric::None), 1.0),
            (UnitTime::Second(Metric::Peta), 1000000000000000.0),
            (UnitTime::Second(Metric::Pico), 0.000000000001),
            (UnitTime::Second(Metric::Tera), 1000000000000.0),
            (UnitTime::Second(Metric::Yocto), 0.000000000000000000000001),
            (UnitTime::Second(Metric::Yotta), 1000000000000000000000000.0),
            (UnitTime::Second(Metric::Zepto), 0.000000000000000000001),
            (UnitTime::Second(Metric::Zetta), 1000000000000000000000.0),
            (UnitTime::Hour, 1.0),
            (UnitTime::Minute, 1.0),
            (UnitTime::Day, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
