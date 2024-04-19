use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constants;

use super::{Metric, volume::UnitVolume, BaseUnit, Convert};

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
            }
        )
    }
}

impl Into<String> for UnitLength {
    fn into(self) -> String {
        self.to_string()
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
            constants::METER3_TO_LITER) *   // meters to liters
            UnitVolume::Liter(Metric::None).convert(other) // convert to correct volume
    }
}

impl BaseUnit for UnitLength {
    fn scale(&self) -> f64 {
        match self {
            Self::Meter(m) => m.scale(),
            _ => 1.0,
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Meter(_) => 1.0,
            Self::Inch => constants::LENGTH_IN_TO_METER,
            Self::Foot => constants::LENGTH_FT_TO_METER,
            Self::Yard => constants::LENGTH_YD_TO_METER,
            Self::Mile => constants::LENGTH_MILE_TO_METER,
            Self::AstronomicalUnit => constants::LENGTH_AU_TO_METER,
            Self::Parsec(_) => constants::LENGTH_PC_TO_METER,
            Self::LightYear(_) => constants::LENGTH_LYR_TO_METER,
            Self::Angstrom => constants::LENGTH_A_TO_METER,
        }
    }

    fn get_metric(&self) -> Metric {
        match self {
            Self::Meter(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod length_testing {
    use crate::units::{length::UnitLength, Metric, BaseUnit};

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
    }
}
