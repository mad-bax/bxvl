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
    consts::{
        ABSORBED_DOSE_INDEX, ABSORBED_DOSE_MAP, ANGLE_INDEX, ANGLE_MAP, CAPACITANCE_INDEX,
        CAPACITANCE_MAP, CATALYTIC_ACTIVITY_INDEX, CATALYTIC_ACTIVITY_MAP, ELECTRIC_CHARGE_INDEX,
        ELECTRIC_CHARGE_MAP, ELECTRIC_CONDUCTANCE_INDEX, ELECTRIC_CONDUCTANCE_MAP,
        ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP, ELECTRIC_POTENTIAL_INDEX,
        ELECTRIC_POTENTIAL_MAP, ENERGY_INDEX, ENERGY_MAP, FORCE_INDEX, FORCE_MAP, FREQUENCY_INDEX,
        FREQUENCY_MAP, ILLUMINANCE_INDEX, ILLUMINANCE_MAP, INDUCTANCE_INDEX, INDUCTANCE_MAP,
        INFORMATION_INDEX, INFORMATION_MAP, LENGTH_INDEX, LENGTH_MAP, LUMINOUS_FLUX_INDEX,
        LUMINOUS_FLUX_MAP, LUMINOUS_INTENSITY_INDEX, LUMINOUS_INTENSITY_MAP,
        MAGNETIC_FLUX_DENSITY_INDEX, MAGNETIC_FLUX_DENSITY_MAP, MAGNETIC_FLUX_INDEX,
        MAGNETIC_FLUX_MAP, MASS_INDEX, MASS_MAP, POWER_INDEX, POWER_MAP, PRESSURE_INDEX,
        PRESSURE_MAP, RADIOACTIVITY_EXPOSURE_INDEX, RADIOACTIVITY_EXPOSURE_MAP,
        RADIOACTIVITY_INDEX, RADIOACTIVITY_MAP, RESISTANCE_INDEX, RESISTANCE_MAP,
        SOLID_ANGLE_INDEX, SOLID_ANGLE_MAP, SOUND_INDEX, SOUND_MAP, SUBSTANCE_INDEX, SUBSTANCE_MAP,
        TEMPERATURE_INDEX, TEMPERATURE_MAP, TIME_INDEX, TIME_MAP, VOLUME_INDEX, VOLUME_MAP,
    },
    errors::V3Error,
    units::{Convert, Metric, UnitAngle, UnitLength},
    value::Value,
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
    /// use bxvl::value::Value;
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
    pub(crate) fn _convert(&mut self, other: &Value) -> Result<(), V3Error> {
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
                .convert(&other.v_temperature.unwrap(), self.val)
                .powi(self.exp[TEMPERATURE_INDEX]);
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
                self.val *= match region {
                    LENGTH_MAP => {
                        tmp = self
                            .v_length
                            .unwrap()
                            .convert(&other.v_length.unwrap())
                            .powi(self.exp[LENGTH_INDEX]);
                        self.v_length = other.v_length;
                        tmp
                    }
                    TIME_MAP => {
                        tmp = self
                            .v_time
                            .unwrap()
                            .convert(&other.v_time.unwrap())
                            .powi(self.exp[TIME_INDEX]);
                        self.v_time = other.v_time;
                        tmp
                    }
                    MASS_MAP => {
                        tmp = self
                            .v_mass
                            .unwrap()
                            .convert(&other.v_mass.unwrap())
                            .powi(self.exp[MASS_INDEX]);
                        self.v_mass = other.v_mass;
                        tmp
                    }
                    ELECTRIC_CURRENT_MAP => {
                        tmp = self
                            .v_electric_current
                            .unwrap()
                            .convert(&other.v_electric_current.unwrap())
                            .powi(self.exp[ELECTRIC_CURRENT_INDEX]);
                        self.v_electric_current = other.v_electric_current;
                        tmp
                    }
                    ELECTRIC_CHARGE_MAP => {
                        tmp = self
                            .v_electric_charge
                            .unwrap()
                            .convert(&other.v_electric_charge.unwrap())
                            .powi(self.exp[ELECTRIC_CHARGE_INDEX]);
                        self.v_electric_charge = other.v_electric_charge;
                        tmp
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        tmp = self
                            .v_electric_potential
                            .unwrap()
                            .convert(&other.v_electric_potential.unwrap())
                            .powi(self.exp[ELECTRIC_POTENTIAL_INDEX]);
                        self.v_electric_potential = other.v_electric_potential;
                        tmp
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        tmp = self
                            .v_electric_conductance
                            .unwrap()
                            .convert(&other.v_electric_conductance.unwrap())
                            .powi(self.exp[ELECTRIC_CONDUCTANCE_INDEX]);
                        self.v_electric_conductance = other.v_electric_conductance;
                        tmp
                    }
                    CAPACITANCE_MAP => {
                        tmp = self
                            .v_capacitance
                            .unwrap()
                            .convert(&other.v_capacitance.unwrap())
                            .powi(self.exp[CAPACITANCE_INDEX]);
                        self.v_capacitance = other.v_capacitance;
                        tmp
                    }
                    RESISTANCE_MAP => {
                        tmp = self
                            .v_resistance
                            .unwrap()
                            .convert(&other.v_resistance.unwrap())
                            .powi(self.exp[RESISTANCE_INDEX]);
                        self.v_resistance = other.v_resistance;
                        tmp
                    }
                    INDUCTANCE_MAP => {
                        tmp = self
                            .v_inductance
                            .unwrap()
                            .convert(&other.v_inductance.unwrap())
                            .powi(self.exp[INDUCTANCE_INDEX]);
                        self.v_inductance = other.v_inductance;
                        tmp
                    }
                    MAGNETIC_FLUX_MAP => {
                        tmp = self
                            .v_magnetic_flux
                            .unwrap()
                            .convert(&other.v_magnetic_flux.unwrap())
                            .powi(self.exp[MAGNETIC_FLUX_INDEX]);
                        self.v_magnetic_flux = other.v_magnetic_flux;
                        tmp
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        tmp = self
                            .v_magnetic_flux_density
                            .unwrap()
                            .convert(&other.v_magnetic_flux_density.unwrap())
                            .powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
                        self.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        tmp
                    }
                    TEMPERATURE_MAP => unreachable!("This cannot be reached!"),
                    SUBSTANCE_MAP => {
                        tmp = self
                            .v_substance
                            .unwrap()
                            .convert(&other.v_substance.unwrap())
                            .powi(self.exp[SUBSTANCE_INDEX]);
                        self.v_substance = other.v_substance;
                        tmp
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        tmp = self
                            .v_luminous_flux_intensity
                            .unwrap()
                            .convert(&other.v_luminous_flux_intensity.unwrap())
                            .powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
                        self.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        tmp
                    }
                    LUMINOUS_FLUX_MAP => {
                        tmp = self
                            .v_luminous_flux
                            .unwrap()
                            .convert(&other.v_luminous_flux.unwrap())
                            .powi(self.exp[LUMINOUS_FLUX_INDEX]);
                        self.v_luminous_flux = other.v_luminous_flux;
                        tmp
                    }
                    ILLUMINANCE_MAP => {
                        tmp = self
                            .v_illuminance
                            .unwrap()
                            .convert(&other.v_illuminance.unwrap())
                            .powi(self.exp[ILLUMINANCE_INDEX]);
                        self.v_illuminance = other.v_illuminance;
                        tmp
                    }
                    VOLUME_MAP => {
                        tmp = self
                            .v_volume
                            .unwrap()
                            .convert(&other.v_volume.unwrap())
                            .powi(self.exp[VOLUME_INDEX]);
                        self.v_volume = other.v_volume;
                        tmp
                    }
                    PRESSURE_MAP => {
                        tmp = self
                            .v_pressure
                            .unwrap()
                            .convert(&other.v_pressure.unwrap())
                            .powi(self.exp[PRESSURE_INDEX]);
                        self.v_pressure = other.v_pressure;
                        tmp
                    }
                    ANGLE_MAP => {
                        tmp = self
                            .v_angle
                            .unwrap()
                            .convert(&other.v_angle.unwrap())
                            .powi(self.exp[ANGLE_INDEX]);
                        self.v_angle = other.v_angle;
                        tmp
                    }
                    FREQUENCY_MAP => {
                        tmp = self
                            .v_frequency
                            .unwrap()
                            .convert(&other.v_frequency.unwrap())
                            .powi(self.exp[FREQUENCY_INDEX]);
                        self.v_frequency = other.v_frequency;
                        tmp
                    }
                    FORCE_MAP => {
                        tmp = self
                            .v_force
                            .unwrap()
                            .convert(&other.v_force.unwrap())
                            .powi(self.exp[FORCE_INDEX]);
                        self.v_force = other.v_force;
                        tmp
                    }
                    ENERGY_MAP => {
                        tmp = self
                            .v_energy
                            .unwrap()
                            .convert(&other.v_energy.unwrap())
                            .powi(self.exp[ENERGY_INDEX]);
                        self.v_energy = other.v_energy;
                        tmp
                    }
                    POWER_MAP => {
                        tmp = self
                            .v_power
                            .unwrap()
                            .convert(&other.v_power.unwrap())
                            .powi(self.exp[POWER_INDEX]);
                        self.v_power = other.v_power;
                        tmp
                    }
                    RADIOACTIVITY_MAP => {
                        tmp = self
                            .v_radioactivity
                            .unwrap()
                            .convert(&other.v_radioactivity.unwrap())
                            .powi(self.exp[RADIOACTIVITY_INDEX]);
                        self.v_radioactivity = other.v_radioactivity;
                        tmp
                    }
                    ABSORBED_DOSE_MAP => {
                        tmp = self
                            .v_ab_dose
                            .unwrap()
                            .convert(&other.v_ab_dose.unwrap())
                            .powi(self.exp[ABSORBED_DOSE_INDEX]);
                        self.v_ab_dose = other.v_ab_dose;
                        tmp
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        tmp = self
                            .v_radioactivity_exposure
                            .unwrap()
                            .convert(&other.v_radioactivity_exposure.unwrap())
                            .powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
                        self.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        tmp
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        tmp = self
                            .v_catalytic
                            .unwrap()
                            .convert(&other.v_catalytic.unwrap())
                            .powi(self.exp[CATALYTIC_ACTIVITY_INDEX]);
                        self.v_catalytic = other.v_catalytic;
                        tmp
                    }
                    SOUND_MAP => {
                        tmp = self
                            .v_sound
                            .unwrap()
                            .convert(&other.v_sound.unwrap())
                            .powi(self.exp[SOUND_INDEX]);
                        self.v_sound = other.v_sound;
                        tmp
                    }
                    INFORMATION_MAP => {
                        tmp = self
                            .v_information
                            .unwrap()
                            .convert(&other.v_information.unwrap())
                            .powi(self.exp[INFORMATION_INDEX]);
                        self.v_information = other.v_information;
                        tmp
                    }
                    SOLID_ANGLE_MAP => {
                        tmp = self
                            .v_solid_angle
                            .unwrap()
                            .convert(&other.v_solid_angle.unwrap())
                            .powi(self.exp[SOLID_ANGLE_INDEX]);
                        self.v_solid_angle = other.v_solid_angle;
                        tmp
                    }
                    _ => {
                        return Err(V3Error::UnknownError("[_convert] Value conversion"));
                    }
                };
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod conversion_testing {
    use crate::units::{
        Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
        UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
        UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency,
        UnitIlluminance, UnitInformation, UnitLength, UnitLuminousFlux, UnitLuminousIntensity,
        UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitNone, UnitPower, UnitPressure,
        UnitRadioactivity, UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance,
        UnitTemperature, UnitTime, UnitVolume,
    };

    #[test]
    fn explicit_convert() {
        let mut t1 = 4.5 * UnitLength::Foot;
        t1.convert("m").unwrap();
    }

    #[test]
    #[should_panic]
    fn explicit_convert_bad() {
        let mut t1 = 4.5 * UnitLength::Foot;
        t1.convert("zz").unwrap();
    }

    #[test]
    fn str_slide() {
        let t1 = 4.5 * UnitLength::Foot;
        let t2 = (t1 >> "m").unwrap();
        assert_eq!(t2.val, 1.3716000000000002);
    }

    #[test]
    #[should_panic]
    fn str_slide_bad() {
        let t1 = 4.5 * UnitLength::Foot;
        let _ = (t1 >> "zz").unwrap();
    }

    #[test]
    fn str_slide_mut() {
        let mut t1 = 4.5 * UnitLength::Foot;
        t1 >>= "m";
        assert_eq!(t1.val, 1.3716000000000002);
    }

    #[test]
    #[should_panic]
    fn str_slide_mut_bad() {
        let mut t1 = 4.5 * UnitLength::Foot;
        t1 >>= "zz";
    }

    #[test]
    fn string_slide() {
        let t1 = 4.5 * UnitLength::Foot;
        let t2 = (t1 >> String::from("m")).unwrap();
        assert_eq!(t2.val, 1.3716000000000002);
    }

    #[test]
    fn string_slide_mut() {
        let mut t1 = 4.5 * UnitLength::Foot;
        t1 >>= String::from("m");
        assert_eq!(t1.val, 1.3716000000000002);
    }

    #[test]
    #[should_panic]
    fn string_slide_mut_bad() {
        let mut t1 = 4.5 * UnitLength::Foot;
        t1 >>= String::from("zz");
    }

    #[test]
    #[should_panic]
    fn value_slide_incompatible_types() {
        let t1 = 4.5 * UnitLength::Foot;
        let t2 = 1.0 * UnitTime::Second(Metric::None);
        let _ = (t1 >> t2).unwrap();
    }

    #[test]
    #[should_panic]
    fn value_slide_incompatible_types_mut() {
        let mut t1 = 4.5 * UnitLength::Foot;
        let t2 = 1.0 * UnitTime::Second(Metric::None);
        t1 >>= t2;
    }

    #[test]
    #[should_panic]
    fn value_cubic_conversion_bad() {
        let mut t1 = 4.5 * UnitVolume::Liter(Metric::None);
        let t2 = 1.3 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);

        t1._convert(&t2).unwrap();
    }

    #[test]
    #[should_panic]
    fn value_volume_conversion_bad() {
        let t1 = 4.5 / UnitVolume::Liter(Metric::None);
        let mut t2 = 1.3
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None);

        t2._convert(&t1).unwrap();
    }

    #[test]
    fn non_val_to_angle() {
        let mut t1 = 4.5 * UnitNone::None;
        let t2 = 6.7 * UnitAngle::Radian(Metric::None);

        t1._convert(&t2).unwrap();
        assert!(t1.is_angle());

        let mut t1 = 4.5 * UnitNone::None;
        let t2 = 6.7 * UnitAngle::Radian(Metric::Micro);
        assert!(t1._convert(&t2).is_err());
    }

    #[test]
    fn freq_to_time() {
        let mut t1 = 4.5 * UnitFrequency::Hertz(Metric::None);
        let t2 = 1.0 / UnitTime::Second(Metric::None);

        t1 >>= t2;
        assert_eq!(t1.to_string(), "4.5 1/s");

        let mut t1 = 4.5 / UnitTime::Second(Metric::None);
        let t2 = 1.0 * UnitFrequency::Hertz(Metric::None);

        t1 >>= t2;
        assert_eq!(t1.to_string(), "4.5 Hz");
    }

    #[test]
    #[should_panic]
    fn bad_unit_maps_conversion() {
        let mut t1 = 4.5 * UnitTime::Second(Metric::None);
        let t2 = 1.5 * UnitLength::Meter(Metric::None);

        t1._convert(&t2).unwrap();
    }

    #[test]
    #[should_panic]
    fn temperature_conversion_multiple_types_bad() {
        let mut t1 = 4.5 * UnitLength::Meter(Metric::None) * UnitTemperature::Celsius(Metric::None);
        let t2 = 1.0 * UnitLength::Meter(Metric::None) * UnitTemperature::Kelvin(Metric::None);

        t1._convert(&t2).unwrap();
    }

    #[test]
    fn temperature_conversion() {
        let mut t1 = 4.5 * UnitTemperature::Celsius(Metric::None);
        let t2 = 1.0 * UnitTemperature::Kelvin(Metric::None);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "277.65 K");
    }

    #[test]
    #[should_panic]
    fn unit_exp_mismatch() {
        let mut t1 = 4.5 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 4.5 * UnitLength::Meter(Metric::None);

        t1._convert(&t2).unwrap();
    }

    #[test]
    fn standard_value_to_value_conversions() {
        let mut t1 = 450.0 * UnitMass::Gram(Metric::None);
        let t2 = 1.0 * UnitMass::Gram(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kg");

        let mut t1 = 450.0 * UnitElectricCurrent::Ampere(Metric::None);
        let t2 = 1.0 * UnitElectricCurrent::Ampere(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kA");

        let mut t1 = 450.0 * UnitElectricCharge::Coulomb(Metric::None);
        let t2 = 1.0 * UnitElectricCharge::Coulomb(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kC");

        let mut t1 = 450.0 * UnitAngle::Radian(Metric::None);
        let t2 = 1.0 * UnitAngle::Radian(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 krad");

        let mut t1 = 450.0 * UnitFrequency::Hertz(Metric::None);
        let t2 = 1.0 * UnitFrequency::Hertz(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kHz");

        let mut t1 = 450.0 * UnitTime::Second(Metric::None);
        let t2 = 1.0 * UnitTime::Second(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 ks");

        let mut t1 = 450.0 * UnitLength::Meter(Metric::None);
        let t2 = 1.0 * UnitLength::Meter(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 km");

        let mut t1 = 450.0 * UnitVolume::Liter(Metric::None);
        let t2 = 1.0 * UnitVolume::Liter(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kl");

        let mut t1 = 450.0 * UnitAbsorbedDose::Gray(Metric::None);
        let t2 = 1.0 * UnitAbsorbedDose::Gray(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kGy");

        let mut t1 = 450.0 * UnitCatalyticActivity::Katal(Metric::None);
        let t2 = 1.0 * UnitCatalyticActivity::Katal(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kkat");

        let mut t1 = 450.0 * UnitElectricCapacitance::Farad(Metric::None);
        let t2 = 1.0 * UnitElectricCapacitance::Farad(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kF");

        let mut t1 = 450.0 * UnitElectricConductance::Siemens(Metric::None);
        let t2 = 1.0 * UnitElectricConductance::Siemens(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kS");

        let mut t1 = 450.0 * UnitElectricInductance::Henry(Metric::None);
        let t2 = 1.0 * UnitElectricInductance::Henry(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kH");

        let mut t1 = 450.0 * UnitElectricPotential::Volt(Metric::None);
        let t2 = 1.0 * UnitElectricPotential::Volt(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kV");

        let mut t1 = 450.0 * UnitEnergy::Joule(Metric::None);
        let t2 = 1.0 * UnitEnergy::Joule(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kJ");

        let mut t1 = 450.0 * UnitElectricResistance::Ohm(Metric::None);
        let t2 = 1.0 * UnitElectricResistance::Ohm(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kΩ");

        let mut t1 = 450.0 * UnitForce::Newton(Metric::None);
        let t2 = 1.0 * UnitForce::Newton(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kN");

        let mut t1 = 450.0 * UnitIlluminance::Lux(Metric::None);
        let t2 = 1.0 * UnitIlluminance::Lux(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 klx");

        let mut t1 = 512.0 * UnitInformation::Byte(Metric::None);
        let t2 = 1.0 * UnitInformation::Byte(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.5 kb");

        let mut t1 = 450.0 * UnitLuminousFlux::Lumen(Metric::None);
        let t2 = 1.0 * UnitLuminousFlux::Lumen(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 klm");

        let mut t1 = 450.0 * UnitLuminousIntensity::Candela(Metric::None);
        let t2 = 1.0 * UnitLuminousIntensity::Candela(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kcd");

        let mut t1 = 450.0 * UnitMagneticFlux::Weber(Metric::None);
        let t2 = 1.0 * UnitMagneticFlux::Weber(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kWb");

        let mut t1 = 450.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t2 = 1.0 * UnitMagneticFluxDensity::Tesla(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kT");

        let mut t1 = 450.0 * UnitPower::Watt(Metric::None);
        let t2 = 1.0 * UnitPower::Watt(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kW");

        let mut t1 = 450.0 * UnitPressure::Bar(Metric::None);
        let t2 = 1.0 * UnitPressure::Bar(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kbar");

        let mut t1 = 450.0 * UnitRadioactivity::Becquerel(Metric::None);
        let t2 = 1.0 * UnitRadioactivity::Becquerel(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kBq");

        let mut t1 = 450.0 * UnitRadioactivityExposure::Sievert(Metric::None);
        let t2 = 1.0 * UnitRadioactivityExposure::Sievert(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kSv");

        let mut t1 = 450.0 * UnitSolidAngle::Steradian(Metric::None);
        let t2 = 1.0 * UnitSolidAngle::Steradian(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 ksr");

        let mut t1 = 450.0 * UnitSound::Bel(Metric::None);
        let t2 = 1.0 * UnitSound::Bel(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kB");

        let mut t1 = 450.0 * UnitSubstance::Mole(Metric::None);
        let t2 = 1.0 * UnitSubstance::Mole(Metric::Kilo);

        t1._convert(&t2).unwrap();
        assert_eq!(t1.to_string(), "0.45 kmol");
    }
}
