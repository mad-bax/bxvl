use std::fmt::Display;

use crate::consts;

use super::{BaseUnit, Convert, Metric, UnitLength, UnitVolume};

impl Display for UnitLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnitLength::Meter(m) => format!("{}{}", m.as_str(), "m"),
                UnitLength::Inch => "in".into(),
                UnitLength::Foot => "ft".into(),
                UnitLength::Yard => "yd".into(),
                UnitLength::Mile => "miles".into(),
                UnitLength::AstronomicalUnit => "AU".into(),
                UnitLength::Parsec(m) => format!("{}{}", m.as_str(), "pc"),
                UnitLength::LightYear(m) => format!("{}{}", m.as_str(), "lyr"),
                UnitLength::Angstrom => "Å".into(),
                //UnitLength::Fathom => "fathoms".into(),
                //UnitLength::Furlong => "furlongs".into(),
                //UnitLength::NauticalMile => "NM".into(),
                //UnitLength::Chain => "chains".into(),
            }
        )
    }
}

impl From<UnitLength> for String {
    fn from(val: UnitLength) -> Self {
        val.to_string()
    }
}

impl Convert<UnitLength> for UnitLength {
    fn convert(&self, other: &UnitLength) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl Convert<UnitVolume> for UnitLength {
    fn convert(&self, other: &UnitVolume) -> f64 {
        self.scale() / // get current metric scale if present
            (f64::powf(UnitLength::Meter(Metric::None).convert(self), 3.0) / // Convert ourselves to meters
            consts::METER3_TO_LITER) *   // meters to liters
            UnitVolume::Liter(Metric::None).convert(other) // convert to correct volume
    }
}

impl BaseUnit for UnitLength {
    fn scale(&self) -> f64 {
        match self {
            Self::Meter(m) => m.scale(),
            Self::LightYear(m) => m.scale(),
            Self::Parsec(m) => m.scale(),
            _ => 1.0,
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Meter(_) => 1.0,
            Self::Inch => consts::LENGTH_IN_TO_METER,
            Self::Foot => consts::LENGTH_FT_TO_METER,
            Self::Yard => consts::LENGTH_YD_TO_METER,
            Self::Mile => consts::LENGTH_MILE_TO_METER,
            Self::AstronomicalUnit => consts::LENGTH_AU_TO_METER,
            Self::Parsec(_) => consts::LENGTH_PC_TO_METER,
            Self::LightYear(_) => consts::LENGTH_LYR_TO_METER,
            Self::Angstrom => consts::LENGTH_A_TO_METER,
        }
    }

    fn get_metric(&self) -> Metric {
        match self {
            Self::Meter(m) => *m,
            Self::LightYear(m) => *m,
            Self::Parsec(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod length_testing {
    use crate::units::{length::UnitLength, BaseUnit, Metric};

    /// Unit Length Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_length_base_comparison() {
        // Meters are the base SI unit
        assert!(UnitLength::Meter(Metric::None).base() == 1.0);
        // Feet
        assert!(UnitLength::Foot.base() == 0.3048);
        // Inches
        assert!(UnitLength::Inch.base() == 0.0254);
        // Yards
        assert!(UnitLength::Yard.base() == 0.9144);
        // Mile
        assert!(UnitLength::Mile.base() == 1609.344);
        // Astronomical Unit
        assert!(UnitLength::AstronomicalUnit.base() == 149_597_870_700.0);
        // Lightyear
        assert!(UnitLength::LightYear(Metric::None).base() == 9_460_730_472_580_800.0);
        // Ångström
        assert!(UnitLength::Angstrom.base() == 0.000_000_000_1);
        // Parsec
        assert!(UnitLength::Parsec(Metric::None).base() >= 3.085_677_581_491e16);
        assert!(UnitLength::Parsec(Metric::None).base() < 3.085_677_581_493e16);
    }

    #[test]
    fn unit_length_to_string() {
        for i in [
            (UnitLength::Meter(Metric::Ronto), "rm"),
            (UnitLength::Meter(Metric::Ronna), "Rm"),
            (UnitLength::Meter(Metric::Quetta), "Qm"),
            (UnitLength::Meter(Metric::Quecto), "qm"),
            (UnitLength::Meter(Metric::Atto), "am"),
            (UnitLength::Meter(Metric::Centi), "cm"),
            (UnitLength::Meter(Metric::Deca), "dam"),
            (UnitLength::Meter(Metric::Deci), "dm"),
            (UnitLength::Meter(Metric::Exa), "Em"),
            (UnitLength::Meter(Metric::Femto), "fm"),
            (UnitLength::Meter(Metric::Giga), "Gm"),
            (UnitLength::Meter(Metric::Hecto), "hm"),
            (UnitLength::Meter(Metric::Kilo), "km"),
            (UnitLength::Meter(Metric::Mega), "Mm"),
            (UnitLength::Meter(Metric::Micro), "μm"),
            (UnitLength::Meter(Metric::Milli), "mm"),
            (UnitLength::Meter(Metric::Nano), "nm"),
            (UnitLength::Meter(Metric::None), "m"),
            (UnitLength::Meter(Metric::Peta), "Pm"),
            (UnitLength::Meter(Metric::Pico), "pm"),
            (UnitLength::Meter(Metric::Tera), "Tm"),
            (UnitLength::Meter(Metric::Yocto), "ym"),
            (UnitLength::Meter(Metric::Yotta), "Ym"),
            (UnitLength::Meter(Metric::Zepto), "zm"),
            (UnitLength::Meter(Metric::Zetta), "Zm"),
            (UnitLength::LightYear(Metric::Ronto), "rlyr"),
            (UnitLength::LightYear(Metric::Ronna), "Rlyr"),
            (UnitLength::LightYear(Metric::Quetta), "Qlyr"),
            (UnitLength::LightYear(Metric::Quecto), "qlyr"),
            (UnitLength::LightYear(Metric::Atto), "alyr"),
            (UnitLength::LightYear(Metric::Centi), "clyr"),
            (UnitLength::LightYear(Metric::Deca), "dalyr"),
            (UnitLength::LightYear(Metric::Deci), "dlyr"),
            (UnitLength::LightYear(Metric::Exa), "Elyr"),
            (UnitLength::LightYear(Metric::Femto), "flyr"),
            (UnitLength::LightYear(Metric::Giga), "Glyr"),
            (UnitLength::LightYear(Metric::Hecto), "hlyr"),
            (UnitLength::LightYear(Metric::Kilo), "klyr"),
            (UnitLength::LightYear(Metric::Mega), "Mlyr"),
            (UnitLength::LightYear(Metric::Micro), "μlyr"),
            (UnitLength::LightYear(Metric::Milli), "mlyr"),
            (UnitLength::LightYear(Metric::Nano), "nlyr"),
            (UnitLength::LightYear(Metric::None), "lyr"),
            (UnitLength::LightYear(Metric::Peta), "Plyr"),
            (UnitLength::LightYear(Metric::Pico), "plyr"),
            (UnitLength::LightYear(Metric::Tera), "Tlyr"),
            (UnitLength::LightYear(Metric::Yocto), "ylyr"),
            (UnitLength::LightYear(Metric::Yotta), "Ylyr"),
            (UnitLength::LightYear(Metric::Zepto), "zlyr"),
            (UnitLength::LightYear(Metric::Zetta), "Zlyr"),
            (UnitLength::Parsec(Metric::Ronto), "rpc"),
            (UnitLength::Parsec(Metric::Ronna), "Rpc"),
            (UnitLength::Parsec(Metric::Quetta), "Qpc"),
            (UnitLength::Parsec(Metric::Quecto), "qpc"),
            (UnitLength::Parsec(Metric::Atto), "apc"),
            (UnitLength::Parsec(Metric::Centi), "cpc"),
            (UnitLength::Parsec(Metric::Deca), "dapc"),
            (UnitLength::Parsec(Metric::Deci), "dpc"),
            (UnitLength::Parsec(Metric::Exa), "Epc"),
            (UnitLength::Parsec(Metric::Femto), "fpc"),
            (UnitLength::Parsec(Metric::Giga), "Gpc"),
            (UnitLength::Parsec(Metric::Hecto), "hpc"),
            (UnitLength::Parsec(Metric::Kilo), "kpc"),
            (UnitLength::Parsec(Metric::Mega), "Mpc"),
            (UnitLength::Parsec(Metric::Micro), "μpc"),
            (UnitLength::Parsec(Metric::Milli), "mpc"),
            (UnitLength::Parsec(Metric::Nano), "npc"),
            (UnitLength::Parsec(Metric::None), "pc"),
            (UnitLength::Parsec(Metric::Peta), "Ppc"),
            (UnitLength::Parsec(Metric::Pico), "ppc"),
            (UnitLength::Parsec(Metric::Tera), "Tpc"),
            (UnitLength::Parsec(Metric::Yocto), "ypc"),
            (UnitLength::Parsec(Metric::Yotta), "Ypc"),
            (UnitLength::Parsec(Metric::Zepto), "zpc"),
            (UnitLength::Parsec(Metric::Zetta), "Zpc"),
            (UnitLength::Foot, "ft"),
            (UnitLength::Inch, "in"),
            (UnitLength::Angstrom, "Å"),
            (UnitLength::AstronomicalUnit, "AU"),
            (UnitLength::Mile, "miles"),
            (UnitLength::Yard, "yd"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_length_scale() {
        for i in [
            (UnitLength::Meter(Metric::Ronna), Metric::Ronna),
            (UnitLength::Meter(Metric::Ronto), Metric::Ronto),
            (UnitLength::Meter(Metric::Quetta), Metric::Quetta),
            (UnitLength::Meter(Metric::Quecto), Metric::Quecto),
            (UnitLength::Meter(Metric::Atto), Metric::Atto),
            (UnitLength::Meter(Metric::Centi), Metric::Centi),
            (UnitLength::Meter(Metric::Deca), Metric::Deca),
            (UnitLength::Meter(Metric::Deci), Metric::Deci),
            (UnitLength::Meter(Metric::Exa), Metric::Exa),
            (UnitLength::Meter(Metric::Femto), Metric::Femto),
            (UnitLength::Meter(Metric::Giga), Metric::Giga),
            (UnitLength::Meter(Metric::Hecto), Metric::Hecto),
            (UnitLength::Meter(Metric::Kilo), Metric::Kilo),
            (UnitLength::Meter(Metric::Mega), Metric::Mega),
            (UnitLength::Meter(Metric::Micro), Metric::Micro),
            (UnitLength::Meter(Metric::Milli), Metric::Milli),
            (UnitLength::Meter(Metric::Nano), Metric::Nano),
            (UnitLength::Meter(Metric::None), Metric::None),
            (UnitLength::Meter(Metric::Peta), Metric::Peta),
            (UnitLength::Meter(Metric::Pico), Metric::Pico),
            (UnitLength::Meter(Metric::Tera), Metric::Tera),
            (UnitLength::Meter(Metric::Yocto), Metric::Yocto),
            (UnitLength::Meter(Metric::Yotta), Metric::Yotta),
            (UnitLength::Meter(Metric::Zepto), Metric::Zepto),
            (UnitLength::Meter(Metric::Zetta), Metric::Zetta),
            (UnitLength::LightYear(Metric::Ronna), Metric::Ronna),
            (UnitLength::LightYear(Metric::Ronto), Metric::Ronto),
            (UnitLength::LightYear(Metric::Quetta), Metric::Quetta),
            (UnitLength::LightYear(Metric::Quecto), Metric::Quecto),
            (UnitLength::LightYear(Metric::Atto), Metric::Atto),
            (UnitLength::LightYear(Metric::Centi), Metric::Centi),
            (UnitLength::LightYear(Metric::Deca), Metric::Deca),
            (UnitLength::LightYear(Metric::Deci), Metric::Deci),
            (UnitLength::LightYear(Metric::Exa), Metric::Exa),
            (UnitLength::LightYear(Metric::Femto), Metric::Femto),
            (UnitLength::LightYear(Metric::Giga), Metric::Giga),
            (UnitLength::LightYear(Metric::Hecto), Metric::Hecto),
            (UnitLength::LightYear(Metric::Kilo), Metric::Kilo),
            (UnitLength::LightYear(Metric::Mega), Metric::Mega),
            (UnitLength::LightYear(Metric::Micro), Metric::Micro),
            (UnitLength::LightYear(Metric::Milli), Metric::Milli),
            (UnitLength::LightYear(Metric::Nano), Metric::Nano),
            (UnitLength::LightYear(Metric::None), Metric::None),
            (UnitLength::LightYear(Metric::Peta), Metric::Peta),
            (UnitLength::LightYear(Metric::Pico), Metric::Pico),
            (UnitLength::LightYear(Metric::Tera), Metric::Tera),
            (UnitLength::LightYear(Metric::Yocto), Metric::Yocto),
            (UnitLength::LightYear(Metric::Yotta), Metric::Yotta),
            (UnitLength::LightYear(Metric::Zepto), Metric::Zepto),
            (UnitLength::LightYear(Metric::Zetta), Metric::Zetta),
            (UnitLength::Parsec(Metric::Ronna), Metric::Ronna),
            (UnitLength::Parsec(Metric::Ronto), Metric::Ronto),
            (UnitLength::Parsec(Metric::Quetta), Metric::Quetta),
            (UnitLength::Parsec(Metric::Quecto), Metric::Quecto),
            (UnitLength::Parsec(Metric::Atto), Metric::Atto),
            (UnitLength::Parsec(Metric::Centi), Metric::Centi),
            (UnitLength::Parsec(Metric::Deca), Metric::Deca),
            (UnitLength::Parsec(Metric::Deci), Metric::Deci),
            (UnitLength::Parsec(Metric::Exa), Metric::Exa),
            (UnitLength::Parsec(Metric::Femto), Metric::Femto),
            (UnitLength::Parsec(Metric::Giga), Metric::Giga),
            (UnitLength::Parsec(Metric::Hecto), Metric::Hecto),
            (UnitLength::Parsec(Metric::Kilo), Metric::Kilo),
            (UnitLength::Parsec(Metric::Mega), Metric::Mega),
            (UnitLength::Parsec(Metric::Micro), Metric::Micro),
            (UnitLength::Parsec(Metric::Milli), Metric::Milli),
            (UnitLength::Parsec(Metric::Nano), Metric::Nano),
            (UnitLength::Parsec(Metric::None), Metric::None),
            (UnitLength::Parsec(Metric::Peta), Metric::Peta),
            (UnitLength::Parsec(Metric::Pico), Metric::Pico),
            (UnitLength::Parsec(Metric::Tera), Metric::Tera),
            (UnitLength::Parsec(Metric::Yocto), Metric::Yocto),
            (UnitLength::Parsec(Metric::Yotta), Metric::Yotta),
            (UnitLength::Parsec(Metric::Zepto), Metric::Zepto),
            (UnitLength::Parsec(Metric::Zetta), Metric::Zetta),
            (UnitLength::Foot, Metric::None),
            (UnitLength::Inch, Metric::None),
            (UnitLength::Angstrom, Metric::None),
            (UnitLength::AstronomicalUnit, Metric::None),
            (UnitLength::Mile, Metric::None),
            (UnitLength::Yard, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
        for i in [
            (UnitLength::Meter(Metric::Quecto), 1.0e-30),
            (UnitLength::Meter(Metric::Quetta), 1.0e30),
            (UnitLength::Meter(Metric::Ronto), 1.0e-27),
            (UnitLength::Meter(Metric::Ronna), 1.0e27),
            (UnitLength::Meter(Metric::Atto), 1.0e-18),
            (UnitLength::Meter(Metric::Centi), 0.01),
            (UnitLength::Meter(Metric::Deca), 10.0),
            (UnitLength::Meter(Metric::Deci), 0.1),
            (UnitLength::Meter(Metric::Exa), 1.0e18),
            (UnitLength::Meter(Metric::Femto), 1.0e-15),
            (UnitLength::Meter(Metric::Giga), 1.0e9),
            (UnitLength::Meter(Metric::Hecto), 100.0),
            (UnitLength::Meter(Metric::Kilo), 1.0e3),
            (UnitLength::Meter(Metric::Mega), 1.0e6),
            (UnitLength::Meter(Metric::Micro), 1.0e-6),
            (UnitLength::Meter(Metric::Milli), 0.001),
            (UnitLength::Meter(Metric::Nano), 1.0e-9),
            (UnitLength::Meter(Metric::None), 1.0),
            (UnitLength::Meter(Metric::Peta), 1.0e15),
            (UnitLength::Meter(Metric::Pico), 1.0e-12),
            (UnitLength::Meter(Metric::Tera), 1.0e12),
            (UnitLength::Meter(Metric::Yocto), 1.0e-24),
            (UnitLength::Meter(Metric::Yotta), 1.0e24),
            (UnitLength::Meter(Metric::Zepto), 1.0e-21),
            (UnitLength::Meter(Metric::Zetta), 1.0e21),
            (UnitLength::LightYear(Metric::Quecto), 1.0e-30),
            (UnitLength::LightYear(Metric::Quetta), 1.0e30),
            (UnitLength::LightYear(Metric::Ronto), 1.0e-27),
            (UnitLength::LightYear(Metric::Ronna), 1.0e27),
            (UnitLength::LightYear(Metric::Atto), 1.0e-18),
            (UnitLength::LightYear(Metric::Centi), 0.01),
            (UnitLength::LightYear(Metric::Deca), 10.0),
            (UnitLength::LightYear(Metric::Deci), 0.1),
            (UnitLength::LightYear(Metric::Exa), 1.0e18),
            (UnitLength::LightYear(Metric::Femto), 1.0e-15),
            (UnitLength::LightYear(Metric::Giga), 1.0e9),
            (UnitLength::LightYear(Metric::Hecto), 100.0),
            (UnitLength::LightYear(Metric::Kilo), 1.0e3),
            (UnitLength::LightYear(Metric::Mega), 1.0e6),
            (UnitLength::LightYear(Metric::Micro), 1.0e-6),
            (UnitLength::LightYear(Metric::Milli), 0.001),
            (UnitLength::LightYear(Metric::Nano), 1.0e-9),
            (UnitLength::LightYear(Metric::None), 1.0),
            (UnitLength::LightYear(Metric::Peta), 1.0e15),
            (UnitLength::LightYear(Metric::Pico), 1.0e-12),
            (UnitLength::LightYear(Metric::Tera), 1.0e12),
            (UnitLength::LightYear(Metric::Yocto), 1.0e-24),
            (UnitLength::LightYear(Metric::Yotta), 1.0e24),
            (UnitLength::LightYear(Metric::Zepto), 1.0e-21),
            (UnitLength::LightYear(Metric::Zetta), 1.0e21),
            (UnitLength::Parsec(Metric::Quecto), 1.0e-30),
            (UnitLength::Parsec(Metric::Quetta), 1.0e30),
            (UnitLength::Parsec(Metric::Ronto), 1.0e-27),
            (UnitLength::Parsec(Metric::Ronna), 1.0e27),
            (UnitLength::Parsec(Metric::Atto), 1.0e-18),
            (UnitLength::Parsec(Metric::Centi), 0.01),
            (UnitLength::Parsec(Metric::Deca), 10.0),
            (UnitLength::Parsec(Metric::Deci), 0.1),
            (UnitLength::Parsec(Metric::Exa), 1.0e18),
            (UnitLength::Parsec(Metric::Femto), 1.0e-15),
            (UnitLength::Parsec(Metric::Giga), 1.0e9),
            (UnitLength::Parsec(Metric::Hecto), 100.0),
            (UnitLength::Parsec(Metric::Kilo), 1.0e3),
            (UnitLength::Parsec(Metric::Mega), 1.0e6),
            (UnitLength::Parsec(Metric::Micro), 1.0e-6),
            (UnitLength::Parsec(Metric::Milli), 0.001),
            (UnitLength::Parsec(Metric::Nano), 1.0e-9),
            (UnitLength::Parsec(Metric::None), 1.0),
            (UnitLength::Parsec(Metric::Peta), 1.0e15),
            (UnitLength::Parsec(Metric::Pico), 1.0e-12),
            (UnitLength::Parsec(Metric::Tera), 1.0e12),
            (UnitLength::Parsec(Metric::Yocto), 1.0e-24),
            (UnitLength::Parsec(Metric::Yotta), 1.0e24),
            (UnitLength::Parsec(Metric::Zepto), 1.0e-21),
            (UnitLength::Parsec(Metric::Zetta), 1.0e21),
            (UnitLength::Foot, 1.0),
            (UnitLength::Inch, 1.0),
            (UnitLength::Angstrom, 1.0),
            (UnitLength::AstronomicalUnit, 1.0),
            (UnitLength::Mile, 1.0),
            (UnitLength::Yard, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
