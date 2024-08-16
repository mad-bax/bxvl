/// Module used to display and parsing a [`Value`]
pub mod strings;

/// Module used to implement basic operations on a [`Value`]
pub mod value_impl;

mod arithmetic_ops;

mod construction;

mod manipulation;

mod value_std_ops;

/// Module used to provide [`Value`] constants
pub mod consts;

use serde::{Deserialize, Serialize};

use crate::units::{
    UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
    UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
    UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency,
    UnitIlluminance, UnitInformation, UnitLength, UnitLuminousFlux, UnitLuminousIntensity,
    UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure,
    UnitRadioactivity, UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance,
    UnitTemperature, UnitTime, UnitVolume,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
enum ValueLogBase {
    BaseE,
    #[default]
    Base10,
    Base2,
}

/// The `Value` struct definition
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Value {
    /// The numerical value for the `Value` struct
    pub val: f64,
    /// The unit map which specifies which units are present in the `Value`
    unit_map: usize,
    /// The exponent storage of all the units within a `Value`
    exp: [i32; 31],
    /// the absorbed dose of ionizing radiation measure
    v_ab_dose: Option<UnitAbsorbedDose>,
    /// the angle measure
    v_angle: Option<UnitAngle>,
    /// the capacitance measure
    v_capacitance: Option<UnitElectricCapacitance>,
    /// the catalytic activity measure
    v_catalytic: Option<UnitCatalyticActivity>,
    /// the electric charge measure
    v_electric_charge: Option<UnitElectricCharge>,
    /// the electric conductance measure
    v_electric_conductance: Option<UnitElectricConductance>,
    /// the electric current measure
    v_electric_current: Option<UnitElectricCurrent>,
    /// the electric potential measure
    v_electric_potential: Option<UnitElectricPotential>,
    /// the energy measure
    v_energy: Option<UnitEnergy>,
    /// the force measure
    v_force: Option<UnitForce>,
    /// the frequency measure
    v_frequency: Option<UnitFrequency>,
    /// the illuminance measure
    v_illuminance: Option<UnitIlluminance>,
    /// the inductance measure
    v_inductance: Option<UnitElectricInductance>,
    /// the information measure
    v_information: Option<UnitInformation>,
    /// the length measure
    v_length: Option<UnitLength>,
    /// the luminous flux measure
    v_luminous_flux: Option<UnitLuminousFlux>,
    /// the luminous intensity measure
    v_luminous_flux_intensity: Option<UnitLuminousIntensity>,
    /// the mass measure
    v_mass: Option<UnitMass>,
    /// the power measure
    v_power: Option<UnitPower>,
    /// the pressure measure
    v_pressure: Option<UnitPressure>,
    /// the radioactivity measure
    v_radioactivity: Option<UnitRadioactivity>,
    /// the equivalent dose measure
    v_radioactivity_exposure: Option<UnitRadioactivityExposure>,
    /// the resistance measure
    v_resistance: Option<UnitElectricResistance>,
    /// the sound measure
    v_sound: Option<UnitSound>,
    /// the substance measure
    v_substance: Option<UnitSubstance>,
    /// the temperature measure
    v_temperature: Option<UnitTemperature>,
    /// the time measure
    v_time: Option<UnitTime>,
    /// the volume measure
    v_volume: Option<UnitVolume>,
    /// the magnetic flux measure
    v_magnetic_flux: Option<UnitMagneticFlux>,
    /// The magnetic flux density measure
    v_magnetic_flux_density: Option<UnitMagneticFluxDensity>,
    /// The solid angle measure
    v_solid_angle: Option<UnitSolidAngle>,
}

/// Macro to create a new `Value`
#[macro_export]
macro_rules! value {
    ($v:expr, $u:expr) => {
        Value::new($v as f64, &$u.to_string()).unwrap()
    };
}
