use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitRadioactivityExposure};

impl Display for UnitRadioactivityExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sievert(m) => format!("{}Sv", m.as_str()),
                Self::Rem => "rem".into(),
            }
        )
    }
}

impl Into<String> for UnitRadioactivityExposure {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitRadioactivityExposure> for UnitRadioactivityExposure {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitRadioactivityExposure) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitRadioactivityExposure {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Sievert(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Sievert(_) => 1.0,
            Self::Rem => constants::RADEX_REM_TO_SV,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Sievert(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod equivalent_dose_testing {
    use crate::units::{radiation_equivalent_dose::UnitRadioactivityExposure, BaseUnit, Metric};

    /// Unit Equivalent Dose of Ionizing Radiation Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_equivalent_base_comparison() {
        // Sieverts are the base SI unit
        assert!(UnitRadioactivityExposure::Sievert(Metric::None).base() == 1.0);
        // Rems
        assert!(UnitRadioactivityExposure::Rem.base() == 0.01);
    }

    #[test]
    fn unit_length_to_string() {
        for i in [
            (UnitRadioactivityExposure::Sievert(Metric::Ronto), "rSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Ronna), "RSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Quetta), "QSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Quecto), "qSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Atto), "aSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Centi), "cSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Deca), "daSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Deci), "dSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Exa), "ESv"),
            (UnitRadioactivityExposure::Sievert(Metric::Femto), "fSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Giga), "GSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Hecto), "hSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Kilo), "kSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Mega), "MSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Micro), "μSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Milli), "mSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Nano), "nSv"),
            (UnitRadioactivityExposure::Sievert(Metric::None), "Sv"),
            (UnitRadioactivityExposure::Sievert(Metric::Peta), "PSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Pico), "pSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Tera), "TSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Yocto), "ySv"),
            (UnitRadioactivityExposure::Sievert(Metric::Yotta), "YSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Zepto), "zSv"),
            (UnitRadioactivityExposure::Sievert(Metric::Zetta), "ZSv"),
            (UnitRadioactivityExposure::Rem, "rem"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_length_scale() {
        for i in [
            (
                UnitRadioactivityExposure::Sievert(Metric::Ronto),
                Metric::Ronto,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Ronna),
                Metric::Ronna,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Quetta),
                Metric::Quetta,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Quecto),
                Metric::Quecto,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Atto),
                Metric::Atto,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Centi),
                Metric::Centi,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Deca),
                Metric::Deca,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Deci),
                Metric::Deci,
            ),
            (UnitRadioactivityExposure::Sievert(Metric::Exa), Metric::Exa),
            (
                UnitRadioactivityExposure::Sievert(Metric::Femto),
                Metric::Femto,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Giga),
                Metric::Giga,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Hecto),
                Metric::Hecto,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Kilo),
                Metric::Kilo,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Mega),
                Metric::Mega,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Micro),
                Metric::Micro,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Milli),
                Metric::Milli,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Nano),
                Metric::Nano,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::None),
                Metric::None,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Peta),
                Metric::Peta,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Pico),
                Metric::Pico,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Tera),
                Metric::Tera,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Yocto),
                Metric::Yocto,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Yotta),
                Metric::Yotta,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Zepto),
                Metric::Zepto,
            ),
            (
                UnitRadioactivityExposure::Sievert(Metric::Zetta),
                Metric::Zetta,
            ),
            (UnitRadioactivityExposure::Rem, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
        for i in [
            (UnitRadioactivityExposure::Sievert(Metric::Ronto), 1.0e-27),
            (UnitRadioactivityExposure::Sievert(Metric::Ronna), 1.0e27),
            (UnitRadioactivityExposure::Sievert(Metric::Quetta), 1.0e30),
            (UnitRadioactivityExposure::Sievert(Metric::Quecto), 1.0e-30),
            (UnitRadioactivityExposure::Sievert(Metric::Atto), 1.0e-18),
            (UnitRadioactivityExposure::Sievert(Metric::Centi), 0.01),
            (UnitRadioactivityExposure::Sievert(Metric::Deca), 10.0),
            (UnitRadioactivityExposure::Sievert(Metric::Deci), 0.1),
            (UnitRadioactivityExposure::Sievert(Metric::Exa), 1.0e18),
            (UnitRadioactivityExposure::Sievert(Metric::Femto), 1.0e-15),
            (UnitRadioactivityExposure::Sievert(Metric::Giga), 1.0e9),
            (UnitRadioactivityExposure::Sievert(Metric::Hecto), 100.0),
            (UnitRadioactivityExposure::Sievert(Metric::Kilo), 1.0e3),
            (UnitRadioactivityExposure::Sievert(Metric::Mega), 1.0e6),
            (UnitRadioactivityExposure::Sievert(Metric::Micro), 1.0e-6),
            (UnitRadioactivityExposure::Sievert(Metric::Milli), 0.001),
            (UnitRadioactivityExposure::Sievert(Metric::Nano), 1.0e-9),
            (UnitRadioactivityExposure::Sievert(Metric::None), 1.0),
            (UnitRadioactivityExposure::Sievert(Metric::Peta), 1.0e15),
            (UnitRadioactivityExposure::Sievert(Metric::Pico), 1.0e-12),
            (UnitRadioactivityExposure::Sievert(Metric::Tera), 1.0e12),
            (UnitRadioactivityExposure::Sievert(Metric::Yocto), 1.0e-24),
            (UnitRadioactivityExposure::Sievert(Metric::Yotta), 1.0e24),
            (UnitRadioactivityExposure::Sievert(Metric::Zepto), 1.0e-21),
            (UnitRadioactivityExposure::Sievert(Metric::Zetta), 1.0e21),
            (UnitRadioactivityExposure::Rem, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
