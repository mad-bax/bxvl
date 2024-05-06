use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitInformation};

impl Display for UnitInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bit(m) => format!("{}bits", m.as_str()),
                Self::Byte(m) => format!("{}b", m.as_str()),
            }
        )
    }
}

impl Into<String> for UnitInformation {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitInformation> for UnitInformation {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitInformation) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitInformation {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            UnitInformation::Bit(m) | UnitInformation::Byte(m) => match m {
                Metric::Yotta => 1208925819614629174706176.0,
                Metric::Zetta => 1180591620717411303424.0,
                Metric::Exa => 1152921504606846976.0,
                Metric::Peta => 1125899906842624.0,
                Metric::Tera => 1099511627776.0,
                Metric::Giga => 1073741824.0,
                Metric::Mega => 1048576.0,
                Metric::Kilo => 1024.0,
                Metric::None => 1.0,
                _ => 1.0,
            },
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Byte(_) => 1.0,
            Self::Bit(_) => 0.125,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Bit(m) => *m,
            Self::Byte(m) => *m,
        }
    }
}

#[cfg(test)]
mod information_testing {
    use crate::units::{information::UnitInformation, BaseUnit, Metric};

    /// Unit Information Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_information_base_comparison() {
        // Bits
        assert!(UnitInformation::Bit(Metric::None).base() == 0.125);
        // Bytes are the base 'SI unit'
        assert!(UnitInformation::Byte(Metric::None).base() == 1.0);
    }

    /// Unit Information string conversion
    ///
    /// All units must be able to convert to a string.
    #[test]
    fn unit_information_to_string() {
        for i in [
            (UnitInformation::Byte(Metric::Atto), "ab"),
            (UnitInformation::Byte(Metric::Centi), "cb"),
            (UnitInformation::Byte(Metric::Deca), "dab"),
            (UnitInformation::Byte(Metric::Deci), "db"),
            (UnitInformation::Byte(Metric::Exa), "Eb"),
            (UnitInformation::Byte(Metric::Femto), "fb"),
            (UnitInformation::Byte(Metric::Giga), "Gb"),
            (UnitInformation::Byte(Metric::Hecto), "hb"),
            (UnitInformation::Byte(Metric::Kilo), "kb"),
            (UnitInformation::Byte(Metric::Mega), "Mb"),
            (UnitInformation::Byte(Metric::Micro), "μb"),
            (UnitInformation::Byte(Metric::Milli), "mb"),
            (UnitInformation::Byte(Metric::Nano), "nb"),
            (UnitInformation::Byte(Metric::None), "b"),
            (UnitInformation::Byte(Metric::Peta), "Pb"),
            (UnitInformation::Byte(Metric::Pico), "pb"),
            (UnitInformation::Byte(Metric::Tera), "Tb"),
            (UnitInformation::Byte(Metric::Yocto), "yb"),
            (UnitInformation::Byte(Metric::Yotta), "Yb"),
            (UnitInformation::Byte(Metric::Zepto), "zb"),
            (UnitInformation::Byte(Metric::Zetta), "Zb"),
        
            (UnitInformation::Bit(Metric::Atto), "abits"),
            (UnitInformation::Bit(Metric::Centi), "cbits"),
            (UnitInformation::Bit(Metric::Deca), "dabits"),
            (UnitInformation::Bit(Metric::Deci), "dbits"),
            (UnitInformation::Bit(Metric::Exa), "Ebits"),
            (UnitInformation::Bit(Metric::Femto), "fbits"),
            (UnitInformation::Bit(Metric::Giga), "Gbits"),
            (UnitInformation::Bit(Metric::Hecto), "hbits"),
            (UnitInformation::Bit(Metric::Kilo), "kbits"),
            (UnitInformation::Bit(Metric::Mega), "Mbits"),
            (UnitInformation::Bit(Metric::Micro), "μbits"),
            (UnitInformation::Bit(Metric::Milli), "mbits"),
            (UnitInformation::Bit(Metric::Nano), "nbits"),
            (UnitInformation::Bit(Metric::None), "bits"),
            (UnitInformation::Bit(Metric::Peta), "Pbits"),
            (UnitInformation::Bit(Metric::Pico), "pbits"),
            (UnitInformation::Bit(Metric::Tera), "Tbits"),
            (UnitInformation::Bit(Metric::Yocto), "ybits"),
            (UnitInformation::Bit(Metric::Yotta), "Ybits"),
            (UnitInformation::Bit(Metric::Zepto), "zbits"),
            (UnitInformation::Bit(Metric::Zetta), "Zbits"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    /// Unit information metric scaling
    /// 
    /// All units must return the metric 'scaler' value.
    #[test]
    fn unit_information_scale() {
        for i in [
            (UnitInformation::Byte(Metric::Atto), Metric::Atto),
            (UnitInformation::Byte(Metric::Centi), Metric::Centi),
            (UnitInformation::Byte(Metric::Deca), Metric::Deca),
            (UnitInformation::Byte(Metric::Deci), Metric::Deci),
            (UnitInformation::Byte(Metric::Exa), Metric::Exa),
            (UnitInformation::Byte(Metric::Femto), Metric::Femto),
            (UnitInformation::Byte(Metric::Giga), Metric::Giga),
            (UnitInformation::Byte(Metric::Hecto), Metric::Hecto),
            (UnitInformation::Byte(Metric::Kilo), Metric::Kilo),
            (UnitInformation::Byte(Metric::Mega), Metric::Mega),
            (UnitInformation::Byte(Metric::Micro), Metric::Micro),
            (UnitInformation::Byte(Metric::Milli), Metric::Milli),
            (UnitInformation::Byte(Metric::Nano), Metric::Nano),
            (UnitInformation::Byte(Metric::None), Metric::None),
            (UnitInformation::Byte(Metric::Peta), Metric::Peta),
            (UnitInformation::Byte(Metric::Pico), Metric::Pico),
            (UnitInformation::Byte(Metric::Tera), Metric::Tera),
            (UnitInformation::Byte(Metric::Yocto), Metric::Yocto),
            (UnitInformation::Byte(Metric::Yotta), Metric::Yotta),
            (UnitInformation::Byte(Metric::Zepto), Metric::Zepto),
            (UnitInformation::Byte(Metric::Zetta), Metric::Zetta),

            (UnitInformation::Bit(Metric::Atto), Metric::Atto),
            (UnitInformation::Bit(Metric::Centi), Metric::Centi),
            (UnitInformation::Bit(Metric::Deca), Metric::Deca),
            (UnitInformation::Bit(Metric::Deci), Metric::Deci),
            (UnitInformation::Bit(Metric::Exa), Metric::Exa),
            (UnitInformation::Bit(Metric::Femto), Metric::Femto),
            (UnitInformation::Bit(Metric::Giga), Metric::Giga),
            (UnitInformation::Bit(Metric::Hecto), Metric::Hecto),
            (UnitInformation::Bit(Metric::Kilo), Metric::Kilo),
            (UnitInformation::Bit(Metric::Mega), Metric::Mega),
            (UnitInformation::Bit(Metric::Micro), Metric::Micro),
            (UnitInformation::Bit(Metric::Milli), Metric::Milli),
            (UnitInformation::Bit(Metric::Nano), Metric::Nano),
            (UnitInformation::Bit(Metric::None), Metric::None),
            (UnitInformation::Bit(Metric::Peta), Metric::Peta),
            (UnitInformation::Bit(Metric::Pico), Metric::Pico),
            (UnitInformation::Bit(Metric::Tera), Metric::Tera),
            (UnitInformation::Bit(Metric::Yocto), Metric::Yocto),
            (UnitInformation::Bit(Metric::Yotta), Metric::Yotta),
            (UnitInformation::Bit(Metric::Zepto), Metric::Zepto),
            (UnitInformation::Bit(Metric::Zetta), Metric::Zetta),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [
            (UnitInformation::Byte(Metric::Giga), 1073741824.0),
            (UnitInformation::Byte(Metric::Kilo), 1024.0),
            (UnitInformation::Byte(Metric::Mega), 1048576.0),
            (UnitInformation::Byte(Metric::None), 1.0),
            (UnitInformation::Byte(Metric::Peta), 1125899906842624.0),
            (UnitInformation::Byte(Metric::Tera),  1099511627776.0),
            (UnitInformation::Byte(Metric::Yotta), 1208925819614629174706176.0),
            (UnitInformation::Byte(Metric::Zetta), 1180591620717411303424.0),
            (UnitInformation::Byte(Metric::Exa), 1152921504606846976.0),
            (UnitInformation::Byte(Metric::Atto), 1.0),

            (UnitInformation::Bit(Metric::Giga), 1073741824.0),
            (UnitInformation::Bit(Metric::Kilo), 1024.0),
            (UnitInformation::Bit(Metric::Mega), 1048576.0),
            (UnitInformation::Bit(Metric::None), 1.0),
            (UnitInformation::Bit(Metric::Peta), 1125899906842624.0),
            (UnitInformation::Bit(Metric::Tera),  1099511627776.0),
            (UnitInformation::Bit(Metric::Yotta), 1208925819614629174706176.0),
            (UnitInformation::Bit(Metric::Zetta), 1180591620717411303424.0),
            (UnitInformation::Bit(Metric::Exa), 1152921504606846976.0),
            (UnitInformation::Bit(Metric::Atto), 1.0)
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
