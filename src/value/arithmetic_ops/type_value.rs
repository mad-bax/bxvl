use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::consts::*;
use crate::units::Convert;
use crate::value::Value;

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, other: Value) -> Value {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map != TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot Add values {} and {}", self, other);
        }

        if !self.__equivalent(&other) {
            panic!("Cannot Add values {} and {}", self, other);
        }

        let mut cmp_val: f64 = other.val;

        for i in 0..31_usize {
            let region: usize = 1 << i;
            if region & self.unit_map != 0 {
                match region {
                    LENGTH_MAP => {
                        if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(self.exp[LENGTH_INDEX]);
                        }
                    }
                    TIME_MAP => {
                        if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(self.exp[TIME_INDEX]);
                        }
                    }
                    MASS_MAP => {
                        if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(self.exp[MASS_INDEX]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(self.exp[ELECTRIC_CURRENT_INDEX]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(self.exp[ELECTRIC_CHARGE_INDEX]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(self.exp[ELECTRIC_POTENTIAL_INDEX]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(self.exp[ELECTRIC_CONDUCTANCE_INDEX]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(self.exp[CAPACITANCE_INDEX]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(self.exp[RESISTANCE_INDEX]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(self.exp[INDUCTANCE_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(self.exp[TEMPERATURE_INDEX]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(self.exp[SUBSTANCE_INDEX]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(self.exp[LUMINOUS_FLUX_INDEX]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(self.exp[ILLUMINANCE_INDEX]);
                        }
                    }
                    VOLUME_MAP => {
                        if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(self.exp[VOLUME_INDEX]);
                        }
                    }
                    PRESSURE_MAP => {
                        if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(self.exp[PRESSURE_INDEX]);
                        }
                    }
                    ANGLE_MAP => {
                        if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(self.exp[ANGLE_INDEX]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(self.exp[FREQUENCY_INDEX]);
                        }
                    }
                    FORCE_MAP => {
                        if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(self.exp[FORCE_INDEX]);
                        }
                    }
                    ENERGY_MAP => {
                        if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(self.exp[ENERGY_INDEX]);
                        }
                    }
                    POWER_MAP => {
                        if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(self.exp[POWER_INDEX]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(self.exp[RADIOACTIVITY_INDEX]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(self.exp[ABSORBED_DOSE_INDEX]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(self.exp[CATALYTIC_ACTIVITY_INDEX]);
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(self.exp[SOUND_INDEX]);
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(self.exp[INFORMATION_INDEX]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(self.exp[SOLID_ANGLE_INDEX]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot Add values {} and {}", self, other);
                    }
                }
            }
        }

        let mut n: Value = self;
        n.val += cmp_val;
        n
    }
}

impl AddAssign<Value> for Value {
    fn add_assign(&mut self, other: Value) {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map > TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot AddAssign values {} and {}", self, other);
        }

        if !self.__equivalent(&other) {
            panic!("Cannot Add values {} and {}", self, other);
        }

        let mut cmp_val: f64 = other.val;

        for i in 0..31_usize {
            let region: usize = 1 << i;
            if region & self.unit_map != 0 {
                match region {
                    LENGTH_MAP => {
                        if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(self.exp[LENGTH_INDEX]);
                        }
                    }
                    TIME_MAP => {
                        if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(self.exp[TIME_INDEX]);
                        }
                    }
                    MASS_MAP => {
                        if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(self.exp[MASS_INDEX]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(self.exp[ELECTRIC_CURRENT_INDEX]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(self.exp[ELECTRIC_CHARGE_INDEX]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(self.exp[ELECTRIC_POTENTIAL_INDEX]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(self.exp[ELECTRIC_CONDUCTANCE_INDEX]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(self.exp[CAPACITANCE_INDEX]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(self.exp[RESISTANCE_INDEX]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(self.exp[INDUCTANCE_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(self.exp[TEMPERATURE_INDEX]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(self.exp[SUBSTANCE_INDEX]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(self.exp[LUMINOUS_FLUX_INDEX]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(self.exp[ILLUMINANCE_INDEX]);
                        }
                    }
                    VOLUME_MAP => {
                        if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(self.exp[VOLUME_INDEX]);
                        }
                    }
                    PRESSURE_MAP => {
                        if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(self.exp[PRESSURE_INDEX]);
                        }
                    }
                    ANGLE_MAP => {
                        if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(self.exp[ANGLE_INDEX]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(self.exp[FREQUENCY_INDEX]);
                        }
                    }
                    FORCE_MAP => {
                        if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(self.exp[FORCE_INDEX]);
                        }
                    }
                    ENERGY_MAP => {
                        if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(self.exp[ENERGY_INDEX]);
                        }
                    }
                    POWER_MAP => {
                        if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(self.exp[POWER_INDEX]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(self.exp[RADIOACTIVITY_INDEX]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(self.exp[ABSORBED_DOSE_INDEX]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(self.exp[CATALYTIC_ACTIVITY_INDEX]);
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(self.exp[SOUND_INDEX]);
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(self.exp[INFORMATION_INDEX]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(self.exp[SOLID_ANGLE_INDEX]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot AddAssign values {} and {}", self, other);
                    }
                }
            }
        }

        self.val += cmp_val;
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Value {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map > TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot Sub values {} and {}", self, other);
        }

        if !self.__equivalent(&other) {
            panic!("Cannot Sub values {} and {}", self, other);
        }

        let mut cmp_val: f64 = other.val;

        for i in 0..31_usize {
            let region: usize = 1 << i;
            if region & self.unit_map != 0 {
                match region {
                    LENGTH_MAP => {
                        if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(self.exp[LENGTH_INDEX]);
                        }
                    }
                    TIME_MAP => {
                        if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(self.exp[TIME_INDEX]);
                        }
                    }
                    MASS_MAP => {
                        if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(self.exp[MASS_INDEX]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(self.exp[ELECTRIC_CURRENT_INDEX]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(self.exp[ELECTRIC_CHARGE_INDEX]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(self.exp[ELECTRIC_POTENTIAL_INDEX]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(self.exp[ELECTRIC_CONDUCTANCE_INDEX]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(self.exp[CAPACITANCE_INDEX]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(self.exp[RESISTANCE_INDEX]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(self.exp[INDUCTANCE_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(self.exp[TEMPERATURE_INDEX]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(self.exp[SUBSTANCE_INDEX]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(self.exp[LUMINOUS_FLUX_INDEX]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(self.exp[ILLUMINANCE_INDEX]);
                        }
                    }
                    VOLUME_MAP => {
                        if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(self.exp[VOLUME_INDEX]);
                        }
                    }
                    PRESSURE_MAP => {
                        if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(self.exp[PRESSURE_INDEX]);
                        }
                    }
                    ANGLE_MAP => {
                        if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(self.exp[ANGLE_INDEX]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(self.exp[FREQUENCY_INDEX]);
                        }
                    }
                    FORCE_MAP => {
                        if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(self.exp[FORCE_INDEX]);
                        }
                    }
                    ENERGY_MAP => {
                        if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(self.exp[ENERGY_INDEX]);
                        }
                    }
                    POWER_MAP => {
                        if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(self.exp[POWER_INDEX]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(self.exp[RADIOACTIVITY_INDEX]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(self.exp[ABSORBED_DOSE_INDEX]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(self.exp[CATALYTIC_ACTIVITY_INDEX]);
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(self.exp[SOUND_INDEX]);
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(self.exp[INFORMATION_INDEX]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(self.exp[SOLID_ANGLE_INDEX]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot Sub values {} and {}", self, other);
                    }
                }
            }
        }

        let mut n: Value = self;
        n.val -= cmp_val;
        n
    }
}

impl SubAssign<Value> for Value {
    fn sub_assign(&mut self, other: Value) {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map > TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot SubAssign values {} and {}", self, other);
        }

        if !self.__equivalent(&other) {
            panic!("Cannot SubAssign values {} and {}", self, other);
        }

        let mut cmp_val: f64 = other.val;

        for i in 0..31_usize {
            let region: usize = 1 << i;
            if region & self.unit_map != 0 {
                match region {
                    LENGTH_MAP => {
                        if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(self.exp[LENGTH_INDEX]);
                        }
                    }
                    TIME_MAP => {
                        if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(self.exp[TIME_INDEX]);
                        }
                    }
                    MASS_MAP => {
                        if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(self.exp[MASS_INDEX]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(self.exp[ELECTRIC_CURRENT_INDEX]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(self.exp[ELECTRIC_CHARGE_INDEX]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(self.exp[ELECTRIC_POTENTIAL_INDEX]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(self.exp[ELECTRIC_CONDUCTANCE_INDEX]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(self.exp[CAPACITANCE_INDEX]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(self.exp[RESISTANCE_INDEX]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(self.exp[INDUCTANCE_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_INDEX]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(self.exp[TEMPERATURE_INDEX]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(self.exp[SUBSTANCE_INDEX]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(self.exp[LUMINOUS_FLUX_INDEX]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(self.exp[ILLUMINANCE_INDEX]);
                        }
                    }
                    VOLUME_MAP => {
                        if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(self.exp[VOLUME_INDEX]);
                        }
                    }
                    PRESSURE_MAP => {
                        if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(self.exp[PRESSURE_INDEX]);
                        }
                    }
                    ANGLE_MAP => {
                        if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(self.exp[ANGLE_INDEX]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(self.exp[FREQUENCY_INDEX]);
                        }
                    }
                    FORCE_MAP => {
                        if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(self.exp[FORCE_INDEX]);
                        }
                    }
                    ENERGY_MAP => {
                        if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(self.exp[ENERGY_INDEX]);
                        }
                    }
                    POWER_MAP => {
                        if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(self.exp[POWER_INDEX]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(self.exp[RADIOACTIVITY_INDEX]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(self.exp[ABSORBED_DOSE_INDEX]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(self.exp[CATALYTIC_ACTIVITY_INDEX]);
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(self.exp[SOUND_INDEX]);
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(self.exp[INFORMATION_INDEX]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(self.exp[SOLID_ANGLE_INDEX]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot SubAssign values {} and {}", self, other);
                    }
                }
            }
        }

        self.val -= cmp_val;
    }
}

impl Mul<Value> for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Value {
        let mut n: Value = self;
        n.unit_map = 0;

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map > TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot Mul values {} and {}", self, other);
        }

        if other.is_radians() && !self.is_angle() {
            n.val *= other.val;
            n.unit_map = self.unit_map;
            return n;
        } else if self.is_radians() && !other.is_angle() {
            n.val *= other.val;
            n.unit_map = other.unit_map;
            return n;
        }

        let mut cmp_val: f64 = other.val;
        for i in 0..31_usize {
            n.exp[i] = self.exp[i] + other.exp[i];
            let region: usize = 1 << i;
            let in_other: bool = region & other.unit_map != 0;
            let in_self: bool = self.unit_map & region != 0;
            let must_assign: bool = !in_self && in_other;

            if n.exp[i] != 0 {
                n.unit_map |= region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            n.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            n.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            n.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            n.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            n.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            n.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            n.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            n.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            n.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            n.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            n.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            n.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            n.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(other.exp[i]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            n.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            n.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity
                        {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            n.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            n.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            n.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            n.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            n.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            n.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FORCE_MAP => {
                        if must_assign {
                            n.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            n.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            n.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            n.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            n.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            n.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            n.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            n.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            n.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            n.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot Mul values {} and {}", self, other);
                    }
                }
            }
        }
        n.val *= cmp_val;
        n
    }
}

impl MulAssign<Value> for Value {
    fn mul_assign(&mut self, other: Value) {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map > TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot MulAssign values {} and {}", self, other);
        }

        if other.is_radians() && !self.is_angle() {
            self.val *= other.val;
            return;
        } else if self.is_radians() && !other.is_angle() {
            let t: f64 = self.val;
            *self = other;
            self.val *= t;
            return;
        }

        let mut cmp_val: f64 = other.val;
        for i in 0..31_usize {
            self.exp[i] += other.exp[i];
            let region: usize = 1 << i;

            let in_other: bool = region & other.unit_map != 0;
            let in_self: bool = self.unit_map & region != 0;
            let must_assign: bool = !in_self && in_other;

            if self.exp[i] != 0 {
                self.unit_map |= region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            self.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            self.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            self.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            self.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            self.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            self.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            self.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            self.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            self.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            self.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            self.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            self.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            self.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(other.exp[i]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            self.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            self.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity
                        {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            self.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            self.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            self.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            self.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            self.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            self.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FORCE_MAP => {
                        if must_assign {
                            self.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            self.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            self.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            self.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            self.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            self.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            self.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            self.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            self.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            self.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot MulAssign values {} and {}", self, other);
                    }
                }
            }
        }
        self.val *= cmp_val;
    }
}

impl Div<Value> for Value {
    type Output = Value;
    fn div(self, other: Value) -> Value {
        let mut n: Value = self;
        n.unit_map = 0;

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map > TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot Div values {} and {}", self, other);
        }

        if other.is_radians() && !self.is_angle() {
            n.val /= other.val;
            n.unit_map = self.unit_map;
            return n;
        } else if self.is_radians() && !other.is_angle() {
            n.val /= other.val;
            n.unit_map = other.unit_map;
            return n;
        }

        let mut cmp_val: f64 = other.val;
        for i in 0..31_usize {
            n.exp[i] = self.exp[i] - other.exp[i];
            let region: usize = 1 << i;
            let in_other: bool = region & other.unit_map != 0;
            let in_self: bool = self.unit_map & region != 0;
            let must_assign: bool = !in_self && in_other;

            if n.exp[i] != 0 {
                n.unit_map |= region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            n.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            n.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            n.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            n.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            n.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            n.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            n.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            n.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            n.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            n.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            n.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            n.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            n.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(other.exp[i]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            n.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            n.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity
                        {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            n.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            n.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            n.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            n.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            n.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            n.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FORCE_MAP => {
                        if must_assign {
                            n.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            n.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            n.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            n.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            n.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            n.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            n.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            n.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            n.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            n.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot Div values {} and {}", self, other);
                    }
                }
            }
        }
        n.val /= cmp_val;
        n
    }
}

impl DivAssign<Value> for Value {
    fn div_assign(&mut self, other: Value) {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map > TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            // Error cannot convert as part of larger unit
            panic!("Cannot DivAssign values {} and {}", self, other);
        }

        if other.is_radians() && !self.is_angle() {
            self.val /= other.val;
            return;
        } else if self.is_radians() && !other.is_angle() {
            let t: f64 = self.val;
            *self = other;
            self.val *= 1.0 / t; // TODO : How to divide radian by value?
            return;
        }

        let mut cmp_val: f64 = other.val;
        for i in 0..31_usize {
            self.exp[i] -= other.exp[i];
            let region: usize = 1 << i;
            let in_other: bool = region & other.unit_map != 0;
            let in_self: bool = self.unit_map & region != 0;
            let must_assign: bool = !in_self && in_other;

            if self.exp[i] != 0 {
                self.unit_map |= region;
            } else {
                self.unit_map &= !region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            self.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other
                                .v_length
                                .unwrap()
                                .convert(&self.v_length.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            self.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other
                                .v_time
                                .unwrap()
                                .convert(&self.v_time.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            self.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other
                                .v_mass
                                .unwrap()
                                .convert(&self.v_mass.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            self.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            self.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            self.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            self.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            self.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            self.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            self.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            self.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            self.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            self.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val)
                                .powi(other.exp[i]);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            self.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            self.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity
                        {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            self.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            self.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            self.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other
                                .v_volume
                                .unwrap()
                                .convert(&self.v_volume.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            self.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other
                                .v_pressure
                                .unwrap()
                                .convert(&self.v_pressure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            self.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other
                                .v_angle
                                .unwrap()
                                .convert(&self.v_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            self.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    FORCE_MAP => {
                        if must_assign {
                            self.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other
                                .v_force
                                .unwrap()
                                .convert(&self.v_force.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            self.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other
                                .v_energy
                                .unwrap()
                                .convert(&self.v_energy.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            self.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other
                                .v_power
                                .unwrap()
                                .convert(&self.v_power.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            self.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            self.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other
                                .v_ab_dose
                                .unwrap()
                                .convert(&self.v_ab_dose.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            self.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            self.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            self.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other
                                .v_sound
                                .unwrap()
                                .convert(&self.v_sound.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            self.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            self.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap())
                                .powi(other.exp[i]);
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot DivAssign values {} and {}", self, other);
                    }
                }
            }
        }
        self.val /= cmp_val;
    }
}

#[cfg(test)]
mod arithmetic_ops_testing {
    use crate::units::{
        Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
        UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
        UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency,
        UnitIlluminance, UnitInformation, UnitLength, UnitLuminousFlux, UnitLuminousIntensity,
        UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure,
        UnitRadioactivity, UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance,
        UnitTemperature, UnitTime, UnitVolume,
    };

    #[test]
    fn add() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        let v3 = v1 + v2;
        assert_eq!(v1 + 4.0, v3);
    }

    #[test]
    fn add_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        v1 += v2;
        assert_eq!(v1, 16.0);
    }

    #[test]
    fn sub() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        let v3 = v1 - v2;
        assert_eq!(v1 - 4.0, v3);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        v1 -= v2;
        assert_eq!(v1, 8.0);
    }

    #[test]
    fn mul() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        let v3 = v1 * v2;
        assert_eq!(v1 * 4.0, v3.val);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        v1 *= v2;
        assert_eq!(v1, 48.0);
    }

    #[test]
    fn div() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        let v3 = v1 / v2;
        assert_eq!(v1 / 4.0, v3.val);
    }

    #[test]
    fn div_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = 4.0 * UnitLength::Meter(Metric::None);
        v1 /= v2;
        assert_eq!(v1, 3.0);
    }

    #[test]
    #[should_panic]
    fn value_bad_add() {
        let t1 = 4.0 * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTime::Second(Metric::None);

        let _ = t1 + t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_sub() {
        let t1 = 4.0 * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTime::Second(Metric::None);

        let _ = t1 - t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_add_mut() {
        let mut t1 = 4.0 * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTime::Second(Metric::None);

        t1 += t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_sub_mut() {
        let mut t1 = 4.0 * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTime::Second(Metric::None);

        t1 -= t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_add_exp() {
        let t1 = 4.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitLength::Meter(Metric::None);

        let _ = t1 + t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_sub_exp() {
        let t1 = 4.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitLength::Meter(Metric::None);

        let _ = t1 - t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_add_exp_mut() {
        let mut t1 = 4.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitLength::Meter(Metric::None);

        t1 += t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_sub_exp_mut() {
        let mut t1 = 4.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitLength::Meter(Metric::None);

        t1 -= t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_add_temp() {
        let t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        let _ = t1 + t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_sub_temp() {
        let t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        let _ = t1 - t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_add_temp_mut() {
        let mut t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        t1 += t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_sub_temp_mut() {
        let mut t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        t1 -= t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_mul_temp() {
        let t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        let _ = t1 * t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_div_temp() {
        let t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        let _ = t1 / t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_mul_temp_mut() {
        let mut t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        t1 *= t2;
    }

    #[test]
    #[should_panic]
    fn value_bad_div_temp_mut() {
        let mut t1 = 4.0 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitTemperature::Fahrenheit * UnitLength::Meter(Metric::None);

        t1 /= t2;
    }

    #[test]
    fn value_must_assigns() {
        let t2 = 16.0 * UnitTemperature::Kelvin(Metric::None);

        let mut t1 = 4.0 * UnitSound::Bel(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitSound::Bel(Metric::None);

        let mut t1 = 4.0 * UnitAngle::Radian(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitAngle::Radian(Metric::None);

        let mut t1 = 4.0 * UnitAbsorbedDose::Gray(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitAbsorbedDose::Gray(Metric::None);

        let mut t1 = 4.0 * UnitSolidAngle::Steradian(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitSolidAngle::Steradian(Metric::None);

        let mut t1 = 4.0 * UnitVolume::Liter(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitVolume::Liter(Metric::None);

        let mut t1 = 4.0 * UnitTime::Second(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitTime::Second(Metric::None);

        let mut t1 = 4.0 * UnitSubstance::Mole(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitSubstance::Mole(Metric::None);

        let mut t1 = 4.0 * UnitMass::Gram(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitMass::Gram(Metric::None);

        let mut t1 = 4.0 * UnitRadioactivity::Becquerel(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitRadioactivity::Becquerel(Metric::None);

        let mut t1 = 4.0 * UnitRadioactivityExposure::Sievert(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitRadioactivityExposure::Sievert(Metric::None);

        let mut t1 = 4.0 * UnitPower::Watt(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitPower::Watt(Metric::None);

        let mut t1 = 4.0 * UnitPressure::Bar(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitPressure::Bar(Metric::None);

        let mut t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitMagneticFluxDensity::Tesla(Metric::None);

        let mut t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitMagneticFlux::Weber(Metric::None);

        let mut t1 = 4.0 * UnitLuminousIntensity::Candela(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitLuminousIntensity::Candela(Metric::None);

        let mut t1 = 4.0 * UnitLuminousFlux::Lumen(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitLuminousFlux::Lumen(Metric::None);

        let mut t1 = 4.0 * UnitInformation::Byte(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 2.0 * UnitInformation::Byte(Metric::None);

        let mut t1 = 4.0 * UnitIlluminance::Lux(Metric::None);
        assert_eq!(t1 * t2, 8.0);
        assert_eq!(t1 / t2, 2.0);
        t1 *= t2;
        assert_eq!(t1, 8.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitIlluminance::Lux(Metric::None);

        let mut t1 = 4.0 * UnitFrequency::Hertz(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitFrequency::Hertz(Metric::None);

        let mut t1 = 4.0 * UnitForce::Newton(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitForce::Newton(Metric::None);

        let mut t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitEnergy::Joule(Metric::None);

        let mut t1 = 4.0 * UnitElectricResistance::Ohm(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitElectricResistance::Ohm(Metric::None);

        let mut t1 = 4.0 * UnitElectricPotential::Volt(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitElectricPotential::Volt(Metric::None);

        let mut t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitElectricInductance::Henry(Metric::None);

        let mut t1 = 4.0 * UnitElectricCurrent::Ampere(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitElectricCurrent::Ampere(Metric::None);

        let mut t1 = 4.0 * UnitElectricConductance::Siemens(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitElectricConductance::Siemens(Metric::None);

        let mut t1 = 4.0 * UnitElectricCharge::Coulomb(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitElectricCharge::Coulomb(Metric::None);

        let mut t1 = 4.0 * UnitElectricCapacitance::Farad(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitElectricCapacitance::Farad(Metric::None);

        let mut t1 = 4.0 * UnitLength::Meter(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitLength::Meter(Metric::None);

        let mut t1 = 4.0 * UnitAbsorbedDose::Gray(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitAbsorbedDose::Gray(Metric::None);

        let mut t1 = 4.0 * UnitAngle::Radian(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitAngle::Radian(Metric::None);

        let mut t1 = 4.0 * UnitCatalyticActivity::Katal(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
        assert_eq!(t1 / t2, 4.0);
        t1 /= t2;
        assert_eq!(t1, 4.0);
        let t2 = 16.0 * UnitCatalyticActivity::Katal(Metric::None);

        let mut t1 = 4.0 * UnitTemperature::Kelvin(Metric::None);
        assert_eq!(t1 * t2, 64.0);
        assert_eq!(t2 / t1, 4.0);
        t1 *= t2;
        assert_eq!(t1, 64.0);
    }

    #[test]
    fn value_arithmetic() {
        let mut t1 = 4.0 * UnitTemperature::Kelvin(Metric::None);
        let t2 = 16.0 * UnitTemperature::Kelvin(Metric::None);
        let t3 = 8.0 * UnitTemperature::Kelvin(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitSound::Bel(Metric::None);
        let t2 = 16.0 * UnitSound::Bel(Metric::None);
        let t3 = 8.0 * UnitSound::Bel(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitSolidAngle::Steradian(Metric::None);
        let t2 = 16.0 * UnitSolidAngle::Steradian(Metric::None);
        let t3 = 8.0 * UnitSolidAngle::Steradian(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitVolume::Liter(Metric::None);
        let t2 = 16.0 * UnitVolume::Liter(Metric::None);
        let t3 = 8.0 * UnitVolume::Liter(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitTime::Second(Metric::None);
        let t2 = 16.0 * UnitTime::Second(Metric::None);
        let t3 = 8.0 * UnitTime::Second(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitSubstance::Mole(Metric::None);
        let t2 = 16.0 * UnitSubstance::Mole(Metric::None);
        let t3 = 8.0 * UnitSubstance::Mole(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitMass::Gram(Metric::None);
        let t2 = 16.0 * UnitMass::Gram(Metric::None);
        let t3 = 8.0 * UnitMass::Gram(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitRadioactivity::Becquerel(Metric::None);
        let t2 = 16.0 * UnitRadioactivity::Becquerel(Metric::None);
        let t3 = 8.0 * UnitRadioactivity::Becquerel(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitRadioactivityExposure::Sievert(Metric::None);
        let t2 = 16.0 * UnitRadioactivityExposure::Sievert(Metric::None);
        let t3 = 8.0 * UnitRadioactivityExposure::Sievert(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitPower::Watt(Metric::None);
        let t2 = 16.0 * UnitPower::Watt(Metric::None);
        let t3 = 8.0 * UnitPower::Watt(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitPressure::Bar(Metric::None);
        let t2 = 16.0 * UnitPressure::Bar(Metric::None);
        let t3 = 8.0 * UnitPressure::Bar(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t2 = 16.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t3 = 8.0 * UnitMagneticFluxDensity::Tesla(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        let t2 = 16.0 * UnitMagneticFlux::Weber(Metric::None);
        let t3 = 8.0 * UnitMagneticFlux::Weber(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitLuminousIntensity::Candela(Metric::None);
        let t2 = 16.0 * UnitLuminousIntensity::Candela(Metric::None);
        let t3 = 8.0 * UnitLuminousIntensity::Candela(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitLuminousFlux::Lumen(Metric::None);
        let t2 = 16.0 * UnitLuminousFlux::Lumen(Metric::None);
        let t3 = 8.0 * UnitLuminousFlux::Lumen(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitInformation::Byte(Metric::None);
        let t2 = 2.0 * UnitInformation::Byte(Metric::None);
        let t3 = 8.0 * UnitInformation::Byte(Metric::Kilo);

        assert_eq!(t1 + t3, 8196.0);
        assert_eq!(t3 - t1, 7.99609375);
        assert_eq!(t1 + t2, 6.0);
        assert_eq!(t2 - t1, -2.0);
        t1 += t3;
        assert_eq!(t1, 8196.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32768.0);
        t1 *= t3;
        assert_eq!(t1, 32768.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitIlluminance::Lux(Metric::None);
        let t2 = 16.0 * UnitIlluminance::Lux(Metric::None);
        let t3 = 8.0 * UnitIlluminance::Lux(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitFrequency::Hertz(Metric::None);
        let t2 = 16.0 * UnitFrequency::Hertz(Metric::None);
        let t3 = 8.0 * UnitFrequency::Hertz(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitForce::Newton(Metric::None);
        let t2 = 16.0 * UnitForce::Newton(Metric::None);
        let t3 = 8.0 * UnitForce::Newton(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        let t2 = 16.0 * UnitEnergy::Joule(Metric::None);
        let t3 = 8.0 * UnitEnergy::Joule(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitElectricResistance::Ohm(Metric::None);
        let t2 = 16.0 * UnitElectricResistance::Ohm(Metric::None);
        let t3 = 8.0 * UnitElectricResistance::Ohm(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitElectricPotential::Volt(Metric::None);
        let t2 = 16.0 * UnitElectricPotential::Volt(Metric::None);
        let t3 = 8.0 * UnitElectricPotential::Volt(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        let t2 = 16.0 * UnitElectricInductance::Henry(Metric::None);
        let t3 = 8.0 * UnitElectricInductance::Henry(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitElectricCurrent::Ampere(Metric::None);
        let t2 = 16.0 * UnitElectricCurrent::Ampere(Metric::None);
        let t3 = 8.0 * UnitElectricCurrent::Ampere(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitElectricConductance::Siemens(Metric::None);
        let t2 = 16.0 * UnitElectricConductance::Siemens(Metric::None);
        let t3 = 8.0 * UnitElectricConductance::Siemens(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitElectricCharge::Coulomb(Metric::None);
        let t2 = 16.0 * UnitElectricCharge::Coulomb(Metric::None);
        let t3 = 8.0 * UnitElectricCharge::Coulomb(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitElectricCapacitance::Farad(Metric::None);
        let t2 = 16.0 * UnitElectricCapacitance::Farad(Metric::None);
        let t3 = 8.0 * UnitElectricCapacitance::Farad(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitLength::Meter(Metric::None);
        let t2 = 16.0 * UnitLength::Meter(Metric::None);
        let t3 = 8.0 * UnitLength::Meter(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitAbsorbedDose::Gray(Metric::None);
        let t2 = 16.0 * UnitAbsorbedDose::Gray(Metric::None);
        let t3 = 8.0 * UnitAbsorbedDose::Gray(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitAngle::Radian(Metric::None);
        let t2 = 16.0 * UnitAngle::Radian(Metric::None);
        let t3 = 8.0 * UnitAngle::Radian(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);

        let mut t1 = 4.0 * UnitCatalyticActivity::Katal(Metric::None);
        let t2 = 16.0 * UnitCatalyticActivity::Katal(Metric::None);
        let t3 = 8.0 * UnitCatalyticActivity::Katal(Metric::Kilo);

        assert_eq!(t1 + t3, 8004.0);
        assert_eq!(t3 - t1, 7.996);
        assert_eq!(t1 + t2, 20.0);
        assert_eq!(t2 - t1, 12.0);
        t1 += t3;
        assert_eq!(t1, 8004.0);
        t1 -= t3;
        assert_eq!(t1, 4.0);
        assert_eq!(t1 * t3, 32000.0);
        t1 *= t3;
        assert_eq!(t1, 32000.0);
        assert_eq!(t1 / t3, 4.0);
        t1 /= t3;
        assert_eq!(t1, 4.0);
    }
}
