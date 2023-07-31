use std::{fmt::Display, str::FromStr};
use crate::{values::Value, errors::V3Error};
use crate::constants::*;

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nums:Vec<String> = vec![];
        let mut denoms:Vec<String> = vec![];

        for i in 0..31_usize {
            let region:usize = 1<<i;
            if self.unit_map & region == 0 {
                continue;
            }
            let u = match region {
                LENGTH_MAP => {
                    self.v_length.unwrap().to_string()
                }
                TIME_MAP => {
                    self.v_time.unwrap().to_string()
                }
                MASS_MAP => {
                    self.v_mass.unwrap().to_string()
                }
                ELECTRIC_CURRENT_MAP => {
                    self.v_electric_current.unwrap().to_string()
                }
                ELECTRIC_CHARGE_MAP => {
                    self.v_electric_charge.unwrap().to_string()
                }
                ELECTRIC_POTENTIAL_MAP => {
                    self.v_electric_potential.unwrap().to_string()
                }
                ELECTRIC_CONDUCTANCE_MAP => {
                    self.v_electric_conductance.unwrap().to_string()
                }
                CAPACITANCE_MAP => {
                    self.v_capacitance.unwrap().to_string()
                }
                RESISTANCE_MAP => {
                    self.v_resistance.unwrap().to_string()
                }
                INDUCTANCE_MAP => {
                    self.v_inductance.unwrap().to_string()
                }
                MAGNETIC_FLUX_MAP => {
                    self.v_magnetic_flux.unwrap().to_string()
                }
                MAGNETIC_FLUX_DENSITY_MAP => {
                    self.v_magnetic_flux_density.unwrap().to_string()
                }
                TEMPERATURE_MAP => {
                    self.v_temperature.unwrap().to_string()
                }
                SUBSTANCE_MAP => {
                    self.v_substance.unwrap().to_string()
                }
                LUMINOUS_INTENSITY_MAP => {
                    self.v_luminous_flux_intensity.unwrap().to_string()
                }
                LUMINOUS_FLUX_MAP => {
                    self.v_luminous_flux.unwrap().to_string()
                }
                ILLUMINANCE_MAP => {
                    self.v_illuminance.unwrap().to_string()
                }
                VOLUME_MAP => {
                    self.v_volume.unwrap().to_string()
                }
                PRESSURE_MAP => {
                    self.v_pressure.unwrap().to_string()
                }
                ANGLE_MAP => {
                    self.v_angle.unwrap().to_string()
                }
                FREQUENCY_MAP => {
                    self.v_frequency.unwrap().to_string()
                }
                FORCE_MAP => {
                    self.v_force.unwrap().to_string()
                }
                ENERGY_MAP => {
                    self.v_energy.unwrap().to_string()
                }
                POWER_MAP => {
                    self.v_power.unwrap().to_string()
                }
                RADIOACTIVITY_MAP => {
                    self.v_radioactivity.unwrap().to_string()
                }
                ABSORBED_DOSE_MAP => {
                    self.v_ab_dose.unwrap().to_string()
                }
                RADIOACTIVITY_EXPOSURE_MAP => {
                    self.v_radioactivity_exposure.unwrap().to_string()
                }
                CATALYTIC_ACTIVITY_MAP => {
                    self.v_catalytic.unwrap().to_string()
                }
                SOUND_MAP => {
                    self.v_sound.unwrap().to_string()
                }
                INFORMATION_MAP => {
                    self.v_information.unwrap().to_string()
                }
                SOLID_ANGLE_MAP => {
                    self.v_solid_angle.unwrap().to_string()
                }
                _ => {
                    panic!("[fmt] Unreachable");
                }
            };
            if self.exp[i] < -1 {
                denoms.push(format!("{}^{}", u, -self.exp[i]))
            } else if self.exp[i] > 1 {
                nums.push(format!("{}^{}", u, self.exp[i]))
            } else if self.exp[i] == 1 {
                nums.push(u);
            } else if self.exp[i] == -1 {
                denoms.push(u);
            }
        }

        let final_num = nums.join("*");
        let final_denom = denoms.join("*");

        let final_str:String;

        if !final_num.is_empty() && !final_denom.is_empty() {
            final_str = format!("{}/{}", final_num, final_denom);
        } else if !final_num.is_empty() && final_denom.is_empty() {
            final_str = final_num;
        } else if final_num.is_empty() && !final_denom.is_empty() {
            final_str = format!("1/{}", final_denom);
        } else {
            final_str = String::from("");
        }

        write!(f, "{} {}", self.val, final_str)
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        let temp:Value = match Value::new(1.0, other) {
            Ok(t) => t,
            Err(_) => return false // TODO : Report Error?
        };
        self.__equal(&temp)
    }
}

impl FromStr for Value {
    type Err = V3Error;
    fn from_str(s:&str) -> Result<Value, V3Error> {
        if !s.contains(char::is_whitespace) {
            let val:Value = match s.parse::<f64>() {
                Ok(t) => Value::new(t, "").unwrap(),
                Err(_) => {
                    return Err(V3Error::ParsingError("[from_str] float conversion"));
                }
            };
            return Ok(val);
        }
        let temp:Vec<&str> = s.split(' ').collect();
        let v:f64 = match temp[0].parse::<f64>() {
            Ok(t) => t,
            Err(_) => {
                    return Err(V3Error::ParsingError("[from_str] float conversion"));
            }
        };
        Value::new(v, temp[1])
    }
}
