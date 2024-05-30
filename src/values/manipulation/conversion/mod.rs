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
pub mod power;
pub mod pressure;
pub mod radiation_absorbed_dose;
pub mod radiation_equivalent_dose;
pub mod radioactivity;
pub mod sound;
pub mod substance;
pub mod temperature;
pub mod time;
pub mod volume;

use std::ops::{Shr, ShrAssign};

use crate::{
    constants::{
        ABSORBED_DOSE_MAP, ANGLE_INDEX, ANGLE_MAP, CAPACITANCE_MAP, CATALYTIC_ACTIVITY_MAP,
        ELECTRIC_CHARGE_MAP, ELECTRIC_CONDUCTANCE_MAP, ELECTRIC_CURRENT_MAP,
        ELECTRIC_POTENTIAL_MAP, ENERGY_MAP, FORCE_MAP, FREQUENCY_INDEX, FREQUENCY_MAP,
        ILLUMINANCE_MAP, INDUCTANCE_MAP, INFORMATION_MAP, LENGTH_INDEX, LENGTH_MAP,
        LUMINOUS_FLUX_MAP, LUMINOUS_INTENSITY_MAP, MAGNETIC_FLUX_DENSITY_MAP, MAGNETIC_FLUX_MAP,
        MASS_MAP, POWER_MAP, PRESSURE_MAP, RADIOACTIVITY_EXPOSURE_MAP, RADIOACTIVITY_MAP,
        RESISTANCE_MAP, SOLID_ANGLE_MAP, SOUND_MAP, SUBSTANCE_MAP, TEMPERATURE_MAP, TIME_INDEX,
        TIME_MAP, VOLUME_INDEX, VOLUME_MAP,
    },
    errors::V3Error,
    units::{Convert, Metric, UnitAngle, UnitLength},
    values::Value,
};

impl Shr<Value> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: Value) -> Self::Output {
        if self.__equivalent(&other) {
            let mut ret: Value = self;
            ret._convert(&other)?;
            return Ok(ret);
        }
        Err(V3Error::ValueConversionError("[shr] Incompatible types"))
    }
}

impl Shr<&str> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: &str) -> Self::Output {
        let n: Value = Value::new(1.0, other)?;
        self >> n
    }
}

impl Shr<String> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: String) -> Self::Output {
        let n: Value = Value::new(1.0, other.as_str())?;
        self >> n
    }
}

impl ShrAssign<Value> for Value {
    fn shr_assign(&mut self, other: Value) {
        if self.__equivalent(&other) {
            match self._convert(&other) {
                Ok(_) => {}
                Err(_) => panic!("[shr_assign] Incompatible value types: {}, {}", self, other),
            }
        } else {
            panic!("[shr_assign] Incompatible value types: {}, {}", self, other);
        }
    }
}

impl ShrAssign<&str> for Value {
    fn shr_assign(&mut self, other: &str) {
        let n: Value = match Value::new(1.0, other) {
            Ok(t) => t,
            Err(_) => panic!("[shr_assign] Incompatible value types"),
        };
        *self >>= n;
    }
}

impl ShrAssign<String> for Value {
    fn shr_assign(&mut self, other: String) {
        let n: Value = match Value::new(1.0, other.as_str()) {
            Ok(t) => t,
            Err(_) => panic!("[shr_assign] Incompatible value types"),
        };
        *self >>= n;
    }
}

impl Value {
    /// Convert a [`Value`] to another of the same base unit types.
    ///
    /// `convert` uses a `&str` as an argument and parses it into the relevant units.
    /// The parsed `&str` must be over the same unit types as the [`Value`] to be converted.
    ///
    /// e.g. m/s and ft/hour, or J/kg and cal/lbs
    ///
    /// The given string *must* be representative of the full unit to convert to.
    ///
    /// e.g. `20 miles/hr` must be converted with `km/hr` or `kph` not just `km`
    ///
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// let mut speed:Value = match Value::new(20.0, "mph") {
    ///     Ok(t) => t,
    ///     Err(e) => panic!("{}", e)
    /// };
    ///
    /// match speed.convert("kph") {
    ///     Ok(_) => {}
    ///     Err(e) => panic!("{}", e)
    /// }
    /// ```
    pub fn convert(&mut self, other: &str) -> Result<(), V3Error> {
        let temp: Value = Value::new(0.0, other)?; // We make a temporary Value to make conversion easier
        self._convert(&temp)
    }

    /// Actual convert functionality with a given [`Value`] argument
    pub(in crate::values) fn _convert(&mut self, other: &Value) -> Result<(), V3Error> {
        if self.unit_map == VOLUME_MAP && other.unit_map == LENGTH_MAP {
            if self.exp[VOLUME_INDEX] == 1 && other.exp[LENGTH_INDEX] == 3 {
                self.val *= self.v_volume.unwrap().convert(&other.v_length.unwrap());
                self.exp[LENGTH_INDEX] = 3;
                self.exp[VOLUME_INDEX] = 0;
                self.unit_map = LENGTH_MAP;
                self.v_volume = None;
                self.v_length = other.v_length;
                return Ok(());
            }
            return Err(V3Error::ValueConversionError(
                "[_convert] Error converting volume to cubic",
            ));
        } else if self.unit_map == LENGTH_MAP && other.unit_map == VOLUME_MAP {
            if self.exp[LENGTH_INDEX] == 3 && other.exp[VOLUME_INDEX] == 1 {
                self.val *= f64::powf(
                    self.v_length
                        .unwrap()
                        .convert(&UnitLength::Meter(Metric::None)),
                    3.0,
                );
                self.val *= self.v_length.unwrap().convert(&other.v_volume.unwrap());
                self.exp[LENGTH_INDEX] = 0;
                self.exp[VOLUME_INDEX] = 1;
                self.unit_map = VOLUME_MAP;
                self.v_volume = other.v_volume;
                self.v_length = None;
                return Ok(());
            }
            return Err(V3Error::ValueConversionError(
                "[_convert] Error converting cubic to volume",
            ));
        } else if self.unit_map == 0 && other.unit_map == ANGLE_MAP {
            if other.v_angle.unwrap() == UnitAngle::Radian(Metric::None) {
                self.exp[ANGLE_INDEX] = 1;
                self.unit_map = ANGLE_MAP;
                self.v_angle = Some(UnitAngle::Radian(Metric::None));
                return Ok(());
            }
            return Err(V3Error::ValueConversionError(
                "[_convert] Error converting unitless to radians",
            ));
        } else if self.unit_map == FREQUENCY_MAP
            && other.unit_map == TIME_MAP
            && self.exp[FREQUENCY_INDEX] == 1
            && other.exp[TIME_INDEX] == -1
        {
            self.exp[FREQUENCY_INDEX] = 0;
            self.exp[TIME_INDEX] = -1;
            self.unit_map &= !FREQUENCY_MAP;
            self.unit_map |= TIME_MAP;
            self.val *= self.v_frequency.unwrap().convert(&other.v_time.unwrap());
            self.v_frequency = None;
            self.v_time = other.v_time;
            return Ok(());
        } else if self.unit_map == TIME_MAP
            && other.unit_map == FREQUENCY_MAP
            && self.exp[TIME_INDEX] == -1
            && other.exp[FREQUENCY_INDEX] == 1
        {
            self.exp[FREQUENCY_INDEX] = 1;
            self.exp[TIME_INDEX] = 0;
            self.unit_map &= !TIME_MAP;
            self.unit_map |= FREQUENCY_MAP;
            self.val *= self.v_time.unwrap().convert(&other.v_frequency.unwrap());
            self.v_frequency = other.v_frequency;
            self.v_time = None;
            return Ok(());
        }

        if self.unit_map != other.unit_map {
            return Err(V3Error::ValueConversionError(
                "[_convert] Nonequivalent unit types",
            ));
        }

        // check against temperature
        if self.unit_map & TEMPERATURE_MAP < self.unit_map {
            if self.v_temperature != other.v_temperature {
                return Err(V3Error::ValueConversionError(
                    "[_convert] Temperature unit mismatch",
                ));
            }
        } else if self.unit_map == TEMPERATURE_MAP {
            self.val = self
                .v_temperature
                .unwrap()
                .convert(&other.v_temperature.unwrap(), self.val);
            self.v_temperature = other.v_temperature;
            return Ok(());
        }

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                return Err(V3Error::ValueConversionError(
                    "[_convert] Mismatched value exponents",
                ));
            }
            let region: usize = 1 << i;
            if region & self.unit_map != 0 {
                let tmp: f64;
                self.val *= f64::powi(
                    match region {
                        LENGTH_MAP => {
                            tmp = self.v_length.unwrap().convert(&other.v_length.unwrap());
                            self.v_length = other.v_length;
                            tmp
                        }
                        TIME_MAP => {
                            tmp = self.v_time.unwrap().convert(&other.v_time.unwrap());
                            self.v_time = other.v_time;
                            tmp
                        }
                        MASS_MAP => {
                            tmp = self.v_mass.unwrap().convert(&other.v_mass.unwrap());
                            self.v_mass = other.v_mass;
                            tmp
                        }
                        ELECTRIC_CURRENT_MAP => {
                            tmp = self
                                .v_electric_current
                                .unwrap()
                                .convert(&other.v_electric_current.unwrap());
                            self.v_electric_current = other.v_electric_current;
                            tmp
                        }
                        ELECTRIC_CHARGE_MAP => {
                            tmp = self
                                .v_electric_charge
                                .unwrap()
                                .convert(&other.v_electric_charge.unwrap());
                            self.v_electric_charge = other.v_electric_charge;
                            tmp
                        }
                        ELECTRIC_POTENTIAL_MAP => {
                            tmp = self
                                .v_electric_potential
                                .unwrap()
                                .convert(&other.v_electric_potential.unwrap());
                            self.v_electric_potential = other.v_electric_potential;
                            tmp
                        }
                        ELECTRIC_CONDUCTANCE_MAP => {
                            tmp = self
                                .v_electric_conductance
                                .unwrap()
                                .convert(&other.v_electric_conductance.unwrap());
                            self.v_electric_conductance = other.v_electric_conductance;
                            tmp
                        }
                        CAPACITANCE_MAP => {
                            tmp = self
                                .v_capacitance
                                .unwrap()
                                .convert(&other.v_capacitance.unwrap());
                            self.v_capacitance = other.v_capacitance;
                            tmp
                        }
                        RESISTANCE_MAP => {
                            tmp = self
                                .v_resistance
                                .unwrap()
                                .convert(&other.v_resistance.unwrap());
                            self.v_resistance = other.v_resistance;
                            tmp
                        }
                        INDUCTANCE_MAP => {
                            tmp = self
                                .v_inductance
                                .unwrap()
                                .convert(&other.v_inductance.unwrap());
                            self.v_inductance = other.v_inductance;
                            tmp
                        }
                        MAGNETIC_FLUX_MAP => {
                            tmp = self
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&other.v_magnetic_flux.unwrap());
                            self.v_magnetic_flux = other.v_magnetic_flux;
                            tmp
                        }
                        MAGNETIC_FLUX_DENSITY_MAP => {
                            tmp = self
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&other.v_magnetic_flux_density.unwrap());
                            self.v_magnetic_flux_density = other.v_magnetic_flux_density;
                            tmp
                        }
                        TEMPERATURE_MAP => unreachable!("This cannot be reached!"),
                        SUBSTANCE_MAP => {
                            tmp = self
                                .v_substance
                                .unwrap()
                                .convert(&other.v_substance.unwrap());
                            self.v_substance = other.v_substance;
                            tmp
                        }
                        LUMINOUS_INTENSITY_MAP => {
                            tmp = self
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&other.v_luminous_flux_intensity.unwrap());
                            self.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                            tmp
                        }
                        LUMINOUS_FLUX_MAP => {
                            tmp = self
                                .v_luminous_flux
                                .unwrap()
                                .convert(&other.v_luminous_flux.unwrap());
                            self.v_luminous_flux = other.v_luminous_flux;
                            tmp
                        }
                        ILLUMINANCE_MAP => {
                            tmp = self
                                .v_illuminance
                                .unwrap()
                                .convert(&other.v_illuminance.unwrap());
                            self.v_illuminance = other.v_illuminance;
                            tmp
                        }
                        VOLUME_MAP => {
                            tmp = self.v_volume.unwrap().convert(&other.v_volume.unwrap());
                            self.v_volume = other.v_volume;
                            tmp
                        }
                        PRESSURE_MAP => {
                            tmp = self.v_pressure.unwrap().convert(&other.v_pressure.unwrap());
                            self.v_pressure = other.v_pressure;
                            tmp
                        }
                        ANGLE_MAP => {
                            tmp = self.v_angle.unwrap().convert(&other.v_angle.unwrap());
                            self.v_angle = other.v_angle;
                            tmp
                        }
                        FREQUENCY_MAP => {
                            tmp = self
                                .v_frequency
                                .unwrap()
                                .convert(&other.v_frequency.unwrap());
                            self.v_frequency = other.v_frequency;
                            tmp
                        }
                        FORCE_MAP => {
                            tmp = self.v_force.unwrap().convert(&other.v_force.unwrap());
                            self.v_force = other.v_force;
                            tmp
                        }
                        ENERGY_MAP => {
                            tmp = self.v_energy.unwrap().convert(&other.v_energy.unwrap());
                            self.v_energy = other.v_energy;
                            tmp
                        }
                        POWER_MAP => {
                            tmp = self.v_power.unwrap().convert(&other.v_power.unwrap());
                            self.v_power = other.v_power;
                            tmp
                        }
                        RADIOACTIVITY_MAP => {
                            tmp = self
                                .v_radioactivity
                                .unwrap()
                                .convert(&other.v_radioactivity.unwrap());
                            self.v_radioactivity = other.v_radioactivity;
                            tmp
                        }
                        ABSORBED_DOSE_MAP => {
                            tmp = self.v_ab_dose.unwrap().convert(&other.v_ab_dose.unwrap());
                            self.v_ab_dose = other.v_ab_dose;
                            tmp
                        }
                        RADIOACTIVITY_EXPOSURE_MAP => {
                            tmp = self
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&other.v_radioactivity_exposure.unwrap());
                            self.v_radioactivity_exposure = other.v_radioactivity_exposure;
                            tmp
                        }
                        CATALYTIC_ACTIVITY_MAP => {
                            tmp = self
                                .v_catalytic
                                .unwrap()
                                .convert(&other.v_catalytic.unwrap());
                            self.v_catalytic = other.v_catalytic;
                            tmp
                        }
                        SOUND_MAP => {
                            tmp = self.v_sound.unwrap().convert(&other.v_sound.unwrap());
                            self.v_sound = other.v_sound;
                            tmp
                        }
                        INFORMATION_MAP => {
                            tmp = self
                                .v_information
                                .unwrap()
                                .convert(&other.v_information.unwrap());
                            self.v_information = other.v_information;
                            tmp
                        }
                        SOLID_ANGLE_MAP => {
                            tmp = self
                                .v_solid_angle
                                .unwrap()
                                .convert(&other.v_solid_angle.unwrap());
                            self.v_solid_angle = other.v_solid_angle;
                            tmp
                        }
                        _ => {
                            return Err(V3Error::UnknownError("[_convert] Value conversion"));
                        }
                    },
                    self.exp[i],
                );
            }
        }
        Ok(())
    }
}
