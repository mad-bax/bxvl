use crate::values::Value;
use crate::constants::*;
use crate::errors::V3Error;
use crate::units::{
    Metric,
    UnitAbsorbedDose,
    UnitAngle,
    UnitCapacitance,
    UnitCatalyticActivity,
    UnitElectricCharge,
    UnitElectricConductance,
    UnitElectricCurrent,
    UnitElectricPotential,
    UnitEnergy,
    UnitForce,
    UnitFrequency,
    UnitIlluminance,
    UnitInductance,
    UnitInformation,
    UnitLength,
    UnitLuminousFlux,
    UnitLuminousIntensity,
    UnitMass,
    UnitPower,
    UnitPressure,
    UnitRadioactivity,
    UnitRadioactivityExposure,
    UnitResistance,
    UnitSound,
    UnitSubstance,
    UnitTemperature,
    UnitTime,
    UnitVolume,
    UnitMagneticFlux,
    UnitMagneticFluxDensity,
    UnitSolidAngle
};

impl Default for Value {
    /// The default constructor for a `Value`
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// let mut m:Value = Value::default();
    /// m.val = 1.3;
    /// ```
    fn default() -> Value {
        Value {
            val:0.0,
            unit_map:0,
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
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None,
            v_solid_angle : None
        }
    }
}

impl Value {

    /// The main constructor for a `Value`
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// let m:Value = match Value::new(4.5, "m") {
    ///     Ok(v) => v,
    ///     Err(e) => panic!("{}", e)
    /// };
    /// ```
    pub fn new(val:f64, units:&str) -> Result<Value, V3Error> {
        let mut ret:Value = Value {
            val,
            unit_map:0,
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
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None,
            v_solid_angle : None
        };
        ret._create_unit(units)?;

        Ok(ret)
    }

    /// Creates a `Value` specifically in radians
    fn _radians(val:f64) -> Value {
        let mut ret:Value = Value {
            val,
            unit_map:ANGLE_MAP,
            exp:[0;31],
            v_ab_dose: None,
            v_angle: Some(UnitAngle::Radian(Metric::None)),
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
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None,
            v_solid_angle : None
        };
        ret.exp[ANGLE_INDEX] = 1;
        ret
    }

    /// Convert a `Value` to another of the same base unit types.
    /// 
    /// `convert` uses a `&str` as an argument and parses it into the relevant units.
    /// The parsed `&str` must be over the same unit types as the `Value` to be converted. 
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
    pub fn convert(&mut self, other:&str) -> Result<(), V3Error> {
        let temp:Value = Value::new(0.0, other)?; // We make a temporary Value to make conversion easier
        self._convert(&temp)
    }

    /// Actual convert functionality with a given `Value` argument
    pub(in crate::values) fn _convert(&mut self, other:&Value) -> Result<(), V3Error> {

        if self.unit_map == VOLUME_MAP && other.unit_map == LENGTH_MAP {
            if self.exp[VOLUME_INDEX] == 1 && other.exp[LENGTH_INDEX] == 3 {
                self.val *= self.v_volume.unwrap().convert_meter(&other.v_length.unwrap());
                self.exp[LENGTH_INDEX] = 3;
                self.exp[VOLUME_INDEX] = 0;
                self.unit_map = LENGTH_MAP;
                self.v_volume = None;
                self.v_length = other.v_length;
                return Ok(());
            } 
            return Err(V3Error::ValueConversionError("[_convert] Error converting volume to cubic"));
        } else if self.unit_map == LENGTH_MAP && other.unit_map == VOLUME_MAP {
            if self.exp[LENGTH_INDEX] == 3 && other.exp[VOLUME_INDEX] == 1 {
                self.val *= f64::powf(self.v_length.unwrap().convert(&UnitLength::Meter(Metric::None)), 3.0);
                self.val *= self.v_length.unwrap().convert_liter(&other.v_volume.unwrap());
                self.exp[LENGTH_INDEX] = 0;
                self.exp[VOLUME_INDEX] = 1;
                self.unit_map = VOLUME_MAP;
                self.v_volume = other.v_volume;
                self.v_length = None;
                return Ok(());
            }
            return Err(V3Error::ValueConversionError("[_convert] Error converting cubic to volume"));
        } else if self.unit_map == 0 && other.unit_map == ANGLE_MAP {
            if other.v_angle.unwrap() == UnitAngle::Radian(Metric::None) {
                self.exp[ANGLE_INDEX] = 1;
                self.unit_map = ANGLE_MAP;
                self.v_angle = Some(UnitAngle::Radian(Metric::None));
                return Ok(());
            }
            return Err(V3Error::ValueConversionError("[_convert] Error converting unitless to radians"))
        } else if self.unit_map == FREQUENCY_MAP && other.unit_map == TIME_MAP &&
                  self.exp[FREQUENCY_INDEX] == 1 && other.exp[TIME_INDEX] == -1 {
            self.exp[FREQUENCY_INDEX] = 0;
            self.exp[TIME_INDEX] = -1;
            self.unit_map = TIME_MAP;
            self.val *= self.v_frequency.unwrap().convert_time(&other.v_time.unwrap());
            self.v_frequency = None;
            self.v_time = other.v_time;
            return Ok(());
        } else if self.unit_map == TIME_MAP && other.unit_map == FREQUENCY_MAP &&
                  self.exp[TIME_INDEX] == -1 && other.exp[FREQUENCY_INDEX] == 1 {
            self.exp[FREQUENCY_INDEX] = 1;
            self.exp[TIME_INDEX] = 0;
            self.unit_map = FREQUENCY_MAP;
            self.val *= self.v_time.unwrap().convert_freq(&other.v_frequency.unwrap());
            self.v_frequency = other.v_frequency;
            self.v_time = None;
            return Ok(());
        }

        if self.unit_map != other.unit_map {
            return Err(V3Error::ValueConversionError("[_convert] Nonequivalent unit types"));
        }

        // check against temperature 
        if self.unit_map & TEMPERATURE_MAP < self.unit_map {
            if self.v_temperature != other.v_temperature {
                return Err(V3Error::ValueConversionError("[_convert] Temperature unit mismatch"));
            }
        } else if self.unit_map == TEMPERATURE_MAP {
            self.val = self.v_temperature.unwrap().convert(&other.v_temperature.unwrap(), self.val);
            self.v_temperature = other.v_temperature;
            return Ok(());
        }

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                return Err(V3Error::ValueConversionError("[_convert] Mismatched value exponents"));
            }
            let region:usize = 1<<i;
            if region & self.unit_map != 0 {
                let tmp:f64;
                self.val *= f64::powi(match region {
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
                        tmp = self.v_electric_current.unwrap().convert(&other.v_electric_current.unwrap());
                        self.v_electric_current = other.v_electric_current;
                        tmp
                    }
                    ELECTRIC_CHARGE_MAP => {
                        tmp = self.v_electric_charge.unwrap().convert(&other.v_electric_charge.unwrap());
                        self.v_electric_charge = other.v_electric_charge;
                        tmp
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        tmp = self.v_electric_potential.unwrap().convert(&other.v_electric_potential.unwrap());
                        self.v_electric_potential = other.v_electric_potential;
                        tmp
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        tmp = self.v_electric_conductance.unwrap().convert(&other.v_electric_conductance.unwrap());
                        self.v_electric_conductance = other.v_electric_conductance;
                        tmp
                    }
                    CAPACITANCE_MAP => {
                        tmp = self.v_capacitance.unwrap().convert(&other.v_capacitance.unwrap());
                        self.v_capacitance = other.v_capacitance;
                        tmp
                    }
                    RESISTANCE_MAP => {
                        tmp = self.v_resistance.unwrap().convert(&other.v_resistance.unwrap());
                        self.v_resistance = other.v_resistance;
                        tmp
                    }
                    INDUCTANCE_MAP => {
                        tmp = self.v_inductance.unwrap().convert(&other.v_inductance.unwrap());
                        self.v_inductance = other.v_inductance;
                        tmp
                    }
                    MAGNETIC_FLUX_MAP => {
                        tmp = self.v_magnetic_flux.unwrap().convert(&other.v_magnetic_flux.unwrap());
                        self.v_magnetic_flux = other.v_magnetic_flux;
                        tmp
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        tmp = self.v_magnetic_flux_density.unwrap().convert(&other.v_magnetic_flux_density.unwrap());
                        self.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        tmp
                    }
                    TEMPERATURE_MAP => {
                        1.0 // This should not convert at the moment
                    }
                    SUBSTANCE_MAP => {
                        tmp = self.v_substance.unwrap().convert(&other.v_substance.unwrap());
                        self.v_substance = other.v_substance;
                        tmp
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        tmp = self.v_luminous_flux_intensity.unwrap().convert(&other.v_luminous_flux_intensity.unwrap());
                        self.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        tmp
                    }
                    LUMINOUS_FLUX_MAP => {
                        tmp = self.v_luminous_flux.unwrap().convert(&other.v_luminous_flux.unwrap());
                        self.v_luminous_flux = other.v_luminous_flux;
                        tmp
                    }
                    ILLUMINANCE_MAP => {
                        tmp = self.v_illuminance.unwrap().convert(&other.v_illuminance.unwrap());
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
                        tmp = self.v_frequency.unwrap().convert(&other.v_frequency.unwrap());
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
                        tmp = self.v_radioactivity.unwrap().convert(&other.v_radioactivity.unwrap());
                        self.v_radioactivity = other.v_radioactivity;
                        tmp
                    }
                    ABSORBED_DOSE_MAP => {
                        tmp = self.v_ab_dose.unwrap().convert(&other.v_ab_dose.unwrap());
                        self.v_ab_dose = other.v_ab_dose;
                        tmp
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        tmp = self.v_radioactivity_exposure.unwrap().convert(&other.v_radioactivity_exposure.unwrap());
                        self.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        tmp
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        tmp = self.v_catalytic.unwrap().convert(&other.v_catalytic.unwrap());
                        self.v_catalytic = other.v_catalytic;
                        tmp
                    }
                    SOUND_MAP => {
                        tmp = self.v_sound.unwrap().convert(&other.v_sound.unwrap());
                        self.v_sound = other.v_sound;
                        tmp
                    }
                    INFORMATION_MAP => {
                        tmp = self.v_information.unwrap().convert(&other.v_information.unwrap());
                        self.v_information = other.v_information;
                        tmp
                    }
                    SOLID_ANGLE_MAP => {
                        tmp = self.v_solid_angle.unwrap().convert(&other.v_solid_angle.unwrap());
                        self.v_solid_angle = other.v_solid_angle;
                        tmp
                    }
                    _ => {
                        return Err(V3Error::UnknownError("[_convert] Value conversion"));
                    }
                }, self.exp[i]);
            }
        }
        Ok(())
    }

    /// Inverses the `Value`
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::UnitLength;
    /// let mut v:Value = 4.0 | UnitLength::Inch;
    /// v.inv()
    /// ```
    /// `v` will now be equal to `0.25 1/in`
    pub fn inv(&mut self) {
        self.val = 1.0/self.val;
        for i in 0..self.exp.len() {
            self.exp[i] *= -1;
        }
    }

    /// Converts an angle to radians
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::UnitAngle;
    /// let mut a:Value = 45.0 | UnitAngle::Degree;
    /// a.to_radians();
    /// ```
    pub fn to_radians(&mut self) {
        if self.unit_map != ANGLE_MAP && self.exp[ANGLE_INDEX] != 1 {
            panic!("[to_radians] Cannot convert non angle to radians");
        }
        self.val *= self.v_angle.unwrap().convert(&UnitAngle::Radian(Metric::None));
    }

    /// Converts an angle to degrees
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::{UnitAngle, Metric};
    /// let mut a:Value = (2.0/std::f64::consts::PI) | UnitAngle::Radian(Metric::None);
    /// a.to_degrees();
    /// ```
    pub fn to_degrees(&mut self) {
        if self.unit_map != ANGLE_MAP && self.exp[ANGLE_INDEX] != 1 {
            panic!("[to_degrees] Cannot convert non angle to degrees");
        }
        self.val *= self.v_angle.unwrap().convert(&UnitAngle::Degree);
    }

    /// Returns if the `Value` numeric is NAN
    pub fn is_nan(&self) -> bool {
        self.val.is_nan()
    }

    /// Returns if the `Value` numeric is finite
    pub fn is_finite(&self) -> bool {
        self.val.is_finite()
    }

    /// Returns if the `Value` numeric is infinite
    pub fn is_infinite(&self) -> bool {
        self.val.is_infinite()
    }

    /// Returns if the `Value` numeric is normal
    pub fn is_normal(&self) -> bool {
        self.val.is_normal()
    }

    /// Returns if the `Value` numeric is subnormal
    pub fn is_subnormal(&self)  -> bool {
        self.val.is_subnormal()
    }

    /// Returns if the `Value` numeric is sign positive
    pub fn is_sign_positive(&self) -> bool {
        self.val.is_sign_positive()
    }

    /// Returns if the `Value` numeric is sign negative
    pub fn is_sign_negative(&self) -> bool {
        self.val.is_sign_negative()
    }

    /// Takes the square root of the Value
    /// 
    /// Note: That if the unit exponents are not evenly divisible by 2, the function will panic.
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::UnitLength;
    /// let mut v:Value = 16.0 | UnitLength::Foot | UnitLength::Foot;
    /// let x:Value = v.sqrt();
    /// ```
    /// `x` will be equal to `4.0 ft`
    pub fn sqrt(&self) -> Value {
        let mut n:Value = *self;
        for i in 0..31_usize {
            if n.exp[i]%2 != 0 {
                panic!("[sqrt] Cannot square root Value: {}", self);
            }
            n.exp[i] /= 2;
        }
        n.val = n.val.sqrt();
        n
    }

    /// Returns a new value to some arbitrary power.
    /// 
    /// Note: This is faster than simply multiplying `v*v` to achieve a value to a power.
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::UnitLength;
    /// let v:Value = 4.0 | UnitLength::Foot;
    /// let x:Value = v.powv(2);
    /// assert!(String::from("16 ft^2") == format!("{}", x));
    /// ```
    pub fn powv(&self, p:i32) -> Value {
        let mut n:Value = *self;
        for i in 0..31_usize {
            n.exp[i] *= p;
        }
        n.val = n.val.powf(p as f64);
        n
    }

    /// Takes the cube root of the Value
    /// 
    /// Note: That if the unit exponents are not evenly divisible by 3, the function will panic.
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::UnitLength;
    /// let mut v:Value = 9.0 | UnitLength::Foot | UnitLength::Foot | UnitLength::Foot;
    /// let x:Value = v.cbrt();
    /// ```
    /// `x` will be equal to `3.0 ft`
    pub fn cbrt(&self) -> Value {
        let mut n:Value = *self;
        for i in 0..31_usize {
            if n.exp[i]%3 != 0 {
                panic!("[cbrt] Cannot cube root Value: {}", self);
            }
            n.exp[i] /= 3;
        }
        n.val = n.val.cbrt();
        n
    }

    /// Returns the sine of a `Value` in radians
    pub fn sin(&self) -> Value {
        Value::_radians(self.val.sin())
    }

    /// Returns the cosine of a `Value` in radians
    pub fn cos(&self) -> Value {
        Value::_radians(self.val.cos())
    }

    /// Returns the tangent of a `Value` in radians
    pub fn tan(&self) -> Value {
        Value::_radians(self.val.tan())
    }

    /// Returns the arcsine of a `Value` in radians
    pub fn asin(&self) -> Value {
        Value::_radians(self.val.asin())
    }

    /// Returns the arccosine of a `Value` in radians
    pub fn acos(&self) -> Value {
        Value::_radians(self.val.acos())
    }

    /// Returns the arctangent of a `Value` in radians
    pub fn atan(&self) -> Value {
        Value::_radians(self.val.atan())
    }

    /// Returns the full unit circle arctangent of a `Value` in radians
    /// 
    /// atan2 will panic if the the given `Value` is not an angle.
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::{UnitTime, UnitAngle, Metric};
    /// let a:Value = 10.0 | UnitTime::Second(Metric::None);
    /// let b:Value = 0.3 | UnitAngle::Radian(Metric::None);
    /// let x:Value = a.atan2(&b);
    /// ```
    /// `x` will be approximately equal to `1.5408 radians`
    pub fn atan2(&self, other:&Value) -> Value {
        if other.unit_map != ANGLE_MAP && other.exp[ANGLE_INDEX] != 1 {
            panic!("[atan2] atan2 requires an Value angle");
        }
        let new_v:f64 = other.val * other.v_angle.unwrap().convert(&UnitAngle::Radian(Metric::None));
        Value::_radians(self.val.atan2(new_v))
    }

    /// Combines unit types in a `Value` if applicable.
    /// 
    /// When multiplying different `Value`s together, there are specific combinations that 
    /// can create more complex unit types which are supported by the `Value` type. 
    /// 
    /// e.g. F = m*a, where Newtons = kilogram * (meters/second^2).
    /// 
    /// This unit complexity is not handled implicitly by the `mul` and `div` methods, and must be called by users. 
    /// 
    /// Note: This function will not return an error if a combination cannot be found. This function is intended to be a 'pass through' function.
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::{UnitLength, UnitMass, UnitTime, Metric};
    /// let mass:Value = 4.5 | UnitMass::Gram(Metric::Kilo);
    /// let acc:Value = 9.81 ^ UnitTime::Second(Metric::None) ^ UnitTime::Second(Metric::None) | UnitLength::Meter(Metric::None);
    /// let mut f:Value = match (mass*acc).complex() {
    ///     Ok(t) => t,
    ///     Err(e) => panic!("{}", e)
    /// };
    /// ```
    /// `f` will be equal to `44.145 N`
    pub fn complex(&self) -> Result<Value, V3Error> {
        let mut ret:Value = *self;
        if ret.is_force() && ret.unit_map != FORCE_MAP {
            (ret >>= UnitMass::Gram(Metric::Kilo));
            (ret >>= UnitLength::Meter(Metric::None));
            (ret >>= UnitTime::Second(Metric::None));
            ret.exp[MASS_INDEX] = 0;
            ret.exp[LENGTH_INDEX] = 0;
            ret.exp[TIME_INDEX] = 0;
            ret.unit_map = FORCE_MAP;
            ret.exp[FORCE_INDEX] = 1;
            ret.v_force = Some(UnitForce::Newton(Metric::None));
            ret.v_length = None;
            ret.v_mass = None;
            ret.v_time = None;
        } else if ret.is_pressure() && ret.unit_map != PRESSURE_MAP {
            (ret >>= UnitForce::Newton(Metric::None));
            (ret >>= UnitLength::Meter(Metric::None));
            ret.exp[FORCE_INDEX] = 0;
            ret.exp[LENGTH_INDEX] = 0;
            ret.unit_map = PRESSURE_MAP;
            ret.exp[PRESSURE_INDEX] = 1;
            ret.v_pressure = Some(UnitPressure::Pascal(Metric::None));
            ret.v_force = None;
            ret.v_length = None;
        } else if ret.is_energy() && ret.unit_map != ENERGY_MAP {
            if ret.unit_map & LENGTH_MAP == LENGTH_MAP {
                (ret >>= UnitForce::Newton(Metric::None));
                (ret >>= UnitLength::Meter(Metric::None));
                ret.exp[FORCE_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.v_force = None;
                ret.v_length = None;
            } else if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                (ret >>= UnitElectricCharge::Coulomb(Metric::None));
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                ret.exp[ELECTRIC_CHARGE_INDEX] = 0;
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_charge = None;
            } else if ret.unit_map & POWER_MAP == POWER_MAP {
                (ret >>= UnitPower::Watt(Metric::None));
                (ret >>= UnitTime::Second(Metric::None));
                ret.exp[POWER_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_power = None;
                ret.v_time = None;
            } else {
                panic!("[complex] Unit translation: assumption Energy");
            }
            ret.unit_map = ENERGY_MAP;
            ret.exp[ENERGY_INDEX] = 1;
            ret.v_energy = Some(UnitEnergy::Joule(Metric::None));
        } else if ret.is_power() && ret.unit_map != POWER_MAP {
            if ret.unit_map & ENERGY_MAP == ENERGY_MAP {
                (ret >>= UnitEnergy::Joule(Metric::None));
                (ret >>= UnitTime::Second(Metric::None));
                ret.exp[ENERGY_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_energy = None;
                ret.v_time = None;
            } else if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                (ret >>= UnitElectricCurrent::Ampere(Metric::None));
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_current = None;
            } else {
                panic!("[complex] Unit translation: assumption Power");
            }
            ret.unit_map = POWER_MAP;
            ret.exp[POWER_INDEX] = 1;
            ret.v_power = Some(UnitPower::Watt(Metric::None));
        } else if ret.is_electric_charge() && self.unit_map != ELECTRIC_CHARGE_MAP {
            if ret.unit_map & TIME_MAP == TIME_MAP {
                (ret >>= UnitTime::Second(Metric::None));
                (ret >>= UnitElectricCurrent::Ampere(Metric::None));
                ret.exp[TIME_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_time = None;
                ret.v_electric_current = None;
            } else if ret.unit_map & CAPACITANCE_MAP == CAPACITANCE_MAP {
                (ret >>= UnitCapacitance::Farad(Metric::None));
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                ret.exp[CAPACITANCE_INDEX] = 0;
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.v_capacitance = None;
                ret.v_electric_potential = None;
            } else {
                panic!("[complex] Unit translation: assumption Electric Charge");
            }
            ret.unit_map = ELECTRIC_CHARGE_MAP;
            ret.exp[ELECTRIC_CHARGE_INDEX] = 1;
            ret.v_electric_charge = Some(UnitElectricCharge::Coulomb(Metric::None));
        } else if ret.is_electric_potential() && self.unit_map != ELECTRIC_POTENTIAL_MAP {
            if ret.unit_map & POWER_MAP == POWER_MAP {
                (ret >>= UnitPower::Watt(Metric::None));
                (ret >>= UnitElectricCurrent::Ampere(Metric::None));
                ret.exp[POWER_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_power = None;
                ret.v_electric_current = None;
            } else if ret.unit_map & ENERGY_MAP == ENERGY_MAP {
                (ret >>= UnitEnergy::Joule(Metric::None));
                (ret >>= UnitElectricCharge::Coulomb(Metric::None));
                ret.exp[ENERGY_INDEX] = 0;
                ret.exp[ELECTRIC_CHARGE_INDEX] = 0;
                ret.v_energy = None;
                ret.v_electric_charge = None;
            } else {
                panic!("[complex] Unit translation: assumption Electric Potential");
            }
            ret.unit_map = ELECTRIC_POTENTIAL_MAP;
            ret.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
            ret.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
        } else if ret.is_capacitance() && self.unit_map != CAPACITANCE_MAP {
            if ret.unit_map & ELECTRIC_CHARGE_MAP == ELECTRIC_CHARGE_MAP {
                (ret >>= UnitElectricCharge::Coulomb(Metric::None));
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                ret.exp[ELECTRIC_CHARGE_INDEX] = 0;
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_charge = None;
            } else if ret.unit_map & RESISTANCE_MAP == RESISTANCE_MAP {
                (ret >>= UnitTime::Second(Metric::None));
                (ret >>= UnitResistance::Ohm(Metric::None));
                ret.exp[TIME_INDEX] = 0;
                ret.exp[RESISTANCE_INDEX] = 0;
                ret.v_time = None;
                ret.v_resistance = None;
            } else {
                panic!("[complex] Unit translation: assumption Capacitance")
            }

            ret.unit_map = CAPACITANCE_MAP;
            ret.exp[CAPACITANCE_INDEX] = 1;
            ret.v_capacitance = Some(UnitCapacitance::Farad(Metric::None));
        } else if ret.is_resistance() && self.unit_map != RESISTANCE_MAP {
            if ret.unit_map & ELECTRIC_CONDUCTANCE_MAP == ELECTRIC_CONDUCTANCE_MAP {
                (ret >>= UnitElectricConductance::Siemens(Metric::None));
                ret.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
                ret.v_electric_conductance = None;
            } else if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                (ret >>= UnitElectricCurrent::Ampere(Metric::None));
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_current = None;
            } else {
                panic!("[complex] Unit translation: assumption Resistance");
            }

            ret.unit_map = RESISTANCE_MAP;
            ret.exp[RESISTANCE_INDEX] = 1;
            ret.v_resistance = Some(UnitResistance::Ohm(Metric::None));
        } else if ret.is_conductance() && self.unit_map != ELECTRIC_CONDUCTANCE_MAP {
            if ret.unit_map & RESISTANCE_MAP == RESISTANCE_MAP {
                (ret >>= UnitResistance::Ohm(Metric::None));
                ret.exp[RESISTANCE_INDEX] = 0;
                ret.v_resistance = None;
            } else if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                (ret >>= UnitElectricCurrent::Ampere(Metric::None));
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_current = None;
            } else {
                panic!("[complex] Unit translation: assumption Electric Conductance");
            }

            ret.unit_map = ELECTRIC_CONDUCTANCE_MAP;
            ret.exp[ELECTRIC_CONDUCTANCE_INDEX] = 1;
            ret.v_electric_conductance = Some(UnitElectricConductance::Siemens(Metric::None));
        } else if ret.is_magnetic_flux() && self.unit_map != MAGNETIC_FLUX_MAP {
            if ret.unit_map & ENERGY_MAP == ENERGY_MAP {
                (ret >>= UnitEnergy::Joule(Metric::None));
                (ret >>= UnitElectricCurrent::Ampere(Metric::None));
                ret.exp[ENERGY_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_energy = None;
                ret.v_electric_current = None;
            } else if ret.unit_map & MAGNETIC_FLUX_DENSITY_MAP == MAGNETIC_FLUX_DENSITY_MAP {
                (ret >>= UnitMagneticFluxDensity::Tesla(Metric::None));
                (ret >>= UnitLength::Meter(Metric::None));
                ret.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.v_length = None;
                ret.v_magnetic_flux_density = None;
            } else if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                (ret >>= UnitTime::Second(Metric::None));
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_time = None;
            } else {
                panic!("[complex] Unit translation: assumption Magnetic Flux");
            }

            ret.unit_map = MAGNETIC_FLUX_MAP;
            ret.exp[MAGNETIC_FLUX_INDEX] = 1;
            ret.v_magnetic_flux = Some(UnitMagneticFlux::Weber(Metric::None));
        } else if ret.is_magnetic_flux_density() && self.unit_map != MAGNETIC_FLUX_DENSITY_MAP {
            if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                ret >>= UnitElectricPotential::Volt(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret >>= UnitLength::Meter(Metric::None);
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.v_length = None;
                ret.v_time = None;
                ret.v_length = None;
            } else if ret.unit_map & MAGNETIC_FLUX_MAP == MAGNETIC_FLUX_MAP {
                ret >>= UnitMagneticFlux::Weber(Metric::None);
                ret >>= UnitLength::Meter(Metric::None);
                ret.exp[LENGTH_INDEX] = 0;
                ret.exp[MAGNETIC_FLUX_INDEX] = 0;
                ret.v_magnetic_flux = None;
                ret.v_length = None;
            } else if ret.unit_map & FORCE_MAP == FORCE_MAP {
                ret >>= UnitForce::Newton(Metric::None);
                ret >>= UnitElectricCurrent::Ampere(Metric::None);
                ret >>= UnitLength::Meter(Metric::None);
                ret.exp[FORCE_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.v_length = None;
                ret.v_electric_current = None;
                ret.v_force = None;
            } else {
                panic!("[complex] Unit translation: assumption Magnetic Flux Density");
            }

            ret.unit_map = MAGNETIC_FLUX_DENSITY_MAP;
            ret.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 1;
            ret.v_magnetic_flux_density = Some(UnitMagneticFluxDensity::Tesla(Metric::None));
        } else if ret.is_inductance() && self.unit_map != INDUCTANCE_MAP {
            if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                ret >>= UnitElectricPotential::Volt(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret >>= UnitElectricCurrent::Ampere(Metric::None);
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_time = None;
                ret.v_electric_current = None;
            } else if ret.unit_map & RESISTANCE_MAP == RESISTANCE_MAP {
                ret >>= UnitResistance::Ohm(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret.exp[RESISTANCE_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_resistance = None;
                ret.v_time = None;
            } else if ret.unit_map & MAGNETIC_FLUX_MAP == MAGNETIC_FLUX_MAP {
                ret >>= UnitMagneticFlux::Weber(Metric::None);
                ret >>= UnitElectricCurrent::Ampere(Metric::None);
                ret.exp[MAGNETIC_FLUX_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_magnetic_flux = None;
                ret.v_electric_current = None;
            } else {
                panic!("[complex] Unit translation: assumption Electric Inductance");
            }

            ret.unit_map = INDUCTANCE_MAP;
            ret.exp[INDUCTANCE_INDEX] = 1;
            ret.v_inductance = Some(UnitInductance::Henry(Metric::None));
        } else if ret.is_luminous_flux() && self.unit_map != LUMINOUS_FLUX_MAP {
            ret >>= UnitLuminousIntensity::Candela(Metric::None);
            ret >>= UnitSolidAngle::Steradian(Metric::None);
            ret.exp[LUMINOUS_INTENSITY_INDEX] = 0;
            ret.exp[SOLID_ANGLE_INDEX] = 0;
            ret.v_solid_angle = None;
            ret.v_luminous_flux_intensity = None;
            ret.unit_map = LUMINOUS_FLUX_MAP;
            ret.exp[LUMINOUS_FLUX_INDEX] = 1;
            ret.v_luminous_flux = Some(UnitLuminousFlux::Lumen(Metric::None));
        } else if ret.is_illuminance() && self.unit_map != ILLUMINANCE_MAP {
            ret >>= UnitLuminousFlux::Lumen(Metric::None);
            ret >>= UnitLength::Meter(Metric::None);
            ret.exp[LUMINOUS_FLUX_INDEX] = 0;
            ret.exp[LENGTH_INDEX] = 0;
            ret.v_length = None;
            ret.v_luminous_flux = None;
            ret.unit_map = ILLUMINANCE_MAP;
            ret.exp[ILLUMINANCE_INDEX] = 1;
            ret.v_illuminance = Some(UnitIlluminance::Lux(Metric::None));
        } else if ret.is_catalytic_activity() && self.unit_map != CATALYTIC_ACTIVITY_MAP {
            ret >>= UnitSubstance::Mole(Metric::None);
            ret >>= UnitTime::Second(Metric::None);
            ret.exp[SUBSTANCE_INDEX] = 0;
            ret.exp[TIME_INDEX] = 0;
            ret.v_time = None;
            ret.v_substance = None;
            ret.unit_map = CATALYTIC_ACTIVITY_MAP;
            ret.exp[CATALYTIC_ACTIVITY_INDEX] = 1;
            ret.v_catalytic = Some(UnitCatalyticActivity::Katal(Metric::None));
        }
        Ok(ret)
    }

    /// Reduces a `Value`'s unit complexity. 
    /// 
    /// When a `Value` has a specific type that is composed from base units such as `Newtons`; 
    /// it can be reduced to those base units.
    /// 
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// let mut f:Value = match Value::new(3.0, "N") {
    ///     Ok(t) => t,
    ///     Err(e) => panic!("{}", e)
    /// };
    /// 
    /// match f.reduce("kg*m/s^2") {
    ///     Ok(_) => {}
    ///     Err(e) => panic!("{}", e)
    /// }
    /// ```
    /// `f` will now be equal to `3.0 kg*m/s^2`
    pub fn reduce(&mut self, other:&str) -> Result<(), V3Error> {
        if !self.reducible() {
            return Err(V3Error::UnitReductionError(format!("[reduce] Value {} is not reducible", self)));
        }
        let temp:Value = match Value::new(1.0, other) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        if self._reduce(&temp) {
            return Ok(());
        }
        Err(V3Error::UnitReductionError(format!("[reduce] Value {} cannot be reduced to {}", self, other)))
    }

    /// Returns if a `Value`'s unit type is reducible.
    pub fn reducible(&self) -> bool {

        matches!(self.unit_map,
            FORCE_MAP |
            PRESSURE_MAP |
            ENERGY_MAP |
            FREQUENCY_MAP |
            POWER_MAP |
            ELECTRIC_CHARGE_MAP |
            ELECTRIC_POTENTIAL_MAP |
            RESISTANCE_MAP |
            ELECTRIC_CONDUCTANCE_MAP |
            MAGNETIC_FLUX_MAP |
            MAGNETIC_FLUX_DENSITY_MAP |
            INDUCTANCE_MAP |
            ILLUMINANCE_MAP |
            CAPACITANCE_MAP
        )
    }

    /// Actual reduce function that operates on a `Value` type
    fn _reduce(&mut self, other:&Value) -> bool {
        if self.unit_map == other.unit_map {
            return false;
        }

        if self.unit_map == FORCE_MAP && other.is_force() {
            *self >>= UnitForce::Newton(Metric::None);
            self.v_mass = Some(UnitMass::Gram(Metric::Kilo));
            self.v_time = Some(UnitTime::Second(Metric::None));
            self.v_length = Some(UnitLength::Meter(Metric::None));
            self.v_force = None;

            self.exp[LENGTH_INDEX] = 1;
            self.exp[MASS_INDEX] = 1;
            self.exp[TIME_INDEX] = -2;
            self.exp[FORCE_INDEX] = 0;
            self.unit_map = LENGTH_MAP | TIME_MAP | MASS_MAP;

            *self >>= *other;
            return true;
        } else if self.unit_map == PRESSURE_MAP && other.is_pressure() {
            *self >>= UnitPressure::Pascal(Metric::None);
            self.v_force = Some(UnitForce::Newton(Metric::None));
            self.v_length = Some(UnitLength::Meter(Metric::None));
            self.v_pressure = None;

            self.exp[LENGTH_INDEX] = -2;
            self.exp[FORCE_INDEX] = 1;
            self.exp[PRESSURE_INDEX] = 0;
            self.unit_map = FORCE_MAP | LENGTH_MAP;
            *self >>= *other;
            return true;
        } else if self.unit_map == ENERGY_MAP && other.is_energy() {
            if other.unit_map & FORCE_MAP > 0 {
                *self >>= UnitEnergy::Joule(Metric::None);
                self.v_force = Some(UnitForce::Newton(Metric::None));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_energy = None;
                self.exp[FORCE_INDEX] = 1;
                self.exp[LENGTH_INDEX] = 1;
                self.exp[ENERGY_INDEX] = 0;
                self.unit_map = FORCE_MAP | LENGTH_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ELECTRIC_POTENTIAL_MAP > 0 {
                *self >>= UnitEnergy::Joule(Metric::None);
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_electric_charge = Some(UnitElectricCharge::Coulomb(Metric::None));
                self.v_energy = None;
                self.exp[ELECTRIC_CHARGE_INDEX] = -1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[ENERGY_INDEX] = 1;
                self.unit_map = ELECTRIC_CHARGE_MAP | ELECTRIC_POTENTIAL_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & POWER_MAP > 0 {
                *self >>= UnitEnergy::Joule(Metric::None);
                self.v_power = Some(UnitPower::Watt(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_energy = None;
                self.exp[POWER_INDEX] = 1;
                self.exp[TIME_INDEX] = -1;
                self.exp[ENERGY_INDEX] = 0;
                self.unit_map = POWER_MAP | TIME_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & MASS_MAP > 0 {
                *self >>= UnitEnergy::Joule(Metric::None);
                self.v_mass = Some(UnitMass::Gram(Metric::Kilo));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));   
                self.exp[LENGTH_INDEX] = 2;
                self.exp[MASS_INDEX] = 1;
                self.exp[TIME_INDEX] = -2;
                self.unit_map = LENGTH_MAP | MASS_MAP | TIME_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == FREQUENCY_MAP && other.is_frequency() {
            *self >>= UnitFrequency::Hertz(Metric::None);
            self.v_frequency = None;
            self.v_time = Some(UnitTime::Second(Metric::None));
            self.exp[TIME_INDEX] = -1;
            self.exp[FREQUENCY_INDEX] = 0;
            self.unit_map = TIME_MAP;
            *self >>= *other;
            return true;
        } else if self.unit_map == POWER_MAP && other.is_power() {
            if other.unit_map & ENERGY_MAP > 0 {
                *self >>= UnitPower::Watt(Metric::None);
                self.v_power = None;
                self.v_energy = Some(UnitEnergy::Joule(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.exp[ENERGY_INDEX] = 1;
                self.exp[TIME_INDEX] = -1;
                self.exp[POWER_INDEX] = 0;
                self.unit_map = ENERGY_MAP | TIME_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ELECTRIC_POTENTIAL_MAP > 0 {
                *self >>= UnitPower::Watt(Metric::None);
                self.v_power = None;
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.exp[POWER_INDEX] = 0;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[ELECTRIC_CURRENT_INDEX] = 1;
                self.unit_map = ELECTRIC_POTENTIAL_MAP | ELECTRIC_CURRENT_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & MASS_MAP > 0 {
                *self >>= UnitPower::Watt(Metric::None);
                self.v_power = None;
                self.v_mass = Some(UnitMass::Gram(Metric::Kilo));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.exp[MASS_INDEX] = 1;
                self.exp[LENGTH_INDEX] = 2;
                self.exp[TIME_INDEX] = -3;
                self.unit_map = MASS_MAP | LENGTH_MAP | TIME_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == ELECTRIC_CHARGE_MAP && other.is_electric_charge() {
            if other.unit_map & ELECTRIC_CURRENT_MAP > 0 {
                *self >>= UnitElectricCharge::Coulomb(Metric::None);
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_electric_charge = None;
                self.exp[ELECTRIC_CHARGE_INDEX] = 0;
                self.exp[ELECTRIC_CURRENT_INDEX] = 1;
                self.exp[TIME_INDEX] = 1;
                self.unit_map = ELECTRIC_CURRENT_MAP | TIME_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map  & ELECTRIC_CONDUCTANCE_MAP > 0 {
                *self >>= UnitElectricCharge::Coulomb(Metric::None);
                self.v_capacitance = Some(UnitCapacitance::Farad(Metric::None));
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_electric_charge = None;
                self.exp[CAPACITANCE_INDEX] = 1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[ELECTRIC_CHARGE_INDEX] = 0;
                self.unit_map = CAPACITANCE_MAP | ELECTRIC_POTENTIAL_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == ELECTRIC_POTENTIAL_MAP && other.is_electric_potential() {
            if other.unit_map & POWER_MAP > 0 {
                *self >>= UnitElectricPotential::Volt(Metric::None);
                self.v_power = Some(UnitPower::Watt(Metric::None));
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_electric_potential = None;
                self.exp[POWER_INDEX] = 1;
                self.exp[ELECTRIC_CURRENT_INDEX] = -1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                self.unit_map = POWER_MAP | ELECTRIC_CURRENT_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ENERGY_MAP > 0 {
                *self >>= UnitElectricPotential::Volt(Metric::None);
                self.v_energy = Some(UnitEnergy::Joule(Metric::None));
                self.v_electric_charge = Some(UnitElectricCharge::Coulomb(Metric::None));
                self.v_electric_potential = None;
                self.exp[ENERGY_INDEX] = 1;
                self.exp[ELECTRIC_CHARGE_INDEX] = -1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                self.unit_map = ENERGY_MAP | ELECTRIC_CHARGE_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == CAPACITANCE_MAP && other.is_capacitance() {
            if other.unit_map & ELECTRIC_POTENTIAL_MAP > 0 {
                *self >>= UnitCapacitance::Farad(Metric::None);
                self.v_electric_charge = Some(UnitElectricCharge::Coulomb(Metric::None));
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_capacitance = None;
                self.exp[ELECTRIC_CHARGE_INDEX] = 1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = -1;
                self.exp[CAPACITANCE_INDEX] = 0;
                self.unit_map = ELECTRIC_CHARGE_MAP | ELECTRIC_POTENTIAL_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ENERGY_MAP > 0 {
                *self >>= UnitCapacitance::Farad(Metric::None);
                self.v_electric_charge = Some(UnitElectricCharge::Coulomb(Metric::None));
                self.v_energy = Some(UnitEnergy::Joule(Metric::None));
                self.v_capacitance = None;
                self.exp[CAPACITANCE_INDEX] = 0;
                self.exp[ELECTRIC_CHARGE_INDEX] = 2;
                self.exp[ENERGY_INDEX] = -1;
                self.unit_map = ELECTRIC_CHARGE_MAP | ENERGY_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == RESISTANCE_MAP && other.is_resistance() {
            if other.unit_map & ELECTRIC_CONDUCTANCE_MAP > 0 {
                *self >>= UnitResistance::Ohm(Metric::None);
                self.v_electric_conductance = Some(UnitElectricConductance::Siemens(Metric::None));
                self.v_resistance = None;
                self.exp[ELECTRIC_CONDUCTANCE_INDEX] = -1;
                self.exp[RESISTANCE_INDEX] = 0;
                self.unit_map = ELECTRIC_CONDUCTANCE_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ELECTRIC_CURRENT_MAP > 0 {
                *self >>= UnitResistance::Ohm(Metric::None);
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_resistance = None;
                self.exp[ELECTRIC_CURRENT_INDEX] = -1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[RESISTANCE_INDEX] = 0;
                self.unit_map = ELECTRIC_CURRENT_MAP | ELECTRIC_POTENTIAL_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == ELECTRIC_CONDUCTANCE_MAP && other.is_conductance() {
            if other.unit_map & RESISTANCE_MAP > 0 {
                *self >>= UnitElectricConductance::Siemens(Metric::None);
                self.v_resistance = Some(UnitResistance::Ohm(Metric::None));
                self.v_electric_conductance = None;
                self.exp[RESISTANCE_INDEX] = -1;
                self.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
                self.unit_map = RESISTANCE_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ELECTRIC_CURRENT_MAP > 0 {
                *self >>= UnitElectricConductance::Siemens(Metric::None);
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
                self.exp[ELECTRIC_CURRENT_INDEX] = 1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = -1;
                self.unit_map = ELECTRIC_CURRENT_MAP | ELECTRIC_POTENTIAL_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == MAGNETIC_FLUX_MAP && other.is_magnetic_flux() {
            if other.unit_map & ENERGY_MAP > 0 {
                *self >>= UnitMagneticFlux::Weber(Metric::None);
                self.v_energy = Some(UnitEnergy::Joule(Metric::None));
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_magnetic_flux = None;
                self.exp[ENERGY_INDEX] = 1;
                self.exp[ELECTRIC_CURRENT_INDEX] = -1;
                self.exp[MAGNETIC_FLUX_INDEX] = 0;
                self.unit_map = ENERGY_MAP | ELECTRIC_CURRENT_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & MAGNETIC_FLUX_DENSITY_MAP > 0 {
                *self >>= UnitMagneticFlux::Weber(Metric::None);
                self.v_magnetic_flux_density = Some(UnitMagneticFluxDensity::Tesla(Metric::None));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_magnetic_flux = None;
                self.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 1;
                self.exp[LENGTH_INDEX] = 2;
                self.exp[MAGNETIC_FLUX_INDEX] = 0;
                self.unit_map = MAGNETIC_FLUX_DENSITY_MAP | LENGTH_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ELECTRIC_POTENTIAL_MAP > 0 {
                *self >>= UnitMagneticFlux::Weber(Metric::None);
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_magnetic_flux = None;
                self.exp[TIME_INDEX] = 1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[MAGNETIC_FLUX_INDEX] = 0;
                self.unit_map = TIME_MAP | ELECTRIC_POTENTIAL_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == MAGNETIC_FLUX_DENSITY_MAP && other.is_magnetic_flux_density() {
            if other.unit_map & ELECTRIC_POTENTIAL_MAP > 0 {
                *self >>= UnitMagneticFluxDensity::Tesla(Metric::None);
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_magnetic_flux_density = None;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[TIME_INDEX] = 1;
                self.exp[LENGTH_INDEX] = -2;
                self.unit_map = ELECTRIC_POTENTIAL_MAP | TIME_MAP | LENGTH_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & MAGNETIC_FLUX_MAP > 0 {
                *self >>= UnitMagneticFluxDensity::Tesla(Metric::None);
                self.v_magnetic_flux = Some(UnitMagneticFlux::Weber(Metric::None));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_magnetic_flux_density = None;
                self.exp[LENGTH_INDEX] = -2;
                self.exp[MAGNETIC_FLUX_INDEX] = 1;
                self.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
                self.unit_map = LENGTH_MAP | MAGNETIC_FLUX_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & FORCE_MAP > 0 {
                *self >>= UnitMagneticFluxDensity::Tesla(Metric::None);
                self.v_force = Some(UnitForce::Newton(Metric::None));
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.exp[FORCE_INDEX] = 1;
                self.exp[ELECTRIC_CURRENT_INDEX] = -1;
                self.exp[LENGTH_INDEX] = -1;
                self.unit_map = FORCE_MAP | ELECTRIC_CURRENT_MAP | LENGTH_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == INDUCTANCE_MAP && other.is_inductance() {
            if other.unit_map & ELECTRIC_POTENTIAL_MAP > 0 {
                *self >>= UnitInductance::Henry(Metric::None);
                self.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_inductance = None;
                self.exp[INDUCTANCE_INDEX] = 0;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[TIME_INDEX] = 1;
                self.exp[ELECTRIC_CURRENT_INDEX] = -1;
                self.unit_map = ELECTRIC_POTENTIAL_MAP | TIME_MAP | ELECTRIC_CURRENT_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & RESISTANCE_MAP > 0 {
                *self >>= UnitInductance::Henry(Metric::None);
                self.v_inductance = None;
                self.v_resistance = Some(UnitResistance::Ohm(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.exp[RESISTANCE_INDEX] = 1;
                self.exp[TIME_INDEX] = 1;
                self.exp[INDUCTANCE_INDEX] = 0;
                self.unit_map = RESISTANCE_MAP | TIME_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & MAGNETIC_FLUX_MAP > 0 {
                *self >>= UnitInductance::Henry(Metric::None);
                self.v_magnetic_flux = Some(UnitMagneticFlux::Weber(Metric::None));
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(Metric::None));
                self.v_inductance = None;
                self.exp[MAGNETIC_FLUX_INDEX] = 1;
                self.exp[ELECTRIC_CURRENT_INDEX] = -1;
                self.exp[INDUCTANCE_INDEX] = 0;
                self.unit_map = MAGNETIC_FLUX_MAP | ELECTRIC_CURRENT_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == ILLUMINANCE_MAP && other.is_illuminance() {
            *self >>= UnitIlluminance::Lux(Metric::None);
            self.v_luminous_flux = Some(UnitLuminousFlux::Lumen(Metric::None));
            self.v_length = Some(UnitLength::Meter(Metric::None));
            self.v_illuminance = None;
            self.exp[LUMINOUS_FLUX_INDEX] = 1;
            self.exp[LENGTH_INDEX] = -2;
            self.exp[ILLUMINANCE_INDEX] = 0;
            self.unit_map = LUMINOUS_FLUX_MAP | LENGTH_MAP;
            *self >>= *other;
            return true;
        } else if self.unit_map == CATALYTIC_ACTIVITY_MAP && other.is_catalytic_activity() {
            *self >>= UnitCatalyticActivity::Katal(Metric::None);
            self.v_substance = Some(UnitSubstance::Mole(Metric::None));
            self.v_time = Some(UnitTime::Second(Metric::None));
            self.v_catalytic = None;
            self.exp[SUBSTANCE_INDEX] = 1;
            self.exp[TIME_INDEX] = -1;
            self.exp[CATALYTIC_ACTIVITY_INDEX] = 0;
            self.unit_map = SUBSTANCE_MAP | TIME_MAP;
            *self >>= *other;
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` has no units
    /// 
    /// <none>
    pub fn is_empty(&self) -> bool {
        if self.unit_map == 0 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a length
    /// 
    /// `length`
    pub fn is_length(&self) -> bool {
        if self.unit_map & LENGTH_MAP != self.unit_map || self.exp[LENGTH_INDEX] != 1 {
            return false;
        }
        true
    }

    /// Returns `true` if a `Value` is an area
    /// 
    /// `length^2`
    pub fn is_area(&self) -> bool {
        if self.unit_map & LENGTH_MAP != self.unit_map || self.exp[LENGTH_INDEX] != 2 {
            return false;
        }
        true
    }

    /// Returns `true` if a `Value` is a volume
    /// 
    /// `volume`
    /// 
    /// `length^3`
    pub fn is_volume(&self) -> bool {
        if (self.unit_map == LENGTH_MAP && self.exp[LENGTH_INDEX] == 3) ||
           (self.unit_map == VOLUME_MAP && self.exp[VOLUME_INDEX] == 1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a temperature
    /// 
    /// `temperature`
    pub fn is_temperature(&self) -> bool {
        if self.unit_map == TEMPERATURE_MAP && self.exp[TEMPERATURE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a density
    /// 
    /// `mass / volume`
    pub fn is_density(&self) -> bool {
        if (self.unit_map == MASS_MAP | VOLUME_MAP && self.exp[MASS_INDEX] == 1 && self.exp[VOLUME_INDEX] == -1) ||
           (self.unit_map == MASS_MAP | LENGTH_MAP && self.exp[MASS_INDEX] == 1 && self.exp[LENGTH_INDEX] == -3) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a velocity
    /// 
    /// `length / time`
    pub fn is_velocity(&self) -> bool {
        if self.unit_map & (LENGTH_MAP | TIME_MAP) != self.unit_map {
            return false;
        }
        if self.exp[LENGTH_INDEX] != 1 || self.exp[TIME_INDEX] != -1 {
            return false;
        }
        true
    }

    /// Returns `true` if a `Value` is an acceleration
    /// 
    /// `length / time^2`
    pub fn is_acceleration(&self) -> bool {
        if (self.unit_map & (LENGTH_MAP | TIME_MAP) != self.unit_map) ||
           (self.exp[LENGTH_INDEX] != 1 || self.exp[TIME_INDEX] != -2) {
            return false;
        }
        true
    }

    /// Returns `true` if a `Value` is a force
    /// 
    /// `force`
    /// 
    /// `mass * acceleration`
    pub fn is_force(&self) -> bool {
        if (self.unit_map & (MASS_MAP | LENGTH_MAP | TIME_MAP) == self.unit_map && self.exp[LENGTH_INDEX] == 1 && self.exp[TIME_INDEX] == -2 && self.exp[MASS_INDEX] == 1) ||
           (self.unit_map & FORCE_MAP == self.unit_map && self.exp[FORCE_INDEX] == 1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is measurement of momentum
    /// 
    /// `mass * velocity`
    pub fn is_momentum(&self) -> bool {
        if (self.unit_map & (MASS_MAP | LENGTH_MAP | TIME_MAP) == self.unit_map) && 
           (self.exp[LENGTH_INDEX] == 1 && self.exp[TIME_INDEX] == -1 && self.exp[MASS_INDEX] == 1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a time
    /// 
    /// `time`
    pub fn is_time(&self) -> bool {
        if self.unit_map & TIME_MAP != self.unit_map {
            return false;
        }
        if self.exp[TIME_INDEX] != 1 {
            return false;
        }
        true
    }

    /// Returns `true` if a `Value` is a mass
    /// 
    /// `mass`
    pub fn is_mass(&self) -> bool {
        if self.unit_map == MASS_MAP && self.exp[MASS_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a frequency
    /// 
    /// `frequency`
    /// 
    /// `1/time`
    pub fn is_frequency(&self) -> bool {
        if (self.unit_map == FREQUENCY_MAP && self.exp[FREQUENCY_INDEX] == 1) ||
           (self.unit_map == TIME_MAP && self.exp[TIME_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a pressure
    /// 
    /// `pressure`
    /// 
    /// `force / area`
    /// 
    /// `mass / (length*time^2)`
    pub fn is_pressure(&self) -> bool {
        if (self.unit_map == PRESSURE_MAP && self.exp[PRESSURE_INDEX] == 1) ||
           (self.unit_map == FORCE_MAP | LENGTH_MAP && self.exp[FORCE_INDEX] == 1 && self.exp[LENGTH_INDEX] == -2) ||
           (self.unit_map == MASS_MAP | LENGTH_MAP | TIME_MAP && self.exp[MASS_INDEX] == 1 && self.exp[LENGTH_INDEX] == -1 && self.exp[TIME_INDEX] == -2) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of energy
    /// 
    /// `energy`
    /// 
    /// `length * force`
    /// 
    /// `electric potential * electric charge`
    /// 
    /// `power * time`
    /// 
    /// `mass * area / time^2`
    pub fn is_energy(&self) -> bool {
        if (self.unit_map == ENERGY_MAP && self.exp[ENERGY_INDEX] == 1) ||
           (self.unit_map == LENGTH_MAP | FORCE_MAP && self.exp[LENGTH_INDEX] == 1 && self.exp[FORCE_INDEX] == 1) ||
           (self.unit_map == ELECTRIC_POTENTIAL_MAP | ELECTRIC_CHARGE_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1 && self.exp[ELECTRIC_CHARGE_INDEX] == 1) ||
           (self.unit_map == POWER_MAP | TIME_MAP && self.exp[POWER_INDEX] == 1 && self.exp[TIME_INDEX] == 1) ||
           (self.unit_map == MASS_MAP | LENGTH_MAP | TIME_MAP && self.exp[MASS_INDEX] == 1 && self.exp[LENGTH_INDEX] == 2 && self.exp[TIME_INDEX] == -2) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of power
    /// 
    /// `power`
    /// 
    /// `energy / time`
    /// 
    /// `electric potential * electric current`
    /// 
    /// `mass * area / time^3`
    pub fn is_power(&self) -> bool {
        if (self.unit_map == POWER_MAP && self.exp[POWER_INDEX] == 1) ||
           (self.unit_map == ENERGY_MAP | TIME_MAP && self.exp[ENERGY_INDEX] == 1 && self.exp[TIME_INDEX] == -1) ||
           (self.unit_map == ELECTRIC_POTENTIAL_MAP | ELECTRIC_CURRENT_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1 && self.exp[ELECTRIC_CURRENT_INDEX] == 1) ||
           (self.unit_map == MASS_MAP | LENGTH_MAP | TIME_MAP && self.exp[MASS_INDEX] == 1 && self.exp[LENGTH_INDEX] == 2 && self.exp[TIME_INDEX] == -3) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of electric charge
    /// 
    /// `electric charge`
    /// 
    /// `electric current * time`
    /// 
    /// `electric capacitance * electric potential`
    pub fn is_electric_charge(&self) -> bool {
        if (self.unit_map == ELECTRIC_CHARGE_MAP && self.exp[ELECTRIC_CHARGE_INDEX] == 1) ||
           (self.unit_map == ELECTRIC_CURRENT_MAP | TIME_MAP && self.exp[ELECTRIC_CURRENT_INDEX] == 1 && self.exp[TIME_INDEX] == 1) ||
           (self.unit_map == CAPACITANCE_MAP | ELECTRIC_POTENTIAL_MAP && self.exp[CAPACITANCE_INDEX] == 1 && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of electric current
    /// 
    /// `electric current`
    pub fn is_electric_current(&self) -> bool {
        if self.unit_map == ELECTRIC_CURRENT_MAP && self.exp[ELECTRIC_CURRENT_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of electric potential
    /// 
    /// `electric potential`
    /// 
    /// `power / electric current`
    /// 
    /// `energy / electric charge`
    pub fn is_electric_potential(&self) -> bool {
        if (self.unit_map == ELECTRIC_POTENTIAL_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1) ||
           (self.unit_map == POWER_MAP | ELECTRIC_CURRENT_MAP && self.exp[POWER_INDEX] == 1 && self.exp[ELECTRIC_CURRENT_INDEX] == -1) ||
           (self.unit_map == ENERGY_MAP | ELECTRIC_CHARGE_MAP && self.exp[ENERGY_INDEX] == 1 && self.exp[ELECTRIC_CHARGE_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of electric capacitance
    /// 
    /// `electric capacitance`
    /// 
    /// `electric charge / electric potential`
    /// 
    /// `electric charge^2 / energy`
    pub fn is_capacitance(&self) -> bool {
        if (self.unit_map == CAPACITANCE_MAP && self.exp[CAPACITANCE_INDEX] == 1) ||
           (self.unit_map == ELECTRIC_CHARGE_MAP | ELECTRIC_POTENTIAL_MAP && self.exp[ELECTRIC_CHARGE_INDEX] == 1 && self.exp[ELECTRIC_POTENTIAL_INDEX] == -1) ||
           (self.unit_map == ELECTRIC_CHARGE_MAP | ENERGY_MAP && self.exp[ELECTRIC_CHARGE_INDEX] == 2 && self.exp[ENERGY_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of electric resistance
    /// 
    /// `electric resistance`
    /// 
    /// `1/electric conductance`
    /// 
    /// `electric potential / electric current`
    pub fn is_resistance(&self) -> bool {
        if (self.unit_map == RESISTANCE_MAP && self.exp[RESISTANCE_INDEX] == 1) ||
           (self.unit_map == ELECTRIC_CONDUCTANCE_MAP && self.exp[ELECTRIC_CONDUCTANCE_INDEX] == -1) ||
           (self.unit_map == ELECTRIC_CURRENT_MAP | ELECTRIC_POTENTIAL_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1 && self.exp[ELECTRIC_CURRENT_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of electric conductance
    /// 
    /// `electric conductance`
    /// 
    /// `1/electric resistance`
    /// 
    /// `electric current / electric potential`
    pub fn is_conductance(&self) -> bool {
        if (self.unit_map == ELECTRIC_CONDUCTANCE_MAP && self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 1) ||
           (self.unit_map == RESISTANCE_MAP && self.exp[RESISTANCE_INDEX] == -1) ||
           (self.unit_map == ELECTRIC_CURRENT_MAP | ELECTRIC_POTENTIAL_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == -1 && self.exp[ELECTRIC_CURRENT_INDEX] == 1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of magnetic flux
    /// 
    /// `magnetic flux`
    /// 
    /// `energy / electric current`
    /// 
    /// `magnetic flux density * area`
    /// 
    /// `electric potential * time`
    pub fn is_magnetic_flux(&self) -> bool {
        if (self.unit_map == MAGNETIC_FLUX_MAP && self.exp[MAGNETIC_FLUX_INDEX] == 1) ||
           (self.unit_map == ENERGY_MAP | ELECTRIC_CURRENT_MAP && self.exp[ENERGY_INDEX] == 1 && self.exp[ELECTRIC_CURRENT_INDEX] == -1) ||
           (self.unit_map == MAGNETIC_FLUX_DENSITY_MAP | LENGTH_MAP && self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 1 && self.exp[LENGTH_INDEX] == 2) ||
           (self.unit_map == ELECTRIC_POTENTIAL_MAP | TIME_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1 && self.exp[TIME_INDEX] == 1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of magnetic flux density
    /// 
    /// `magnetic flux density`
    /// 
    /// `electric potential * time / area`
    /// 
    /// `magnetic flux / area`
    /// 
    /// `force / (electric current * length)`
    pub fn is_magnetic_flux_density(&self) -> bool {
        if (self.unit_map == MAGNETIC_FLUX_DENSITY_MAP && self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 1) ||
           (self.unit_map == ELECTRIC_POTENTIAL_MAP | TIME_MAP | LENGTH_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1 && self.exp[TIME_INDEX] == 1 && self.exp[LENGTH_INDEX] == -2) ||
           (self.unit_map == MAGNETIC_FLUX_MAP | LENGTH_MAP && self.exp[MAGNETIC_FLUX_INDEX] == 1 && self.exp[LENGTH_INDEX] == -2) ||
           (self.unit_map == FORCE_MAP | ELECTRIC_CURRENT_MAP | LENGTH_MAP && self.exp[FORCE_INDEX] == 1 && self.exp[ELECTRIC_CURRENT_INDEX] == -1 && self.exp[LENGTH_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of electric inductance
    /// 
    /// `inductance`
    /// 
    /// `electric potential * time / electric current`
    /// 
    /// `electric resistance * time`
    /// 
    /// `magnetic flux / electric current`
    pub fn is_inductance(&self) -> bool {
        if (self.unit_map == INDUCTANCE_MAP && self.exp[INDUCTANCE_INDEX] == 1) ||
           (self.unit_map == ELECTRIC_POTENTIAL_MAP | TIME_MAP | ELECTRIC_CURRENT_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1 && self.exp[TIME_INDEX] == 1 && self.exp[ELECTRIC_CURRENT_INDEX] == -1) ||
           (self.unit_map == RESISTANCE_MAP | TIME_MAP && self.exp[RESISTANCE_INDEX] == 1 && self.exp[TIME_INDEX] == 1) ||
           (self.unit_map == MAGNETIC_FLUX_MAP | ELECTRIC_CURRENT_MAP && self.exp[MAGNETIC_FLUX_INDEX] == 1 && self.exp[ELECTRIC_CURRENT_INDEX] == -1) { 
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of luminous flux
    /// 
    /// `luminous flux`
    pub fn is_luminous_flux(&self) -> bool {
        if self.unit_map == LUMINOUS_FLUX_MAP && self.exp[LUMINOUS_FLUX_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of illuminance
    /// 
    /// `illuminance`
    /// 
    /// `luminous flux / area`
    pub fn is_illuminance(&self) -> bool {
        if (self.unit_map == ILLUMINANCE_MAP && self.exp[ILLUMINANCE_INDEX] == 1) || 
           (self.unit_map == LUMINOUS_FLUX_MAP | LENGTH_MAP && self.exp[LUMINOUS_FLUX_INDEX] == 1 && self.exp[LENGTH_MAP] == -2) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of radioactivity
    /// 
    /// `radioactivity`
    pub fn is_radioactivity(&self) -> bool {
        if self.unit_map == RADIOACTIVITY_MAP && self.exp[RADIOACTIVITY_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of absorbed dose of ionizing radiation
    /// 
    /// `absorbed dose`
    pub fn is_absorbed_dose(&self) -> bool {
        if self.unit_map == ABSORBED_DOSE_MAP && self.exp[ABSORBED_DOSE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of an equivalent dose of ionizing radiation
    /// 
    /// `equivalent dose`
    pub fn is_equivalent_dose(&self) -> bool {
        if self.unit_map == RADIOACTIVITY_EXPOSURE_MAP && self.exp[RADIOACTIVITY_EXPOSURE_MAP] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of catalytic activity
    /// 
    /// `catalytic activity`
    /// 
    /// `substance / time`
    pub fn is_catalytic_activity(&self) -> bool {
        if (self.unit_map == CATALYTIC_ACTIVITY_MAP && self.exp[CATALYTIC_ACTIVITY_INDEX] == 1) || 
           (self.unit_map == SUBSTANCE_MAP | TIME_MAP && self.exp[SUBSTANCE_INDEX] == 1 && self.exp[TIME_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is an angle
    /// 
    /// `angle`
    pub fn is_angle(&self) -> bool {
        if self.unit_map == ANGLE_MAP && self.exp[ANGLE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is an angle measured in radians
    /// 
    /// `angle`
    pub fn is_radians(&self) -> bool {
        if self.unit_map == ANGLE_MAP && self.exp[ANGLE_INDEX] == 1 && self.v_angle == Some(UnitAngle::Radian(Metric::None)) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of information
    /// 
    /// `information`
    pub fn is_information(&self) -> bool {
        if self.unit_map == INFORMATION_MAP && self.exp[INFORMATION_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of luminous intensity
    /// 
    /// `luminous intensity`
    pub fn is_luminous_intensity(&self) -> bool {
        if self.unit_map == LUMINOUS_INTENSITY_MAP && self.exp[LUMINOUS_INTENSITY_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of sound
    /// 
    /// `sound`
    pub fn is_sound(&self) -> bool {
        if self.unit_map == SOUND_MAP && self.exp[SOUND_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of substance
    /// 
    /// `substance`
    pub fn is_substance(&self) -> bool {
        if self.unit_map == SUBSTANCE_MAP && self.exp[SUBSTANCE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a jerk
    /// 
    /// `length / time^3`
    /// 
    /// `school bully`
    pub fn is_jerk(&self) -> bool {
        if self.unit_map == LENGTH_MAP | TIME_MAP && self.exp[LENGTH_INDEX] == 1 && self.exp[TIME_INDEX] == -3 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a snap
    /// 
    /// `length / time^4`
    pub fn is_snap(&self) -> bool {
        if self.unit_map == LENGTH_MAP | TIME_MAP && self.exp[LENGTH_INDEX] == 1 && self.exp[TIME_INDEX] == -4 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of angular velocity
    /// 
    /// `angle / time`
    pub fn is_angular_velocity(&self) -> bool {
        if self.unit_map == ANGLE_MAP | TIME_MAP && self.exp[ANGLE_INDEX] == 1 && self.exp[TIME_INDEX] == -1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of angular acceleration
    /// 
    /// `angle / time^2`
    pub fn is_angular_acceleration(&self) -> bool {
        if self.unit_map == ANGLE_MAP | TIME_MAP && self.exp[ANGLE_INDEX] == 1 && self.exp[TIME_INDEX] == -2 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of frequency drift
    /// 
    /// `frequency / time`
    pub fn is_frequency_drift(&self) -> bool {
        if self.unit_map == FREQUENCY_MAP | TIME_MAP && self.exp[FREQUENCY_INDEX] == 1 && self.exp[TIME_INDEX] == -1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of flow
    /// 
    /// `volume / time`
    pub fn is_flow(&self) -> bool {
        if (self.unit_map == LENGTH_MAP | TIME_MAP && self.exp[LENGTH_INDEX] == 3 && self.exp[TIME_INDEX] == -1) || 
           (self.unit_map == VOLUME_MAP | TIME_MAP && self.exp[VOLUME_INDEX] == 1 && self.exp[TIME_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a yank
    /// 
    /// `force / time`
    pub fn is_yank(&self) -> bool {
        if self.unit_map == FORCE_MAP | TIME_MAP && self.exp[FORCE_INDEX] == 1 && self.exp[TIME_INDEX] == -1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of angular momentum
    /// 
    /// `force * length * time`
    pub fn is_angular_momentum(&self) -> bool {
        if self.unit_map == FORCE_MAP | LENGTH_MAP | TIME_MAP && self.exp[FORCE_INDEX] == 1 && self.exp[TIME_INDEX] == 1 && self.exp[LENGTH_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of torque
    /// 
    /// `force * length`
    /// 
    /// `energy / angle`
    pub fn is_torque(&self) -> bool {
        if (self.unit_map == FORCE_MAP | LENGTH_MAP && self.exp[FORCE_INDEX] == 1 && self.exp[LENGTH_INDEX] == 1) || 
           (self.unit_map == ENERGY_MAP | ANGLE_MAP && self.exp[ENERGY_INDEX] == 1 && self.exp[ANGLE_INDEX] == -1) {
            return true;
        }
        false
    }

    /// Returns `true` if a `Value` is a measurement of energy density
    /// 
    /// `energy / volume`
    pub fn is_energy_density(&self) -> bool {
        if (self.unit_map == ENERGY_MAP | LENGTH_MAP && self.exp[ENERGY_INDEX] == 1 && self.exp[LENGTH_INDEX] == -3) || 
           (self.unit_map == ENERGY_MAP | VOLUME_MAP && self.exp[ENERGY_INDEX] == 1 && self.exp[VOLUME_INDEX] == -1)
         {
            return true;
        }
        false
    }

    
    /// Creates a new unit type when constructing a `Value`
    fn _create_unit(&mut self, units:&str) -> Result<(), V3Error>{
        let tokens:(Vec<String>, Vec<String>) = Value::_get_tokens(units, false)?;

        // do the numors first
        for t in tokens.0 {
            let mut expon:i32 = 1;
            let temp_split:Vec<&str> = t.split('^').collect();
            if temp_split.len() > 1 {
                expon = match temp_split[1].parse::<i32>(){
                    Ok(t) => t,
                    Err(_) => return Err(V3Error::ParsingError("[_create_unit_1] Cannot parse int"))
                };
            }
            self._parse_units(temp_split[0], expon)?;
        }

        // now the denoms
        for t in tokens.1 {
            let mut expon:i32 = -1;
            let temp_split:Vec<&str> = t.split('^').collect();
            if temp_split.len() > 1 {
                expon *= match temp_split[1].parse::<i32>() {
                    Ok(t) => t,
                    Err(_) => return Err(V3Error::ParsingError("[_create_unit_2] Cannot parse int"))
                };
            }
            self._parse_units(temp_split[0], expon)?;
        }

        Ok(())
    }

    /// Tokenizes a given string for a new `Value` for easier parsing
    fn _get_tokens(block:&str, do_denom:bool) -> Result<(Vec<String>, Vec<String>), V3Error> {
        let mut numor:Vec<String> = Vec::new();
        let mut denom:Vec<String> = Vec::new();
    
        // first we find the outer most parentheses
        // if there are non we just continue
        let mut left_count:usize = 0;
        let mut start_index:usize = 0;
        let mut end_index:usize;
        let mut found_divisor:bool = do_denom;
        let mut constructor:String = String::new();
        for index in 0..block.chars().count() {
            let c:char = match block.chars().nth(index) {
                Some(t) => t,
                None => return Err(V3Error::ParsingError("[_get_tokens] Index error"))
            };
            match c {
                '(' => {
                    if left_count == 0 {
                        start_index = index+1;
                    }
                    left_count+=1;
                }
                ')' => {
                    left_count-=1;
                    if left_count == 0 {
                        end_index = index;
                        let mut ret:(Vec<String>, Vec<String>) = Self::_get_tokens(&block[start_index..end_index], found_divisor)?;
                        numor.append(&mut ret.0);
                        denom.append(&mut ret.1);
                    }
                }
                '/' => {
                    if !found_divisor {
                        found_divisor = true;
                    } else {
                        //return Err(V2Error::ParsingError("Too many divisors".to_string()));
                    }
                    if !found_divisor && !constructor.is_empty() {
                        denom.push(constructor.clone());
                    } else if !constructor.is_empty() {
                        numor.push(constructor.clone());
                    }
                    constructor = String::new();
                }
                _ => {
                    if left_count == 0 {
                        if c.is_whitespace() {
                            // Do nothing
                        } else if c == '*' {
                            if !do_denom && !found_divisor {
                                numor.push(constructor.clone());
                            } else {
                                denom.push(constructor.clone());
                            }
                            constructor = String::new();
                        } else {
                            constructor.push(c);
                        }
                    }
                }
            };
        }
    
        if !constructor.is_empty() {
            if !do_denom && !found_divisor {
                numor.push(constructor.clone());
            } else {
                denom.push(constructor.clone());
            }
        }
    
        Ok((numor, denom))
    }

    /// Searches through the given string for a new `Value` to parse for units
    fn _parse_units(&mut self, unit:&str, exp:i32) -> Result<(), V3Error> {
        let l:usize = unit.chars().count();
        if l == 0 {
            return Ok(());
        }

        // first match it against known unique strings
        match unit {
            "mph" => {
                if exp != 1 && exp != -1 {
                    return Err(V3Error::ParsingError("[_parse_units] MPH exponent"));
                }
                self.v_length = Some(UnitLength::Mile);
                self.exp[LENGTH_INDEX] = exp;
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = -exp;
                self.unit_map = LENGTH_MAP | TIME_MAP;
                return Ok(());
            }
            "kph" => {
                if exp != 1 && exp != -1 {
                    return Err(V3Error::ParsingError("[_parse_units] KPH exponent"));
                }
                self.v_length = Some(UnitLength::Meter(Metric::Kilo));
                self.exp[LENGTH_INDEX] = exp;
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = -exp;
                self.unit_map |= LENGTH_MAP | TIME_MAP;
                return Ok(());
            }
            "mmHg" => {
                self.v_pressure = Some(UnitPressure::Hgmm);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "inHg" => {
                self.v_pressure = Some(UnitPressure::Hgin);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "byte" | "bytes" => {
                self.v_information = Some(UnitInformation::Byte(Metric::None));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
                return Ok(());
            }
            "bit" | "bits" => {
                self.v_information = Some(UnitInformation::Bit(Metric::None));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
                return Ok(());
            }
            "radian" | "radians" => {
                self.v_angle = Some(UnitAngle::Radian(Metric::None));
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "angstrom" | "angstroms" => {
                self.v_length = Some(UnitLength::Angstrom);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "inches" | "inch" | "in" => {
                self.v_length = Some(UnitLength::Inch);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "foot" | "feet" | "ft" => {
                self.v_length = Some(UnitLength::Foot);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "yard" | "yards" | "yd" | "yds" => {
                self.v_length = Some(UnitLength::Yard);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "mile" | "miles" => {
                self.v_length = Some(UnitLength::Mile);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "atm" | "ATM" => {
                self.v_pressure = Some(UnitPressure::Atm);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "psi" | "PSI" => {
                self.v_pressure = Some(UnitPressure::Psi);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "f" | "°f" | "°F" => {
                self.v_temperature = Some(UnitTemperature::Fahrenheit);
                self.exp[TEMPERATURE_INDEX] = exp;
                self.unit_map |= TEMPERATURE_MAP;
                return Ok(());
            }
            "c" | "°c" | "°C" => {
                self.v_temperature = Some(UnitTemperature::Celsius);
                self.exp[TEMPERATURE_INDEX] = exp;
                self.unit_map |= TEMPERATURE_MAP;
                return Ok(());
            }
            "footpound" | "footpounds" | "ftlb" | "ftlbs" => {
                self.v_energy = Some(UnitEnergy::FootPound);
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
                return Ok(());
            }
            "poundforce" | "poundsforce" | "lbfr" | "lbsfr" => {
                self.v_force = Some(UnitForce::PoundForce);
                self.exp[FORCE_INDEX] = exp;
                self.unit_map |= FORCE_MAP;
                return Ok(());
            }
            "ounce" | "ounces" | "oz" => {
                self.v_mass = Some(UnitMass::Ounce);
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
                return Ok(());
            }
            "grain" | "grains" | "gr" => {
                self.v_mass = Some(UnitMass::Grain);
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
                return Ok(());
            }
            "pounds" | "lbs" | "lb" => {
                self.v_mass = Some(UnitMass::Pound);
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
                return Ok(());
            }
            "moa" | "MOA" => {
                self.v_angle = Some(UnitAngle::Moa);
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "rads" | "Rads" => {
                self.v_ab_dose = Some(UnitAbsorbedDose::Rad);
                self.exp[ABSORBED_DOSE_INDEX] = exp;
                self.unit_map |= ABSORBED_DOSE_MAP;
                return Ok(());
            }
            "rem" | "Rem" => {
                self.v_radioactivity_exposure = Some(UnitRadioactivityExposure::Rem);
                self.exp[RADIOACTIVITY_EXPOSURE_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
                return Ok(());
            }
            "mil" | "MIL" | "mils" => {
                self.v_angle = Some(UnitAngle::Radian(Metric::Milli));
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "degrees" | "degree" | "°" => {
                self.v_angle = Some(UnitAngle::Degree);
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "lyr" | "lightyear" | "lightyears" => {
                self.v_length = Some(UnitLength::LightYear);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "farad" | "farads" => {
                self.v_capacitance = Some(UnitCapacitance::Farad(Metric::None));
                self.exp[CAPACITANCE_INDEX] = exp;
                self.unit_map |= CAPACITANCE_MAP;
                return Ok(());
            }
            "micron" | "microns" => {
                self.v_length = Some(UnitLength::Meter(Metric::Micro));
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "min" | "minute" | "minutes" => {
                self.v_time = Some(UnitTime::Minute);
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
                return Ok(());
            }
            "h" | "hr" | "hour" | "hours" => {
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
                return Ok(());
            }
            "d" | "day" | "days" => {
                self.v_time = Some(UnitTime::Day);
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
                return Ok(());
            }
            "eV" => {
                self.v_energy = Some(UnitEnergy::ElectronVolt);
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
                return Ok(());
            }
            _ => {
                // do nothing
            }
        }

        if l == 1 {
            self._get_single_letter(unit, exp, Metric::None)?;
        } else if l == 2 {
            self._get_double_letter(unit, exp, Metric::None)?;
        } else if l == 3 {
            self._get_triple_letter(unit, exp, Metric::None)?;
        } else if l == 4 {
            self._get_quadruple_letter(unit, exp, Metric::None)?;
        } else if l == 5 {
            self._get_pentuple_letter(unit, exp, Metric::None)?;
        } else {
            return Err(V3Error::UnsupportedUnit(format!("[_parse_units] Unit {} exceeds parsing bounds", unit)));
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a `Value` during string parsing and construction
    fn _get_single_letter(&mut self, unit:&str, exp:i32, m:Metric) -> Result<(), V3Error>{
        match unit {
            "m" => {
                self.v_length = Some(UnitLength::Meter(m));
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            "g" => {
                self.v_mass = Some(UnitMass::Gram(m));
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
            }
            "s" => {
                self.v_time = Some(UnitTime::Second(m));
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
            }
            "A" => {
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(m));
                self.exp[ELECTRIC_CURRENT_INDEX] = exp;
                self.unit_map |= ELECTRIC_CURRENT_MAP;
            }
            "J" => {
                self.v_energy = Some(UnitEnergy::Joule(m));
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
            }
            "W" => {
                self.v_power = Some(UnitPower::Watt(m));
                self.exp[POWER_INDEX] = exp;
                self.unit_map |= POWER_MAP;
            }
            "C" => {
                self.v_electric_charge = Some(UnitElectricCharge::Coulomb(m));
                self.exp[ELECTRIC_CHARGE_INDEX] = exp;
                self.unit_map |= ELECTRIC_CHARGE_MAP;
            }
            "F" => {
                self.v_capacitance = Some(UnitCapacitance::Farad(m));
                self.exp[CAPACITANCE_INDEX] = exp;
                self.unit_map |= CAPACITANCE_MAP;
            }
            "Ω" | "O" => {
                self.v_resistance = Some(UnitResistance::Ohm(m));
                self.exp[RESISTANCE_INDEX] = exp;
                self.unit_map |= RESISTANCE_MAP;
            }
            "S" => {
                self.v_electric_conductance = Some(UnitElectricConductance::Siemens(m));
                self.exp[ELECTRIC_CONDUCTANCE_INDEX] = exp;
                self.unit_map |= ELECTRIC_CONDUCTANCE_MAP;
            }
            "T" => {
                self.v_magnetic_flux_density = Some(UnitMagneticFluxDensity::Tesla(m));
                self.exp[MAGNETIC_FLUX_DENSITY_INDEX] = exp;
                self.unit_map |= MAGNETIC_FLUX_DENSITY_MAP;
            }
            "N" => {
                self.v_force = Some(UnitForce::Newton(m));
                self.exp[FORCE_INDEX] = exp;
                self.unit_map |= FORCE_MAP;
            }
            "K" => {
                // if m, error
                // TODO add kelvin metric support
                self.v_temperature = Some(UnitTemperature::Kelvin);
                self.exp[TEMPERATURE_INDEX] = exp;
                self.unit_map |= TEMPERATURE_MAP;
            }
            "H" => {
                self.v_inductance = Some(UnitInductance::Henry(m));
                self.exp[INDUCTANCE_INDEX] = exp;
                self.unit_map |= INDUCTANCE_MAP;
            }
            "V" => {
                self.v_electric_potential = Some(UnitElectricPotential::Volt(m));
                self.exp[ELECTRIC_POTENTIAL_INDEX] = exp;
                self.unit_map |= ELECTRIC_POTENTIAL_MAP;
            }
            "B" => {
                self.v_sound = Some(UnitSound::Bel(m));
                self.exp[SOUND_INDEX] = exp;
                self.unit_map |= SOUND_MAP;
            }
            "b" => {
                self.v_information = Some(UnitInformation::Byte(m));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
            }
            "Å" => {
                self.v_length = Some(UnitLength::Angstrom);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            "R" => {
                self.v_ab_dose = Some(UnitAbsorbedDose::Roentgen);
                self.exp[ABSORBED_DOSE_INDEX] = exp;
                self.unit_map |= ABSORBED_DOSE_MAP;
            }
            "l" => {
                self.v_volume = Some(UnitVolume::Liter(m));
                self.exp[VOLUME_INDEX] = exp;
                self.unit_map |= VOLUME_MAP;
            }
            _ => {
                return Err(V3Error::UnsupportedUnit(format!("[_get_single_letter] Unsupported unit: {}", unit)));
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a `Value` during string parsing and construction
    fn _get_double_letter(&mut self, unit:&str, exp:i32, m:Metric) -> Result<(), V3Error> {
        match unit {
            "Hz" => {
                self.v_frequency = Some(UnitFrequency::Hertz(m));
                self.exp[FREQUENCY_INDEX] = exp;
                self.unit_map |= FREQUENCY_MAP;
            }
            "Pa" => {
                self.v_pressure = Some(UnitPressure::Pascal(m));
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
            }
            "Wb" => {
                self.v_magnetic_flux = Some(UnitMagneticFlux::Weber(m));
                self.exp[MAGNETIC_FLUX_INDEX] = exp;
                self.unit_map |= MAGNETIC_FLUX_MAP;
            }
            "lm" => {
                self.v_luminous_flux = Some(UnitLuminousFlux::Lumen(m));
                self.exp[LUMINOUS_FLUX_INDEX] = exp;
                self.unit_map |= LUMINOUS_FLUX_MAP;
            }
            "lx" => {
                self.v_illuminance = Some(UnitIlluminance::Lux(m));
                self.exp[ILLUMINANCE_INDEX] = exp;
                self.unit_map |= ILLUMINANCE_MAP;
            }
            "Bq" => {
                self.v_radioactivity = Some(UnitRadioactivity::Becquerel(m));
                self.exp[RADIOACTIVITY_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_MAP;
            }
            "Sv" => {
                self.v_radioactivity_exposure = Some(UnitRadioactivityExposure::Sievert(m));
                self.exp[RADIOACTIVITY_EXPOSURE_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
            }
            "cd" => {
                self.v_luminous_flux_intensity = Some(UnitLuminousIntensity::Candela(m));
                self.exp[LUMINOUS_INTENSITY_INDEX] = exp;
                self.unit_map |= LUMINOUS_INTENSITY_MAP;
            }
            "au" | "AU" => {
                self.v_length = Some(UnitLength::AstronomicalUnit);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            "pc" => {
                self.v_length = Some(UnitLength::Parsec);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            "Ci" => {
                self.v_radioactivity = Some(UnitRadioactivity::Curie);
                self.exp[RADIOACTIVITY_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_MAP;
            }
            "Gy" => {
                self.v_ab_dose = Some(UnitAbsorbedDose::Gray(m));
                self.exp[ABSORBED_DOSE_INDEX] = exp;
                self.unit_map |= ABSORBED_DOSE_MAP;
            }
            "sr" => {
                self.v_solid_angle = Some(UnitSolidAngle::Steradian(m))
            }
            _ => {
                if m != Metric::None {
                    return Err(V3Error::UnsupportedUnit(format!("[_get_double_letter] Unsupported unit: {}", unit)));
                }
                match self._get_metric(match &unit.chars().next() {
                    Some(t) => t,
                    None => return Err(V3Error::ParsingError("[_get_double_letter] Cannot get next metric char"))
                }) {
                    Ok(new_m) => self._get_single_letter(&unit[1..], exp, new_m)?,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a `Value` during string parsing and construction
    fn _get_triple_letter(&mut self, unit:&str, exp:i32, m:Metric) -> Result<(), V3Error> {

        if let Some(da) = unit.strip_prefix("da") {
            return self._get_single_letter(da, exp, Metric::Deca);
        }

        match unit {
            "mol" => {
                self.v_substance = Some(UnitSubstance::Mole(m));
                self.exp[SUBSTANCE_INDEX] = exp;
                self.unit_map |= SUBSTANCE_MAP;
            }
            "kat" => {
                self.v_catalytic = Some(UnitCatalyticActivity::Katal(m));
                self.exp[CATALYTIC_ACTIVITY_INDEX] = exp;
                self.unit_map |= CATALYTIC_ACTIVITY_MAP;
            }
            "rad" => {
                self.v_angle = Some(UnitAngle::Radian(m));
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
            }
            "bar" => {
                self.v_pressure = Some(UnitPressure::Bar(m));
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
            }
            "Cal" => {
                // if m is not empty error out
                self.v_energy = Some(UnitEnergy::GramCalorie(Metric::Kilo));
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
            }
            "cal" => {
                self.v_energy = Some(UnitEnergy::GramCalorie(m));
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
            }
            _ => {
                if m != Metric::None {
                    return Err(V3Error::UnsupportedUnit(format!("[_get_triple_letter] Unsupported unit: {}", unit)));
                }
                match self._get_metric(match &unit.chars().next() {
                    Some(t) => t,
                    None => return Err(V3Error::ParsingError("[_get_triple_letter] Cannot get next metric char"))
                }) {
                    Ok(new_m) => self._get_double_letter(&unit[1..], exp, new_m)?,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a `Value` during string parsing and construction
    fn _get_quadruple_letter(&mut self, unit:&str, exp:i32, m:Metric) -> Result<(), V3Error> {

        if let Some(da) = unit.strip_prefix("da") {
            return self._get_double_letter(da, exp, Metric::Deca);
        }

        match unit {
            "torr" => {
                self.v_pressure = Some(UnitPressure::Torr);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
            }
            "bits" => {
                self.v_information = Some(UnitInformation::Bit(m));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
            }
            _ => {
                if m != Metric::None {
                    return Err(V3Error::UnsupportedUnit(format!("[_get_quadruple_letter] Unsupported unit: {}", unit)));
                }
                match self._get_metric(match &unit.chars().next() {
                    Some(t) => t,
                    None => return Err(V3Error::ParsingError("[_get_quadruple_letter] Cannot get next metric char"))
                }) {
                    Ok(new_m) => self._get_triple_letter(&unit[1..], exp, new_m)?,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a `Value` during string parsing and construction
    fn _get_pentuple_letter(&mut self, unit:&str, exp:i32, m:Metric) -> Result<(), V3Error> {

        if let Some(da) = unit.strip_prefix("da") {
            return self._get_triple_letter(da, exp, Metric::Deca);
        }

        if m != Metric::None {
            return Err(V3Error::UnsupportedUnit(format!("[_get_pentuple_letter] Unsupported unit: {}", unit)));
        }
        match self._get_metric(match &unit.chars().next() {
            Some(t) => t,
            None => return Err(V3Error::ParsingError("[_get_pentuple_letter] Cannot get next metric char"))
        }) {
            Ok(new_m) => self._get_quadruple_letter(&unit[1..], exp, new_m),
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Returns the `Metric` enum for a given prefix
    fn _get_metric(&mut self, unit:&char) -> Result<Metric, V3Error> {
        match unit {
            'Y' => Ok(Metric::Yotta),
            'Z' => Ok(Metric::Zetta),
            'E' => Ok(Metric::Exa),
            'P' => Ok(Metric::Peta),
            'T' => Ok(Metric::Tera),
            'G' => Ok(Metric::Giga),
            'M' => Ok(Metric::Mega),
            'k' => Ok(Metric::Kilo),
            'h' => Ok(Metric::Hecto),
            'd' => Ok(Metric::Deci),
            'c' => Ok(Metric::Centi),
            'm' => Ok(Metric::Milli),
            'u' | 'μ' => Ok(Metric::Micro),
            'n' => Ok(Metric::Nano),
            'p' => Ok(Metric::Pico),
            'f' => Ok(Metric::Femto),
            'a' => Ok(Metric::Atto),
            'z' => Ok(Metric::Zepto),
            'y' => Ok(Metric::Yocto),
            _ => {
                Err(V3Error::UnsupportedMetric(format!("[_get_metric] Unsupported metric: {}", unit)))
            }
        }
    }

    /// Returns `true` if two `Values` have comparable, not equal, unit types
    pub(in crate::values) fn __equivalent(&self, other:&Value) -> bool {

        if self.unit_map == VOLUME_MAP && other.unit_map == LENGTH_MAP {
            if self.exp[VOLUME_INDEX] == 1 && other.exp[LENGTH_INDEX] == 3 {
                return true;
            } 
            return false;
        } else if self.unit_map == LENGTH_MAP && other.unit_map == VOLUME_MAP {
            if self.exp[LENGTH_INDEX] == 3 && other.exp[VOLUME_INDEX] == 1 {
                return true;
            }
            return false;
        }

        if self.unit_map != other.unit_map {
            return false;
        }

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                return false;
            }
        }

        true
    }

    /// Checks if the `Value` unit types are the same
    pub(in crate::values) fn __equal(&self, other:&Value) -> bool {
        if self.unit_map != other.unit_map {
            return false;
        }
        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                return false;
            }
            let region:usize = 1<<i;
            if region & self.unit_map != 0 {
                match region {
                    LENGTH_MAP => {
                        if self.v_length.unwrap() != other.v_length.unwrap() {
                            return false;
                        }
                    }
                    TIME_MAP => {
                        if self.v_time.unwrap() != other.v_time.unwrap() {
                            return false
                        }
                    }
                    MASS_MAP => {
                        if self.v_mass.unwrap() != other.v_mass.unwrap() {
                            return false;
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if self.v_electric_current.unwrap() != other.v_electric_current.unwrap() {
                            return false;
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge.unwrap() != other.v_electric_charge.unwrap() {
                            return false;
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential.unwrap() != other.v_electric_potential.unwrap() {
                            return false;
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance.unwrap() != other.v_electric_conductance.unwrap() {
                            return false;
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance.unwrap() != other.v_capacitance.unwrap() {
                            return false;
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance.unwrap() != other.v_resistance.unwrap() {
                            return false;
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance.unwrap() != other.v_inductance.unwrap() { 
                            return false;
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux.unwrap() != other.v_magnetic_flux.unwrap() {
                            return false;
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density.unwrap() != other.v_magnetic_flux_density.unwrap() {
                            return false;
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature.unwrap() != other.v_temperature.unwrap() {
                            return false;
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance.unwrap() != other.v_substance.unwrap() {
                            return false;
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity.unwrap() != other.v_luminous_flux_intensity.unwrap() {
                            return false;
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux.unwrap() != other.v_luminous_flux.unwrap() {
                            return false;
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance.unwrap() != other.v_illuminance.unwrap() {
                            return false;
                        }
                    }
                    VOLUME_MAP => {
                        if self.v_volume.unwrap() != other.v_volume.unwrap() {
                            return false;
                        }
                    }
                    PRESSURE_MAP => {
                        if self.v_pressure.unwrap() != other.v_pressure.unwrap() {
                            return false;
                        }
                    }
                    ANGLE_MAP => {
                        if self.v_angle.unwrap() != other.v_angle.unwrap() {
                            return false;
                        }
                    }
                    FREQUENCY_MAP => {
                        if self.v_frequency.unwrap() != other.v_frequency.unwrap() {
                            return false;
                        }
                    }
                    FORCE_MAP => {
                        if self.v_force.unwrap() != other.v_force.unwrap() {
                            return false;
                        }
                    }
                    ENERGY_MAP => {
                        if self.v_energy.unwrap() != other.v_energy.unwrap() {
                            return false;
                        }
                    }
                    POWER_MAP => {
                        if self.v_power.unwrap() != other.v_power.unwrap() {
                            return false;
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if self.v_radioactivity.unwrap() != other.v_radioactivity.unwrap() {
                            return false;
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose.unwrap() != other.v_ab_dose.unwrap() {
                            return false;
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure.unwrap() != other.v_radioactivity_exposure.unwrap() {
                            return false;
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic.unwrap() != other.v_catalytic.unwrap() {
                            return false;
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound.unwrap() != other.v_sound.unwrap() {
                            return false;
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information.unwrap() != other.v_information.unwrap() {
                            return false;
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle.unwrap() != other.v_solid_angle.unwrap() {
                            return false;
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot compare Values {} and {}", self, other);
                    }
                }
            }
        }
        true
    }
}