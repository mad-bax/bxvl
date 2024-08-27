use std::fmt::Display;

use crate::consts;

use super::{BaseUnit, Convert, Metric, UnitPressure};

impl Display for UnitPressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pascal(m) => format!("{}Pa", m.as_str()),
                Self::Bar(m) => format!("{}bar", m.as_str()),
                Self::Torr => "torr".into(),
                Self::Hgmm => "mmHg".into(),
                Self::Hgcm => "cmHg".into(),
                Self::Hgin => "inHg".into(),
                Self::Atm => "atm".into(),
                Self::Psi => "psi".into(),
            }
        )
    }
}

impl From<UnitPressure> for String {
    fn from(val: UnitPressure) -> Self {
        val.to_string()
    }
}

impl Convert<UnitPressure> for UnitPressure {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitPressure) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitPressure {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Pascal(m) => m.scale(),
            Self::Bar(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Pascal(_) => 1.0,
            Self::Bar(_) => consts::PR_BAR_TO_P,
            Self::Torr => consts::PR_TORR_TO_P,
            Self::Hgmm => consts::PR_MM_TO_P,
            Self::Hgcm => consts::PR_CM_TO_P,
            Self::Hgin => consts::PR_IN_TO_P,
            Self::Atm => consts::PR_ATM_TO_P,
            Self::Psi => consts::PR_PSI_TO_P,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Pascal(m) => *m,
            Self::Bar(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod pressure_testing {
    use crate::units::{pressure::UnitPressure, BaseUnit, Metric};

    /// Unit Pressure Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_pressure_base_comparison() {
        // Pascals are the base SI unit
        assert!(UnitPressure::Pascal(Metric::None).base() == 1.0);
        // Atmospheres
        assert!(UnitPressure::Atm.base() == 101325.0);
        // Bar
        assert!(UnitPressure::Bar(Metric::None).base() == 100000.0);
        // inHg
        assert!(UnitPressure::Hgin.base() >= 3386.3886665);
        assert!(UnitPressure::Hgin.base() < 3386.3886667);
        // mmHg
        assert!(UnitPressure::Hgmm.base() >= 133.322387414);
        assert!(UnitPressure::Hgmm.base() < 133.322387416);
        // cmHg
        assert!(UnitPressure::Hgcm.base() >= 1333.22387414);
        assert!(UnitPressure::Hgcm.base() < 1333.22387416);
        // PSI
        assert!(UnitPressure::Psi.base() == 6894.757);
        // Torr
        assert!(UnitPressure::Torr.base() >= 133.322368420);
        assert!(UnitPressure::Torr.base() < 133.322368422);
    }

    #[test]
    fn unit_length_to_string() {
        for i in [
            (UnitPressure::Bar(Metric::Ronto), "rbar"),
            (UnitPressure::Bar(Metric::Ronna), "Rbar"),
            (UnitPressure::Bar(Metric::Quetta), "Qbar"),
            (UnitPressure::Bar(Metric::Quecto), "qbar"),
            (UnitPressure::Bar(Metric::Atto), "abar"),
            (UnitPressure::Bar(Metric::Centi), "cbar"),
            (UnitPressure::Bar(Metric::Deca), "dabar"),
            (UnitPressure::Bar(Metric::Deci), "dbar"),
            (UnitPressure::Bar(Metric::Exa), "Ebar"),
            (UnitPressure::Bar(Metric::Femto), "fbar"),
            (UnitPressure::Bar(Metric::Giga), "Gbar"),
            (UnitPressure::Bar(Metric::Hecto), "hbar"),
            (UnitPressure::Bar(Metric::Kilo), "kbar"),
            (UnitPressure::Bar(Metric::Mega), "Mbar"),
            (UnitPressure::Bar(Metric::Micro), "μbar"),
            (UnitPressure::Bar(Metric::Milli), "mbar"),
            (UnitPressure::Bar(Metric::Nano), "nbar"),
            (UnitPressure::Bar(Metric::None), "bar"),
            (UnitPressure::Bar(Metric::Peta), "Pbar"),
            (UnitPressure::Bar(Metric::Pico), "pbar"),
            (UnitPressure::Bar(Metric::Tera), "Tbar"),
            (UnitPressure::Bar(Metric::Yocto), "ybar"),
            (UnitPressure::Bar(Metric::Yotta), "Ybar"),
            (UnitPressure::Bar(Metric::Zepto), "zbar"),
            (UnitPressure::Bar(Metric::Zetta), "Zbar"),
            (UnitPressure::Pascal(Metric::Ronto), "rPa"),
            (UnitPressure::Pascal(Metric::Ronna), "RPa"),
            (UnitPressure::Pascal(Metric::Quetta), "QPa"),
            (UnitPressure::Pascal(Metric::Quecto), "qPa"),
            (UnitPressure::Pascal(Metric::Atto), "aPa"),
            (UnitPressure::Pascal(Metric::Centi), "cPa"),
            (UnitPressure::Pascal(Metric::Deca), "daPa"),
            (UnitPressure::Pascal(Metric::Deci), "dPa"),
            (UnitPressure::Pascal(Metric::Exa), "EPa"),
            (UnitPressure::Pascal(Metric::Femto), "fPa"),
            (UnitPressure::Pascal(Metric::Giga), "GPa"),
            (UnitPressure::Pascal(Metric::Hecto), "hPa"),
            (UnitPressure::Pascal(Metric::Kilo), "kPa"),
            (UnitPressure::Pascal(Metric::Mega), "MPa"),
            (UnitPressure::Pascal(Metric::Micro), "μPa"),
            (UnitPressure::Pascal(Metric::Milli), "mPa"),
            (UnitPressure::Pascal(Metric::Nano), "nPa"),
            (UnitPressure::Pascal(Metric::None), "Pa"),
            (UnitPressure::Pascal(Metric::Peta), "PPa"),
            (UnitPressure::Pascal(Metric::Pico), "pPa"),
            (UnitPressure::Pascal(Metric::Tera), "TPa"),
            (UnitPressure::Pascal(Metric::Yocto), "yPa"),
            (UnitPressure::Pascal(Metric::Yotta), "YPa"),
            (UnitPressure::Pascal(Metric::Zepto), "zPa"),
            (UnitPressure::Pascal(Metric::Zetta), "ZPa"),
            (UnitPressure::Hgmm, "mmHg"),
            (UnitPressure::Hgcm, "cmHg"),
            (UnitPressure::Psi, "psi"),
            (UnitPressure::Hgin, "inHg"),
            (UnitPressure::Atm, "atm"),
            (UnitPressure::Torr, "torr"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_length_scale() {
        for i in [
            (UnitPressure::Bar(Metric::Quecto), Metric::Quecto),
            (UnitPressure::Bar(Metric::Quetta), Metric::Quetta),
            (UnitPressure::Bar(Metric::Ronto), Metric::Ronto),
            (UnitPressure::Bar(Metric::Ronna), Metric::Ronna),
            (UnitPressure::Bar(Metric::Atto), Metric::Atto),
            (UnitPressure::Bar(Metric::Centi), Metric::Centi),
            (UnitPressure::Bar(Metric::Deca), Metric::Deca),
            (UnitPressure::Bar(Metric::Deci), Metric::Deci),
            (UnitPressure::Bar(Metric::Exa), Metric::Exa),
            (UnitPressure::Bar(Metric::Femto), Metric::Femto),
            (UnitPressure::Bar(Metric::Giga), Metric::Giga),
            (UnitPressure::Bar(Metric::Hecto), Metric::Hecto),
            (UnitPressure::Bar(Metric::Kilo), Metric::Kilo),
            (UnitPressure::Bar(Metric::Mega), Metric::Mega),
            (UnitPressure::Bar(Metric::Micro), Metric::Micro),
            (UnitPressure::Bar(Metric::Milli), Metric::Milli),
            (UnitPressure::Bar(Metric::Nano), Metric::Nano),
            (UnitPressure::Bar(Metric::None), Metric::None),
            (UnitPressure::Bar(Metric::Peta), Metric::Peta),
            (UnitPressure::Bar(Metric::Pico), Metric::Pico),
            (UnitPressure::Bar(Metric::Tera), Metric::Tera),
            (UnitPressure::Bar(Metric::Yocto), Metric::Yocto),
            (UnitPressure::Bar(Metric::Yotta), Metric::Yotta),
            (UnitPressure::Bar(Metric::Zepto), Metric::Zepto),
            (UnitPressure::Bar(Metric::Zetta), Metric::Zetta),
            (UnitPressure::Pascal(Metric::Quecto), Metric::Quecto),
            (UnitPressure::Pascal(Metric::Quetta), Metric::Quetta),
            (UnitPressure::Pascal(Metric::Ronto), Metric::Ronto),
            (UnitPressure::Pascal(Metric::Ronna), Metric::Ronna),
            (UnitPressure::Pascal(Metric::Atto), Metric::Atto),
            (UnitPressure::Pascal(Metric::Centi), Metric::Centi),
            (UnitPressure::Pascal(Metric::Deca), Metric::Deca),
            (UnitPressure::Pascal(Metric::Deci), Metric::Deci),
            (UnitPressure::Pascal(Metric::Exa), Metric::Exa),
            (UnitPressure::Pascal(Metric::Femto), Metric::Femto),
            (UnitPressure::Pascal(Metric::Giga), Metric::Giga),
            (UnitPressure::Pascal(Metric::Hecto), Metric::Hecto),
            (UnitPressure::Pascal(Metric::Kilo), Metric::Kilo),
            (UnitPressure::Pascal(Metric::Mega), Metric::Mega),
            (UnitPressure::Pascal(Metric::Micro), Metric::Micro),
            (UnitPressure::Pascal(Metric::Milli), Metric::Milli),
            (UnitPressure::Pascal(Metric::Nano), Metric::Nano),
            (UnitPressure::Pascal(Metric::None), Metric::None),
            (UnitPressure::Pascal(Metric::Peta), Metric::Peta),
            (UnitPressure::Pascal(Metric::Pico), Metric::Pico),
            (UnitPressure::Pascal(Metric::Tera), Metric::Tera),
            (UnitPressure::Pascal(Metric::Yocto), Metric::Yocto),
            (UnitPressure::Pascal(Metric::Yotta), Metric::Yotta),
            (UnitPressure::Pascal(Metric::Zepto), Metric::Zepto),
            (UnitPressure::Pascal(Metric::Zetta), Metric::Zetta),
            (UnitPressure::Hgmm, Metric::None),
            (UnitPressure::Hgcm, Metric::None),
            (UnitPressure::Psi, Metric::None),
            (UnitPressure::Hgin, Metric::None),
            (UnitPressure::Atm, Metric::None),
            (UnitPressure::Torr, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
        for i in [
            (UnitPressure::Bar(Metric::Ronna), 1.0e27),
            (UnitPressure::Bar(Metric::Ronto), 1.0e-27),
            (UnitPressure::Bar(Metric::Quecto), 1.0e-30),
            (UnitPressure::Bar(Metric::Quetta), 1.0e30),
            (UnitPressure::Bar(Metric::Atto), 1.0e-18),
            (UnitPressure::Bar(Metric::Centi), 0.01),
            (UnitPressure::Bar(Metric::Deca), 10.0),
            (UnitPressure::Bar(Metric::Deci), 0.1),
            (UnitPressure::Bar(Metric::Exa), 1.0e18),
            (UnitPressure::Bar(Metric::Femto), 1.0e-15),
            (UnitPressure::Bar(Metric::Giga), 1.0e9),
            (UnitPressure::Bar(Metric::Hecto), 100.0),
            (UnitPressure::Bar(Metric::Kilo), 1.0e3),
            (UnitPressure::Bar(Metric::Mega), 1.0e6),
            (UnitPressure::Bar(Metric::Micro), 1.0e-6),
            (UnitPressure::Bar(Metric::Milli), 0.001),
            (UnitPressure::Bar(Metric::Nano), 1.0e-9),
            (UnitPressure::Bar(Metric::None), 1.0),
            (UnitPressure::Bar(Metric::Peta), 1.0e15),
            (UnitPressure::Bar(Metric::Pico), 1.0e-12),
            (UnitPressure::Bar(Metric::Tera), 1.0e12),
            (UnitPressure::Bar(Metric::Yocto), 1.0e-24),
            (UnitPressure::Bar(Metric::Yotta), 1.0e24),
            (UnitPressure::Bar(Metric::Zepto), 1.0e-21),
            (UnitPressure::Bar(Metric::Zetta), 1.0e21),
            (UnitPressure::Pascal(Metric::Ronna), 1.0e27),
            (UnitPressure::Pascal(Metric::Ronto), 1.0e-27),
            (UnitPressure::Pascal(Metric::Quecto), 1.0e-30),
            (UnitPressure::Pascal(Metric::Quetta), 1.0e30),
            (UnitPressure::Pascal(Metric::Atto), 1.0e-18),
            (UnitPressure::Pascal(Metric::Centi), 0.01),
            (UnitPressure::Pascal(Metric::Deca), 10.0),
            (UnitPressure::Pascal(Metric::Deci), 0.1),
            (UnitPressure::Pascal(Metric::Exa), 1.0e18),
            (UnitPressure::Pascal(Metric::Femto), 1.0e-15),
            (UnitPressure::Pascal(Metric::Giga), 1.0e9),
            (UnitPressure::Pascal(Metric::Hecto), 100.0),
            (UnitPressure::Pascal(Metric::Kilo), 1.0e3),
            (UnitPressure::Pascal(Metric::Mega), 1.0e6),
            (UnitPressure::Pascal(Metric::Micro), 1.0e-6),
            (UnitPressure::Pascal(Metric::Milli), 0.001),
            (UnitPressure::Pascal(Metric::Nano), 1.0e-9),
            (UnitPressure::Pascal(Metric::None), 1.0),
            (UnitPressure::Pascal(Metric::Peta), 1.0e15),
            (UnitPressure::Pascal(Metric::Pico), 1.0e-12),
            (UnitPressure::Pascal(Metric::Tera), 1.0e12),
            (UnitPressure::Pascal(Metric::Yocto), 1.0e-24),
            (UnitPressure::Pascal(Metric::Yotta), 1.0e24),
            (UnitPressure::Pascal(Metric::Zepto), 1.0e-21),
            (UnitPressure::Pascal(Metric::Zetta), 1.0e21),
            (UnitPressure::Hgmm, 1.0),
            (UnitPressure::Hgcm, 1.0),
            (UnitPressure::Psi, 1.0),
            (UnitPressure::Hgin, 1.0),
            (UnitPressure::Atm, 1.0),
            (UnitPressure::Torr, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
