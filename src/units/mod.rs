
/// Unit module for [`UnitAngle`].
pub mod angle;

/// Unit module for [`UnitSolidAngle`].
pub mod angle_solid;

/// Unit module for [`UnitCatalyticActivity`].
pub mod catalytic_activity;

/// Unit module for [`UnitElectricCapacitance`].
pub mod electrical_capacitance;

/// Unit module for [`UnitElectricCharge`].
pub mod electrical_charge;

/// Unit module for [`UnitElectricConductance`].
pub mod electrical_conductance;

/// Unit module for [`UnitElectricCurrent`].
pub mod electrical_current;

/// Unit module for [`UnitElectricInductance`].
pub mod electrical_inductance;

/// Unit module for [`UnitElectricPotential`].
pub mod electrical_potential;

/// Unit module for [`UnitElectricResistance`].
pub mod electrical_resistance;

/// Unit module for [`UnitEnergy`].
pub mod energy;

/// Unit module for [`UnitForce`].
pub mod force;

/// Unit module for [`UnitFrequency`]
pub mod frequency;

/// Unit module for [`UnitIlluminance`]
pub mod illuminance;

/// Unit module for [`UnitInformation`]
pub mod information;

/// Unit module for [`UnitLength`]
pub mod length;

/// Unit module for [`UnitLuminousFlux`]
pub mod luminous_flux;

/// Unit module for [`UnitLuminousIntensity`]
pub mod luminous_intensity;

/// Unit module for [`UnitMagneticFlux`]
pub mod magnetic_flux;

/// Unit module for [`UnitMagneticFluxDensity`]
pub mod magnetic_flux_density;

/// Unit module for [`UnitMass`]
pub mod mass;

/// Module for [`Metric`]
pub mod metric;

/// Unit module for [`UnitPower`]
pub mod power;

/// Unit module for [`UnitPressure`]
pub mod pressure;

/// Unit module for [`UnitAbsorbedDose`]
pub mod radiation_absorbed_dose;

/// Unit module for [`UnitRadioactivityExposure`]
pub mod radiation_equivalent_dose;

/// Unit module for [`UnitRadioactivity`]
pub mod radioactivity;

/// Unit module for [`UnitSound`]
pub mod sound;

/// Unit module for [`UnitSubstance`]
pub mod substance;

/// Unit module for [`UnitTemperature`]
pub mod temperature;

/// Unit module for [`UnitTime`]
pub mod time;

/// Unit module for [`UnitNone`]
pub mod unitless;

/// Unit module for [`UnitVolume`]
pub mod volume;

use serde::{Deserialize, Serialize};

macro_rules! units {
    ($u:expr, $count:expr) => {
        * u * units!(u, count-1)
    };

    ($u:expr, 0) => {
        * u
    }
}

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

/// The unit types for electric current
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricCurrent {
    /// SI unit
    Ampere(Metric),
}

/// The unit types for electric inductance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricInductance {
    /// SI unit
    Henry(Metric),
}

/// The unit types for electric potential
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricPotential {
    /// SI unit
    Volt(Metric),
}

/// The unit types for electric resistance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitElectricResistance {
    /// SI unit
    Ohm(Metric),
}

/// The unit types of energy
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitEnergy {
    /// SI unit
    Joule(Metric),
    /// SI integrated
    GramCalorie(Metric),
    /// Imperial
    FootPound,
    /// SI integrated
    ElectronVolt(Metric),
}

/// The unit types of force
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitForce {
    /// SI unit
    Newton(Metric),
    /// Imperial
    PoundForce,
}

/// The unit types of frequency
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitFrequency {
    /// SI unit
    Hertz(Metric),
}

/// The unit types for illuminance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitIlluminance {
    /// SI unit
    Lux(Metric),
}

/// The unit types for a measurement of information
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitInformation {
    /// Not SI but uses metric prefixing
    Bit(Metric),
    /// Not SI but uses metric prefixing
    Byte(Metric),
}

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

/// The unit types for luminous flux
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitLuminousFlux {
    /// SI unit
    Lumen(Metric),
}

/// The unit types for luminous intensity
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitLuminousIntensity {
    /// SI unit
    Candela(Metric),
}

/// The unit types for magnetic flux density
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitMagneticFluxDensity {
    /// SI unit
    Tesla(Metric),
}

/// The unit types for magnetic flux
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitMagneticFlux {
    /// SI unit
    Weber(Metric),
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

/// The unit types of power
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitPower {
    /// SI unit
    Watt(Metric),
}

/// The unit types for pressure
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitPressure {
    /// SI unit
    Pascal(Metric),
    /// SI integrated, Common Standard
    Bar(Metric),
    /// Common Standard
    Torr,
    /// SI integrated
    Hgmm,
    /// Imperial
    Hgin,
    /// Common Standard
    Atm,
    /// Imperial
    Psi,
}

/// The unit types of absorbed dose of ionizing radiation
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitAbsorbedDose {
    /// SI unit
    Gray(Metric),
    /// Legacy
    Roentgen,
    /// Legacy
    Rad,
}

/// The unit types of equaivalent dose of ionizing radiation
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitRadioactivityExposure {
    /// SI unit
    Sievert(Metric),
    /// Legacy
    Rem,
}

/// The unit types of radioactivity
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitRadioactivity {
    /// SI unit
    Becquerel(Metric),
    /// Legacy
    Curie,
}

/// The unit types of a measurement of sound
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitSound {
    /// SI unit
    Bel(Metric),
}

/// The unit types for substance
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitSubstance {
    /// SI unit
    Mole(Metric),
}

/// The unit types for temperature
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitTemperature {
    /// SI Unit
    Celsius,
    /// Imperial
    Fahrenheit,
    /// SI unit
    Kelvin(Metric),
}

/// The unit types for time
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitTime {
    /// SI unit
    Second(Metric),
    /// Non - SI
    Minute,
    /// Non - SI
    Hour,
    /// Non - SI
    Day,
}

/// 'Empty' units
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitNone {
    /// To describe a `Value` representing a percentage
    Percentage,

    /// Literally just a number
    None,
}

/// The unit types for volume
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitVolume {
    /// SI unit
    Liter(Metric),
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
    use crate::units::UnitPressure;
    use crate::units::{BaseUnit, Metric};

    #[test]
    fn unit_get_metric() {
        let t = UnitPressure::Bar(Metric::Exa);
        assert_eq!(t.get_metric(), Metric::Exa);
    }
}
