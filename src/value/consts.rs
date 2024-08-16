use crate::constants::*;
use crate::units::Metric;
use crate::units::UnitElectricCapacitance;
use crate::units::UnitElectricCharge;
use crate::units::UnitEnergy;
use crate::units::UnitFrequency;
use crate::units::UnitLength;
use crate::units::UnitMass;
use crate::units::UnitSubstance;
use crate::units::UnitTemperature;
use crate::units::UnitTime;
use crate::value::Value;

/// Defines a `Value` preset to the value of Earth's gravitational acceleration
pub const EARTH_GRAVITY: Value = Value {
    val: VAL_EARTH_GRAV,
    unit_map: LENGTH_MAP | TIME_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = 1;
        r[TIME_INDEX] = -2;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be absolute zero
pub const ABS_ZERO: Value = Value {
    val: VAL_ABS_ZERO,
    unit_map: TEMPERATURE_MAP,
    exp: {
        let mut r = [0; 31];
        r[TEMPERATURE_INDEX] = 1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be Avogadro's Number
pub const AVOGADROS_NUMBER: Value = Value {
    val: VAL_AVOGADROS,
    unit_map: SUBSTANCE_MAP,
    exp: {
        let mut r = [0; 31];
        r[SUBSTANCE_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be Faraday's Constant
pub const FARADAY: Value = Value {
    val: VAL_FARADAY,
    unit_map: SUBSTANCE_MAP | ELECTRIC_CHARGE_MAP,
    exp: {
        let mut r = [0; 31];
        r[ELECTRIC_CHARGE_INDEX] = 1;
        r[SUBSTANCE_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Atomic Mass Constant
pub const ATOMIC_MASS: Value = Value {
    val: VAL_ATOMIC_MASS,
    unit_map: MASS_MAP,
    exp: {
        let mut r = [0; 31];
        r[MASS_INDEX] = 1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the molar gas constant
pub const MOLAR_GAS: Value = Value {
    val: VAL_MOLAR_GAS,
    unit_map: SUBSTANCE_MAP | TEMPERATURE_MAP | ENERGY_MAP,
    exp: {
        let mut r = [0; 31];
        r[ENERGY_INDEX] = 1;
        r[TEMPERATURE_INDEX] = -1;
        r[SUBSTANCE_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be Coulomb's Constant
pub const COULOMB: Value = Value {
    val: VAL_COULOMBS,
    unit_map: SUBSTANCE_MAP,
    exp: {
        let mut r = [0; 31];
        r[SUBSTANCE_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the speed of light
pub const SPEED_OF_LIGHT: Value = Value {
    val: VAL_LIGHT_SPEED,
    unit_map: TIME_MAP | LENGTH_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = 1;
        r[TIME_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Boltzmann Constant
pub const BOLTZMANN: Value = Value {
    val: VAL_BOLTZMANN,
    unit_map: ENERGY_MAP | TEMPERATURE_MAP,
    exp: {
        let mut r = [0; 31];
        r[ENERGY_INDEX] = 1;
        r[TEMPERATURE_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the newtonian gravitational constant
pub const NEWTONIAN_GRAVITATION: Value = Value {
    val: VAL_NEWTONIAN_GRAVITATION,
    unit_map: LENGTH_MAP | MASS_MAP | TIME_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = 3;
        r[TIME_INDEX] = -2;
        r[MASS_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the charge of an electron
pub const ELECTRON_CHARGE: Value = Value {
    val: VAL_ELECTRON_CHARGE,
    unit_map: ELECTRIC_CHARGE_MAP,
    exp: {
        let mut r = [0; 31];
        r[ELECTRIC_CHARGE_INDEX] = 1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Rydberg Constant
pub const RYDBERG: Value = Value {
    val: VAL_RYDBERG,
    unit_map: LENGTH_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Plank Constant
pub const PLANK: Value = Value {
    val: VAL_PLANKS,
    unit_map: ENERGY_MAP | FREQUENCY_MAP,
    exp: {
        let mut r = [0; 31];
        r[ENERGY_INDEX] = 1;
        r[FREQUENCY_INDEX] = -1;
        r
    },
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Vacuum Electric Permittivity Constant
pub const VACUUM_PERMITTIVITY: Value = Value {
    val: VAL_VACUUM_ELECTRIC_PERMITTIVITY,
    unit_map: LENGTH_MAP | CAPACITANCE_MAP,
    exp: {
        let mut r = [0; 31];
        r[CAPACITANCE_INDEX] = 1;
        r[LENGTH_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: Some(UnitElectricCapacitance::Farad(Metric::None)),
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
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

#[cfg(test)]
mod const_value_testing {

    use crate::{
        units::{
            Metric, UnitElectricCapacitance, UnitElectricCharge, UnitEnergy, UnitFrequency,
            UnitLength, UnitMass, UnitSubstance, UnitTemperature, UnitTime,
        },
        value::consts::{
            CAPACITANCE_INDEX, CAPACITANCE_MAP, ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP,
            ENERGY_INDEX, ENERGY_MAP, FREQUENCY_INDEX, FREQUENCY_MAP, LENGTH_INDEX, LENGTH_MAP,
            MASS_INDEX, MASS_MAP, SUBSTANCE_INDEX, SUBSTANCE_MAP, TEMPERATURE_INDEX,
            TEMPERATURE_MAP, TIME_INDEX, TIME_MAP,
        },
    };

    use super::{
        ABS_ZERO, ATOMIC_MASS, AVOGADROS_NUMBER, BOLTZMANN, COULOMB, EARTH_GRAVITY,
        ELECTRON_CHARGE, FARADAY, MOLAR_GAS, NEWTONIAN_GRAVITATION, PLANK, RYDBERG, SPEED_OF_LIGHT,
        VACUUM_PERMITTIVITY,
    };

    #[test]
    fn const_abs_zero_test() {
        let v1 = ABS_ZERO;
        assert!(v1.val == 0.0);
        assert_eq!(v1.exp[TEMPERATURE_INDEX], 1);
        assert_eq!(v1.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v1.v_temperature,
            Some(UnitTemperature::Kelvin(Metric::None))
        );
        assert!(v1.is_temperature());
    }

    #[test]
    fn const_earth_gravity_test() {
        let v1 = EARTH_GRAVITY;
        assert!(v1.val >= 9.80665);
        assert!(v1.val <= 9.80665);
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -2);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
        assert!(v1.is_acceleration());
    }

    #[test]
    fn const_faraday_test() {
        let v1 = FARADAY;
        assert!(v1.val >= 96485.33212331001);
        assert!(v1.val <= 96485.33212331001);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.exp[ELECTRIC_CHARGE_INDEX], 1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP | ELECTRIC_CHARGE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
        assert_eq!(
            v1.v_electric_charge,
            Some(UnitElectricCharge::Coulomb(Metric::None))
        );
    }

    #[test]
    fn const_avogadros_number_test() {
        let v1 = AVOGADROS_NUMBER;
        assert!(v1.val >= 6.02214076e23);
        assert!(v1.val <= 6.02214076e23);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
    }

    #[test]
    fn const_atomic_mass_test() {
        let v1 = ATOMIC_MASS;
        assert!(v1.val >= 1.6605390666e-27);
        assert!(v1.val <= 1.6605390666e-27);
        assert_eq!(v1.exp[MASS_INDEX], 1);
        assert_eq!(v1.unit_map, MASS_MAP);
        assert_eq!(v1.v_mass, Some(UnitMass::Gram(Metric::Kilo)));
    }

    #[test]
    fn const_molar_gas_test() {
        let v1 = MOLAR_GAS;
        assert!(v1.val >= 8.3144621);
        assert!(v1.val <= 8.3144621);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.exp[TEMPERATURE_INDEX], -1);
        assert_eq!(v1.exp[ENERGY_INDEX], 1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP | TEMPERATURE_MAP | ENERGY_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
        assert_eq!(
            v1.v_temperature,
            Some(UnitTemperature::Kelvin(Metric::None))
        );
        assert_eq!(v1.v_energy, Some(UnitEnergy::Joule(Metric::None)));
    }

    #[test]
    fn const_coulomb_test() {
        let v1 = COULOMB;
        assert!(v1.val >= 8.987551);
        assert!(v1.val <= 8.987551);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
    }

    #[test]
    fn const_speed_of_light_test() {
        let v1 = SPEED_OF_LIGHT;
        assert!(v1.val >= 299792458.0);
        assert!(v1.val <= 299792458.0);
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
        assert!(v1.is_velocity());
    }

    #[test]
    fn const_boltzmann_test() {
        let v1 = BOLTZMANN;
        assert!(v1.val >= 1.380649e-23);
        assert!(v1.val <= 1.380649e-23);
        assert_eq!(v1.exp[ENERGY_INDEX], 1);
        assert_eq!(v1.exp[TEMPERATURE_INDEX], -1);
        assert_eq!(v1.unit_map, ENERGY_MAP | TEMPERATURE_MAP);
        assert_eq!(v1.v_energy, Some(UnitEnergy::Joule(Metric::None)));
        assert_eq!(
            v1.v_temperature,
            Some(UnitTemperature::Kelvin(Metric::None))
        );
    }

    #[test]
    fn const_newtonian_gravitation_test() {
        let v1 = NEWTONIAN_GRAVITATION;
        assert!(v1.val >= 6.673015e-11);
        assert!(v1.val <= 6.673015e-11);
        assert_eq!(v1.exp[LENGTH_INDEX], 3);
        assert_eq!(v1.exp[TIME_INDEX], -2);
        assert_eq!(v1.exp[MASS_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP | MASS_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_mass, Some(UnitMass::Gram(Metric::Kilo)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
    }

    #[test]
    fn const_electron_charge_test() {
        let v1 = ELECTRON_CHARGE;
        assert!(v1.val >= 1.602176634e-19);
        assert!(v1.val <= 1.602176635e-19);
        assert_eq!(v1.exp[ELECTRIC_CHARGE_INDEX], 1);
        assert_eq!(v1.unit_map, ELECTRIC_CHARGE_MAP);
        assert_eq!(
            v1.v_electric_charge,
            Some(UnitElectricCharge::Coulomb(Metric::None))
        );
        assert!(v1.is_electric_charge());
    }

    #[test]
    fn const_rydberg_test() {
        let v1 = RYDBERG;
        assert!(v1.val >= 10973731.568539);
        assert!(v1.val <= 10973731.568540);
        assert_eq!(v1.exp[LENGTH_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
    }

    #[test]
    fn const_plank_test() {
        let v1 = PLANK;
        assert!(v1.val >= 6.62607015e-34);
        assert!(v1.val <= 6.62607016e-34);
        assert_eq!(v1.exp[ENERGY_INDEX], 1);
        assert_eq!(v1.exp[FREQUENCY_INDEX], -1);
        assert_eq!(v1.unit_map, ENERGY_MAP | FREQUENCY_MAP);
        assert_eq!(v1.v_energy, Some(UnitEnergy::Joule(Metric::None)));
        assert_eq!(v1.v_frequency, Some(UnitFrequency::Hertz(Metric::None)));
    }

    #[test]
    fn const_vacuum_test() {
        let v1 = VACUUM_PERMITTIVITY;
        assert!(v1.val >= 8.8541878128e-12);
        assert!(v1.val <= 8.8541878129e-12);
        assert_eq!(v1.exp[LENGTH_INDEX], -1);
        assert_eq!(v1.exp[CAPACITANCE_INDEX], 1);
        assert_eq!(v1.unit_map, LENGTH_MAP | CAPACITANCE_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(
            v1.v_capacitance,
            Some(UnitElectricCapacitance::Farad(Metric::None))
        );
    }
}
