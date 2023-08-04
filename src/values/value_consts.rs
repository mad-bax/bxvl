use crate::values::Value;
use crate::constants::*;
use crate::units::{
    Metric,
    UnitTime,
    UnitTemperature,
    UnitLength,
    UnitElectricCharge,
    UnitSubstance,
    UnitEnergy,
    UnitMass,
    UnitFrequency,
    UnitCapacitance,
};

/// Returns a `Value` preset to the value of Earth's gravitational acceleration
pub const fn const_earth_gravity() -> Value {
    let mut ret:Value = Value {
        val:VAL_EARTH_GRAV,
        unit_map:LENGTH_MAP | TIME_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: Some(UnitLength::Meter(Metric::None)),
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: Some(UnitTime::Second(Metric::None)),
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[LENGTH_INDEX] = 1;
    ret.exp[TIME_INDEX] = -2;
    ret
}

/// Returns a `Value` preset to be absolute zero
pub const fn const_abs_zero() -> Value {
    let mut ret:Value = Value {
        val:VAL_ABS_ZERO,
        unit_map:TEMPERATURE_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: Some(UnitTemperature::Kelvin(Metric::None)),
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None,
    };
    ret.exp[TEMPERATURE_INDEX] = 1;
    ret
}

/// Returns a `Value` preset to be Avogadro's Number
pub const fn const_avogadros_number() -> Value {
    let mut ret:Value = Value {
        val:VAL_AVOGADROS,
        unit_map:SUBSTANCE_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: Some(UnitSubstance::Mole(Metric::None)),
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be Faraday's Constant
pub const fn const_faraday() -> Value {
    let mut ret:Value = Value {
        val:VAL_FARADAY,
        unit_map:SUBSTANCE_MAP | ELECTRIC_CHARGE_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: Some(UnitElectricCharge::Coulomb(Metric::None)),
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: Some(UnitSubstance::Mole(Metric::None)),
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[ELECTRIC_CHARGE_INDEX] = 1;
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the Atomic Mass Constant
pub const fn const_atomic_mass() -> Value {
    let mut ret:Value = Value {
        val:VAL_ATOMIC_MASS,
        unit_map:MASS_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: Some(UnitMass::Gram(Metric::Kilo)),
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[MASS_INDEX] = 1;
    ret
}

/// Returns a `Value` preset to be the molar gas constant
pub const fn const_molar_gas() -> Value {
    let mut ret:Value = Value {
        val:VAL_MOLAR_GAS,
        unit_map:SUBSTANCE_MAP | TEMPERATURE_MAP | ENERGY_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: Some(UnitEnergy::Joule(Metric::None)),
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: Some(UnitSubstance::Mole(Metric::None)),
        v_temperature: Some(UnitTemperature::Kelvin(Metric::None)),
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[ENERGY_INDEX] = 1;
    ret.exp[TEMPERATURE_INDEX] = -1;
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be Coulomb's Constant
pub const fn const_coulomb() -> Value {
    let mut ret:Value = Value {
        val:VAL_COULOMBS,
        unit_map:SUBSTANCE_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: Some(UnitSubstance::Mole(Metric::None)),
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the speed of light
pub const fn const_speed_of_light() -> Value {
    let mut ret:Value = Value {
        val:VAL_LIGHT_SPEED,
        unit_map:TIME_MAP | LENGTH_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: Some(UnitLength::Meter(Metric::None)),
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: Some(UnitTime::Second(Metric::None)),
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[LENGTH_INDEX] = 1;
    ret.exp[TIME_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the Boltzmann Constant
pub const fn const_boltzmann() -> Value {
    let mut ret:Value = Value {
        val:VAL_BOLTZMANN,
        unit_map:ENERGY_MAP | TEMPERATURE_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: Some(UnitEnergy::Joule(Metric::None)),
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: Some(UnitTemperature::Kelvin(Metric::None)),
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[ENERGY_INDEX] = 1;
    ret.exp[TEMPERATURE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the newtonian gravitational constant
pub const fn const_newtonian_gravitation() -> Value {
    let mut ret:Value = Value {
        val:VAL_NEWTONIAN_GRAVITATION,
        unit_map:LENGTH_MAP | MASS_MAP | TIME_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: Some(UnitLength::Meter(Metric::None)),
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: Some(UnitMass::Gram(Metric::Kilo)),
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: Some(UnitTime::Second(Metric::None)),
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[LENGTH_INDEX] = 3;
    ret.exp[TIME_INDEX] = -2;
    ret.exp[MASS_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the charge of an electron
pub const fn const_electron_charge() -> Value {
    let mut ret:Value = Value {
        val:VAL_ELECTRON_CHARGE,
        unit_map: ELECTRIC_CHARGE_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: Some(UnitElectricCharge::Coulomb(Metric::None)),
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[ELECTRIC_CHARGE_INDEX] = 1;
    ret
}

/// Returns a `Value` preset to be the Rydberg Constant
pub const fn const_rydberg() -> Value {
    let mut ret:Value = Value {
        val:VAL_RYDBERG,
        unit_map: LENGTH_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: Some(UnitLength::Meter(Metric::None)),
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[LENGTH_INDEX] = -1;
    ret       
}

/// Returns a `Value` preset to be the Plank Constant
pub const fn const_plank() -> Value {
    let mut ret:Value = Value {
        val:VAL_PLANKS,
        unit_map: ENERGY_MAP | FREQUENCY_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: None,
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: Some(UnitEnergy::Joule(Metric::None)),
        v_force: None,
        v_frequency: Some(UnitFrequency::Hertz(Metric::None)),
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: None,
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[ENERGY_INDEX] = 1;
    ret.exp[FREQUENCY_INDEX] = -1;
    ret       
}

/// Returns a `Value` preset to be the Vacuum Electric Permittivity Constant
pub const fn const_vacuum_permittivity() -> Value {
    let mut ret:Value = Value {
        val:VAL_VACUUM_ELECTRIC_PERMITTIVITY,
        unit_map: LENGTH_MAP | CAPACITANCE_MAP,
        exp:[0;31],
        v_ab_dose: None,
        v_angle: None,
        v_capacitance: Some(UnitCapacitance::Farad(Metric::None)),
        v_catalytic: None,
        v_electric_charge: None,
        v_electric_conductance: None,
        v_electric_current: None,
        v_electric_potential: None,
        v_energy: None,
        v_force: None,
        v_frequency: None,
        v_illuminance: None,
        v_inductance: None,
        v_information: None,
        v_length: Some(UnitLength::Meter(Metric::None)),
        v_luminous_flux: None,
        v_luminous_flux_intensity: None,
        v_mass: None,
        v_power: None,
        v_pressure: None,
        v_radioactivity: None,
        v_radioactivity_exposure: None,
        v_resistance: None,
        v_sound: None,
        v_substance: None,
        v_temperature: None,
        v_time: None,
        v_volume: None,
        v_magnetic_flux : None,
        v_magnetic_flux_density : None,
        v_solid_angle : None
    };
    ret.exp[CAPACITANCE_INDEX] = 1;
    ret.exp[LENGTH_INDEX] = -1;
    ret       
}