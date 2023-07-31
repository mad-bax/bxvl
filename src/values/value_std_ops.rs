use crate::{values::Value, constants::*};

impl PartialEq<Value> for Value {
    fn eq(&self, other:&Value) -> bool {
        if !self.__equal(other) {
            return false;
        }

        self.val == other.val
    }
}

impl PartialEq<&mut Value> for Value {
    fn eq(&self, other:&&mut Value) -> bool {
        if !self.__equal(other) {
            return false;
        }

        self.val == other.val
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other:&Value) -> Option<std::cmp::Ordering> {
        if self.unit_map != other.unit_map {
            return None;
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map != TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                return None;
            }

            let region:usize = 1<<i;
            if region & self.unit_map != 0 {
                match region {
                    LENGTH_MAP => {
                        if self.v_length != other.v_length {
                            cmp_val *= other.v_length.unwrap().convert(&self.v_length.unwrap());
                        }
                    }
                    TIME_MAP => {
                        if self.v_time != other.v_time {
                            cmp_val *= other.v_time.unwrap().convert(&self.v_time.unwrap());
                        }
                    }
                    MASS_MAP => {
                        if self.v_mass != other.v_mass {
                            cmp_val *= other.v_mass.unwrap().convert(&self.v_mass.unwrap());
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
                        }
                    }
                    VOLUME_MAP => {
                        if self.v_volume != other.v_volume {
                            cmp_val *= other.v_volume.unwrap().convert(&self.v_volume.unwrap());
                        }
                    }
                    PRESSURE_MAP => {
                        if self.v_pressure != other.v_pressure {
                            cmp_val *= other.v_pressure.unwrap().convert(&self.v_pressure.unwrap());
                        }
                    }
                    ANGLE_MAP => {
                        if self.v_angle != other.v_angle {
                            cmp_val *= other.v_angle.unwrap().convert(&self.v_angle.unwrap());
                        }
                    }
                    FREQUENCY_MAP => {
                        if self.v_frequency != other.v_frequency {
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
                        }
                    }
                    FORCE_MAP => {
                        if self.v_force != other.v_force {
                            cmp_val *= other.v_force.unwrap().convert(&self.v_force.unwrap());
                        }
                    }
                    ENERGY_MAP => {
                        if self.v_energy != other.v_energy {
                            cmp_val *= other.v_energy.unwrap().convert(&self.v_energy.unwrap());
                        }
                    }
                    POWER_MAP => {
                        if self.v_power != other.v_power {
                            cmp_val *= other.v_power.unwrap().convert(&self.v_power.unwrap());
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot compare Values {} and {}", self, other);
                    }
                }
            }
        }

        self.val.partial_cmp(&cmp_val)
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other:&f64) -> bool {
        self.val == *other
    }
}

impl PartialOrd<f64> for Value {
    fn partial_cmp(&self, other:&f64) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(other)
    }
}
