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
use crate::values::Value;

/// Returns a `Value` preset to the value of Earth's gravitational acceleration
pub const fn const_earth_gravity() -> Value {
    let mut ret: Value = Value {
        val: VAL_EARTH_GRAV,
        unit_map: LENGTH_MAP | TIME_MAP,
        exp: [0; 31],
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
    ret.exp[LENGTH_INDEX] = 1;
    ret.exp[TIME_INDEX] = -2;
    ret
}

/// Returns a `Value` preset to be absolute zero
pub const fn const_abs_zero() -> Value {
    let mut ret: Value = Value {
        val: VAL_ABS_ZERO,
        unit_map: TEMPERATURE_MAP,
        exp: [0; 31],
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
    ret.exp[TEMPERATURE_INDEX] = 1;
    ret
}

/// Returns a `Value` preset to be Avogadro's Number
pub const fn const_avogadros_number() -> Value {
    let mut ret: Value = Value {
        val: VAL_AVOGADROS,
        unit_map: SUBSTANCE_MAP,
        exp: [0; 31],
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
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be Faraday's Constant
pub const fn const_faraday() -> Value {
    let mut ret: Value = Value {
        val: VAL_FARADAY,
        unit_map: SUBSTANCE_MAP | ELECTRIC_CHARGE_MAP,
        exp: [0; 31],
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
    ret.exp[ELECTRIC_CHARGE_INDEX] = 1;
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the Atomic Mass Constant
pub const fn const_atomic_mass() -> Value {
    let mut ret: Value = Value {
        val: VAL_ATOMIC_MASS,
        unit_map: MASS_MAP,
        exp: [0; 31],
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
    ret.exp[MASS_INDEX] = 1;
    ret
}

/// Returns a `Value` preset to be the molar gas constant
pub const fn const_molar_gas() -> Value {
    let mut ret: Value = Value {
        val: VAL_MOLAR_GAS,
        unit_map: SUBSTANCE_MAP | TEMPERATURE_MAP | ENERGY_MAP,
        exp: [0; 31],
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
    ret.exp[ENERGY_INDEX] = 1;
    ret.exp[TEMPERATURE_INDEX] = -1;
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be Coulomb's Constant
pub const fn const_coulomb() -> Value {
    let mut ret: Value = Value {
        val: VAL_COULOMBS,
        unit_map: SUBSTANCE_MAP,
        exp: [0; 31],
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
    ret.exp[SUBSTANCE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the speed of light
pub const fn const_speed_of_light() -> Value {
    let mut ret: Value = Value {
        val: VAL_LIGHT_SPEED,
        unit_map: TIME_MAP | LENGTH_MAP,
        exp: [0; 31],
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
    ret.exp[LENGTH_INDEX] = 1;
    ret.exp[TIME_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the Boltzmann Constant
pub const fn const_boltzmann() -> Value {
    let mut ret: Value = Value {
        val: VAL_BOLTZMANN,
        unit_map: ENERGY_MAP | TEMPERATURE_MAP,
        exp: [0; 31],
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
    ret.exp[ENERGY_INDEX] = 1;
    ret.exp[TEMPERATURE_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the newtonian gravitational constant
pub const fn const_newtonian_gravitation() -> Value {
    let mut ret: Value = Value {
        val: VAL_NEWTONIAN_GRAVITATION,
        unit_map: LENGTH_MAP | MASS_MAP | TIME_MAP,
        exp: [0; 31],
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
    ret.exp[LENGTH_INDEX] = 3;
    ret.exp[TIME_INDEX] = -2;
    ret.exp[MASS_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the charge of an electron
pub const fn const_electron_charge() -> Value {
    let mut ret: Value = Value {
        val: VAL_ELECTRON_CHARGE,
        unit_map: ELECTRIC_CHARGE_MAP,
        exp: [0; 31],
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
    ret.exp[ELECTRIC_CHARGE_INDEX] = 1;
    ret
}

/// Returns a `Value` preset to be the Rydberg Constant
pub const fn const_rydberg() -> Value {
    let mut ret: Value = Value {
        val: VAL_RYDBERG,
        unit_map: LENGTH_MAP,
        exp: [0; 31],
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
    ret.exp[LENGTH_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the Plank Constant
pub const fn const_plank() -> Value {
    let mut ret: Value = Value {
        val: VAL_PLANKS,
        unit_map: ENERGY_MAP | FREQUENCY_MAP,
        exp: [0; 31],
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
    ret.exp[ENERGY_INDEX] = 1;
    ret.exp[FREQUENCY_INDEX] = -1;
    ret
}

/// Returns a `Value` preset to be the Vacuum Electric Permittivity Constant
pub const fn const_vacuum_permittivity() -> Value {
    let mut ret: Value = Value {
        val: VAL_VACUUM_ELECTRIC_PERMITTIVITY,
        unit_map: LENGTH_MAP | CAPACITANCE_MAP,
        exp: [0; 31],
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
    ret.exp[CAPACITANCE_INDEX] = 1;
    ret.exp[LENGTH_INDEX] = -1;
    ret
}

#[cfg(test)]
mod const_value_testing {

    use crate::{
        units::{
            Metric, UnitElectricCapacitance, UnitElectricCharge, UnitEnergy, UnitFrequency,
            UnitLength, UnitMass, UnitSubstance, UnitTemperature, UnitTime,
        },
        values::value_consts::{
            CAPACITANCE_INDEX, CAPACITANCE_MAP, ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP,
            ENERGY_INDEX, ENERGY_MAP, FREQUENCY_INDEX, FREQUENCY_MAP, LENGTH_INDEX, LENGTH_MAP,
            MASS_INDEX, MASS_MAP, SUBSTANCE_INDEX, SUBSTANCE_MAP, TEMPERATURE_INDEX,
            TEMPERATURE_MAP, TIME_INDEX, TIME_MAP,
        },
    };

    use super::{
        const_abs_zero, const_atomic_mass, const_avogadros_number, const_boltzmann, const_coulomb,
        const_earth_gravity, const_electron_charge, const_faraday, const_molar_gas,
        const_newtonian_gravitation, const_plank, const_rydberg, const_speed_of_light,
        const_vacuum_permittivity,
    };

    #[test]
    fn const_abs_zero_test() {
        let v1 = const_abs_zero();
        assert!(v1.val == 0.0);
        assert_eq!(v1.exp[TEMPERATURE_INDEX], 1);
        assert_eq!(v1.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v1.v_temperature,
            Some(UnitTemperature::Kelvin(Metric::None))
        );
    }

    #[test]
    fn const_earth_gravity_test() {
        let v1 = const_earth_gravity();
        assert!(v1.val >= 9.80665);
        assert!(v1.val <= 9.80665);
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -2);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
    }

    #[test]
    fn const_faraday_test() {
        let v1 = const_faraday();
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
        let v1 = const_avogadros_number();
        assert!(v1.val >= 6.02214076e23);
        assert!(v1.val <= 6.02214076e23);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
    }

    #[test]
    fn const_atomic_mass_test() {
        let v1 = const_atomic_mass();
        assert!(v1.val >= 1.6605390666e-27);
        assert!(v1.val <= 1.6605390666e-27);
        assert_eq!(v1.exp[MASS_INDEX], 1);
        assert_eq!(v1.unit_map, MASS_MAP);
        assert_eq!(v1.v_mass, Some(UnitMass::Gram(Metric::Kilo)));
    }

    #[test]
    fn const_molar_gas_test() {
        let v1 = const_molar_gas();
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
        let v1 = const_coulomb();
        assert!(v1.val >= 8.987551);
        assert!(v1.val <= 8.987551);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
    }

    #[test]
    fn const_speed_of_light_test() {
        let v1 = const_speed_of_light();
        assert!(v1.val >= 299792458.0);
        assert!(v1.val <= 299792458.0);
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
    }

    #[test]
    fn const_boltzmann_test() {
        let v1 = const_boltzmann();
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
        let v1 = const_newtonian_gravitation();
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
        let v1 = const_electron_charge();
        assert!(v1.val >= 1.602176634e-19);
        assert!(v1.val <= 1.602176635e-19);
        assert_eq!(v1.exp[ELECTRIC_CHARGE_INDEX], 1);
        assert_eq!(v1.unit_map, ELECTRIC_CHARGE_MAP);
        assert_eq!(
            v1.v_electric_charge,
            Some(UnitElectricCharge::Coulomb(Metric::None))
        );
    }

    #[test]
    fn const_rydberg_test() {
        let v1 = const_rydberg();
        assert!(v1.val >= 10973731.568539);
        assert!(v1.val <= 10973731.568540);
        assert_eq!(v1.exp[LENGTH_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
    }

    #[test]
    fn const_plank_test() {
        let v1 = const_plank();
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
        let v1 = const_vacuum_permittivity();
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
