use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{metric::Metric, BaseUnit, UnitVolume};

/// The unit types for length
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitLength {
    /// SI unit
    Meter(Metric),
    /// Imperial
    Inch,
    /// Imperial
    Foot,
    /// Imperial
    Yard,
    /// Imperial
    Mile,
    /// Astronomical
    AstronomicalUnit,
    /// Astronomical
    Parsec(Metric),
    /// SI integrated
    LightYear(Metric),
    /// Legacy
    Angstrom,
}

impl Display for UnitLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into())
    }
}

impl<T2> BaseUnit<UnitLength, T2> for UnitLength {
    fn scale(&self) -> T2 {
        match self {
            Self::Meter(m) => m.scale(),
            _ => 1.0.into(),
        }
    }

    fn base(&self) -> T2 {
        match self {
            Self::Meter(_) => 1.0.into(),
            Self::Inch => constants::LENGTH_IN_TO_METER.into(),
            Self::Foot => constants::LENGTH_FT_TO_METER.into(),
            Self::Yard => constants::LENGTH_YD_TO_METER.into(),
            Self::Mile => constants::LENGTH_MILE_TO_METER.into(),
            Self::AstronomicalUnit => constants::LENGTH_AU_TO_METER.into(),
            Self::Parsec(_) => constants::LENGTH_PC_TO_METER.into(),
            Self::LightYear(_) => constants::LENGTH_LYR_TO_METER.into(),
            Self::Angstrom => constants::LENGTH_A_TO_METER.into(),
        }
    }

    fn convert(&self, other: &UnitLength) -> T2 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    fn get_metric(&self) -> Metric {
        match self {
            Self::Meter(m) => *m,
            _ => Metric::None,
        }
    }
}

impl UnitLength {
    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert_liter(&self, other: &UnitVolume) -> f64 {
        self.scale() / // get current metric scale if present
            (f64::powf(UnitLength::Meter(Metric::None).convert(self), 3.0) / // Convert ourselves to meters
            constants::METER3_TO_LITER) *   // meters to liters
            UnitVolume::Liter(Metric::None).convert(other) // convert to correct volume
    }
}

impl Into<String> for UnitLength {
    fn into(self) -> String {
        String::from(match self {
            UnitLength::Meter(m) => m.as_str()+"m",
            UnitLength::Inch => "in",
            UnitLength::Foot => "ft",
            UnitLength::Yard => "yd",
            UnitLength::Mile => "miles",
            UnitLength::AstronomicalUnit => "AU",
            UnitLength::Parsec(m) => m.as_str()+"pc",
            UnitLength::LightYear(m) => m.as_str()+"lyr",
            UnitLength::Angstrom => "Å",
        })
    }
}

impl From<String> for UnitLength {
    fn from(value: String) -> Self {
        match &value {
            "mile" | "miles" => UnitLength::Mile,
            "in" | "inch" | "inches" => UnitLength::Inch,
            "ft" | "foot" | "feet" => UnitLength::Foot,
            "yd" | "yard" | "yards" => UnitLength::Yard,
            e => panic!("Cannot convert `{:?}` into `UnitLength`", e)
        }
    }
}
