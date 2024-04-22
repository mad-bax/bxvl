pub mod angle;
pub mod angle_solid;
pub mod catalytic_activity;
pub mod electrical_capacitance;
pub mod electrical_charge;
pub mod electrical_conductance;
pub mod electrical_current;
pub mod electrical_inductance;
pub mod electrical_potential;
pub mod electrical_resistance;
pub mod energy;
pub mod force;
pub mod frequency;
pub mod illuminance;
pub mod information;
pub mod length;
pub mod luminous_flux;
pub mod luminous_intensity;
pub mod magnetic_flux;
pub mod magnetic_flux_density;
pub mod mass;
pub mod metric;
pub mod power;
pub mod pressure;
pub mod radiation_absorbed_dose;
pub mod radiation_equivalent_dose;
pub mod radioactivity;
pub mod sound;
pub mod substance;
pub mod temperature;
pub mod time;
pub mod unitless;
pub mod volume;

use serde::{Deserialize, Serialize};

/// The Metric scale names
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub enum Metric {
    /// Yocto
    Yocto,
    /// Zepto
    Zepto,
    /// Atto
    Atto,
    /// Femto
    Femto,
    /// Pico
    Pico,
    /// Nano
    Nano,
    /// Micro
    Micro,
    /// Milli
    Milli,
    /// Centi
    Centi,
    /// Deci
    Deci,
    /// None (default)
    #[default]
    None,
    /// Deca
    Deca,
    /// Hecto
    Hecto,
    /// Kilo
    Kilo,
    /// Mega
    Mega,
    /// Giga
    Giga,
    /// Tera
    Tera,
    /// Peta
    Peta,
    /// Exa
    Exa,
    /// Zetta
    Zetta,
    /// Yotta
    Yotta,
}

/// The unit types of solid angles
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitSolidAngle {
    /// SI unit
    Steradian(Metric),
}

/// The unit types for angles
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitAngle {
    /// Common Standard
    Degree,
    /// SI unit
    Radian(Metric),
    /// Common Standard
    Moa,
}

/// The unit types for catalytic activity
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitCatalyticActivity {
    /// SI unit
    Katal(Metric),
}

/// The unit types for electric capacitance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricCapacitance {
    /// SI unit
    Farad(Metric),
}

/// The unit types for electric charge
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricCharge {
    /// SI unit
    Coulomb(Metric),
}

/// The unit types for electric conductance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricConductance {
    /// SI unit
    Siemens(Metric),
}

/// The unit types for mass
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitMass {
    /// SI unit
    Gram(Metric),
    /// Imperial
    Grain,
    /// Imperial
    Ounce,
    /// Imperial
    Pound,
}

/// Trait that can be used and called by all of the unit types
pub trait Convert<T> {
    /// The function template for a unit type
    fn convert(&self, other: &T) -> f64;
}

/// Provides the function definitions required for unit conversions.
pub trait BaseUnit {
    /// Returns the [`Metric`] scaler of an SI unit
    fn scale(&self) -> f64;

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64;

    /// Returns the [`Metric`] prefix for the unit
    fn get_metric(&self) -> Metric;
}

#[cfg(test)]
mod units_unit_test {
    use crate::units::pressure::UnitPressure;
    use crate::units::{BaseUnit, Metric};

    #[test]
    fn unit_get_metric() {
        let t = UnitPressure::Bar(Metric::Exa);
        assert_eq!(t.get_metric(), Metric::Exa);
    }
}
