use crate::constants::*;
use crate::errors::V3Error;
use crate::units::Convert;
use crate::units::Metric;
use crate::units::UnitAngle;
use crate::values::Value;

impl Default for Value {
    /// The default constructor for a [`Value`]
    ///
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// let mut m:Value = Value::default();
    /// m.val = 1.3;
    /// ```
    fn default() -> Value {
        Value {
            val: 0.0,
            unit_map: 0,
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
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux: None,
            v_magnetic_flux_density: None,
            v_solid_angle: None,
        }
    }
}

impl Value {
    /// The main constructor for a [`Value`]
    ///
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// let m:Value = match Value::new(4.5, "m") {
    ///     Ok(v) => v,
    ///     Err(e) => panic!("{}", e)
    /// };
    /// ```
    pub fn new(val: f64, units: &str) -> Result<Value, V3Error> {
        let mut ret: Value = Value {
            val,
            unit_map: 0,
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
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux: None,
            v_magnetic_flux_density: None,
            v_solid_angle: None,
        };
        ret._create_unit(units)?;

        Ok(ret)
    }

    /// Creates a [`Value`] specifically in radians
    fn _radians(val: f64) -> Value {
        let mut ret: Value = Value {
            val,
            unit_map: ANGLE_MAP,
            exp: [0; 31],
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
            v_magnetic_flux: None,
            v_magnetic_flux_density: None,
            v_solid_angle: None,
        };
        ret.exp[ANGLE_INDEX] = 1;
        ret
    }

    /// Inverses the [`Value`]
    ///
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::UnitLength;
    /// let mut v:Value = 4.0 * UnitLength::Inch;
    /// v.inv()
    /// ```
    /// `v` will now be equal to `0.25 1/in`
    pub fn inv(&mut self) {
        self.val = 1.0 / self.val;
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
    /// let mut a:Value = 45.0 * UnitAngle::Degree;
    /// a.to_radians();
    /// ```
    pub fn to_radians(&mut self) {
        if self.unit_map != ANGLE_MAP && self.exp[ANGLE_INDEX] != 1 {
            panic!("[to_radians] Cannot convert non angle to radians");
        }
        self.val *= self
            .v_angle
            .unwrap()
            .convert(&UnitAngle::Radian(Metric::None));
    }

    /// Converts an angle to degrees
    ///
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::{UnitAngle, Metric};
    /// let mut a:Value = (2.0/std::f64::consts::PI) * UnitAngle::Radian(Metric::None);
    /// a.to_degrees();
    /// ```
    pub fn to_degrees(&mut self) {
        if self.unit_map != ANGLE_MAP && self.exp[ANGLE_INDEX] != 1 {
            panic!("[to_degrees] Cannot convert non angle to degrees");
        }
        self.val *= self.v_angle.unwrap().convert(&UnitAngle::Degree);
    }

    /// Returns if the [`Value`] numeric is NAN
    pub fn is_nan(&self) -> bool {
        self.val.is_nan()
    }

    /// Returns if the [`Value`] numeric is finite
    pub fn is_finite(&self) -> bool {
        self.val.is_finite()
    }

    /// Returns if the [`Value`] numeric is infinite
    pub fn is_infinite(&self) -> bool {
        self.val.is_infinite()
    }

    /// Returns if the [`Value`] numeric is normal
    pub fn is_normal(&self) -> bool {
        self.val.is_normal()
    }

    /// Returns if the [`Value`] numeric is subnormal
    pub fn is_subnormal(&self) -> bool {
        self.val.is_subnormal()
    }

    /// Returns if the [`Value`] numeric is sign positive
    pub fn is_sign_positive(&self) -> bool {
        self.val.is_sign_positive()
    }

    /// Returns if the [`Value`] numeric is sign negative
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
    /// let mut v:Value = 16.0 * UnitLength::Foot * UnitLength::Foot;
    /// let x:Value = v.sqrt();
    /// ```
    /// `x` will be equal to `4.0 ft`
    pub fn sqrt(&self) -> Value {
        let mut n: Value = *self;
        for i in 0..31_usize {
            if n.exp[i] % 2 != 0 {
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
    /// let v:Value = 4.0 * UnitLength::Foot;
    /// let x:Value = v.powv(2);
    /// assert!(String::from("16 ft^2") == format!("{}", x));
    /// ```
    pub fn powv(&self, p: i32) -> Value {
        let mut n: Value = *self;
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
    /// let mut v:Value = 9.0 * UnitLength::Foot * UnitLength::Foot * UnitLength::Foot;
    /// let x:Value = v.cbrt();
    /// ```
    /// `x` will be equal to `3.0 ft`
    pub fn cbrt(&self) -> Value {
        let mut n: Value = *self;
        for i in 0..31_usize {
            if n.exp[i] % 3 != 0 {
                panic!("[cbrt] Cannot cube root Value: {}", self);
            }
            n.exp[i] /= 3;
        }
        n.val = n.val.cbrt();
        n
    }

    /// Returns the sine of a [`Value`] in radians
    pub fn sin(&self) -> Value {
        Value::_radians(self.val.sin())
    }

    /// Returns the cosine of a [`Value`] in radians
    pub fn cos(&self) -> Value {
        Value::_radians(self.val.cos())
    }

    /// Returns the tangent of a [`Value`] in radians
    pub fn tan(&self) -> Value {
        Value::_radians(self.val.tan())
    }

    /// Returns the tangent-h of a [`Value`] in radians
    pub fn tanh(&self) -> Value {
        Value::_radians(self.val.tanh())
    }

    /// Returns the arcsine of a [`Value`] in radians
    pub fn asin(&self) -> Value {
        Value::_radians(self.val.asin())
    }

    /// Returns the arccosine of a [`Value`] in radians
    pub fn acos(&self) -> Value {
        Value::_radians(self.val.acos())
    }

    /// Returns the arctangent of a [`Value`] in radians
    pub fn atan(&self) -> Value {
        Value::_radians(self.val.atan())
    }

    /// Returns the full unit circle arctangent of a [`Value`] in radians
    ///
    /// atan2 will panic if the the given [`Value`] is not an angle.
    ///
    /// # Example
    /// ```rust
    /// use v3::values::Value;
    /// use v3::units::{UnitTime, UnitAngle, Metric};
    /// let a:Value = 10.0 * UnitTime::Second(Metric::None);
    /// let b:Value = 0.3 * UnitAngle::Radian(Metric::None);
    /// let x:Value = a.atan2(&b);
    /// ```
    /// `x` will be approximately equal to `1.5408 radians`
    pub fn atan2(&self, other: &Value) -> Value {
        if !other.is_angle() {
            panic!("[atan2] atan2 requires an Value angle");
        }
        let new_v: f64 = other.val
            * other
                .v_angle
                .unwrap()
                .convert(&UnitAngle::Radian(Metric::None));
        Value::_radians(self.val.atan2(new_v))
    }

    /// Returns `true` if a [`Value`] has no units
    ///
    /// <none>
    pub fn is_empty(&self) -> bool {
        if self.unit_map == 0 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a length
    ///
    /// `length`
    pub fn is_length(&self) -> bool {
        if self.unit_map & LENGTH_MAP != self.unit_map || self.exp[LENGTH_INDEX] != 1 {
            return false;
        }
        true
    }

    /// Returns `true` if a [`Value`] is an area
    ///
    /// `length^2`
    pub fn is_area(&self) -> bool {
        if self.unit_map & LENGTH_MAP != self.unit_map || self.exp[LENGTH_INDEX] != 2 {
            return false;
        }
        true
    }

    /// Returns `true` if a [`Value`] is a volume
    ///
    /// `volume`
    ///
    /// `length^3`
    pub fn is_volume(&self) -> bool {
        if (self.unit_map == LENGTH_MAP && self.exp[LENGTH_INDEX] == 3)
            || (self.unit_map == VOLUME_MAP && self.exp[VOLUME_INDEX] == 1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a temperature
    ///
    /// `temperature`
    pub fn is_temperature(&self) -> bool {
        if self.unit_map == TEMPERATURE_MAP && self.exp[TEMPERATURE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a density
    ///
    /// `mass / volume`
    pub fn is_density(&self) -> bool {
        if (self.unit_map == MASS_MAP | VOLUME_MAP
            && self.exp[MASS_INDEX] == 1
            && self.exp[VOLUME_INDEX] == -1)
            || (self.unit_map == MASS_MAP | LENGTH_MAP
                && self.exp[MASS_INDEX] == 1
                && self.exp[LENGTH_INDEX] == -3)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a velocity
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

    /// Returns `true` if a [`Value`] is an acceleration
    ///
    /// `length / time^2`
    pub fn is_acceleration(&self) -> bool {
        if (self.unit_map & (LENGTH_MAP | TIME_MAP) != self.unit_map)
            || (self.exp[LENGTH_INDEX] != 1 || self.exp[TIME_INDEX] != -2)
        {
            return false;
        }
        true
    }

    /// Returns `true` if a [`Value`] is a force
    ///
    /// `force`
    ///
    /// `mass * acceleration`
    pub fn is_force(&self) -> bool {
        if (self.unit_map & (MASS_MAP | LENGTH_MAP | TIME_MAP) == self.unit_map
            && self.exp[LENGTH_INDEX] == 1
            && self.exp[TIME_INDEX] == -2
            && self.exp[MASS_INDEX] == 1)
            || (self.unit_map & FORCE_MAP == self.unit_map && self.exp[FORCE_INDEX] == 1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is measurement of momentum
    ///
    /// `mass * velocity`
    pub fn is_momentum(&self) -> bool {
        if (self.unit_map & (MASS_MAP | LENGTH_MAP | TIME_MAP) == self.unit_map)
            && (self.exp[LENGTH_INDEX] == 1
                && self.exp[TIME_INDEX] == -1
                && self.exp[MASS_INDEX] == 1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a time
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

    /// Returns `true` if a [`Value`] is a mass
    ///
    /// `mass`
    pub fn is_mass(&self) -> bool {
        if self.unit_map == MASS_MAP && self.exp[MASS_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a frequency
    ///
    /// `frequency`
    ///
    /// `1/time`
    pub fn is_frequency(&self) -> bool {
        if (self.unit_map == FREQUENCY_MAP && self.exp[FREQUENCY_INDEX] == 1)
            || (self.unit_map == TIME_MAP && self.exp[TIME_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a pressure
    ///
    /// `pressure`
    ///
    /// `force / area`
    ///
    /// `mass / (length*time^2)`
    pub fn is_pressure(&self) -> bool {
        if (self.unit_map == PRESSURE_MAP && self.exp[PRESSURE_INDEX] == 1)
            || (self.unit_map == FORCE_MAP | LENGTH_MAP
                && self.exp[FORCE_INDEX] == 1
                && self.exp[LENGTH_INDEX] == -2)
            || (self.unit_map == MASS_MAP | LENGTH_MAP | TIME_MAP
                && self.exp[MASS_INDEX] == 1
                && self.exp[LENGTH_INDEX] == -1
                && self.exp[TIME_INDEX] == -2)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of energy
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
        if (self.unit_map == ENERGY_MAP && self.exp[ENERGY_INDEX] == 1)
            || (self.unit_map == LENGTH_MAP | FORCE_MAP
                && self.exp[LENGTH_INDEX] == 1
                && self.exp[FORCE_INDEX] == 1)
            || (self.unit_map == ELECTRIC_POTENTIAL_MAP | ELECTRIC_CHARGE_MAP
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1
                && self.exp[ELECTRIC_CHARGE_INDEX] == 1)
            || (self.unit_map == POWER_MAP | TIME_MAP
                && self.exp[POWER_INDEX] == 1
                && self.exp[TIME_INDEX] == 1)
            || (self.unit_map == MASS_MAP | LENGTH_MAP | TIME_MAP
                && self.exp[MASS_INDEX] == 1
                && self.exp[LENGTH_INDEX] == 2
                && self.exp[TIME_INDEX] == -2)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of power
    ///
    /// `power`
    ///
    /// `energy / time`
    ///
    /// `electric potential * electric current`
    ///
    /// `mass * area / time^3`
    pub fn is_power(&self) -> bool {
        if (self.unit_map == POWER_MAP && self.exp[POWER_INDEX] == 1)
            || (self.unit_map == ENERGY_MAP | TIME_MAP
                && self.exp[ENERGY_INDEX] == 1
                && self.exp[TIME_INDEX] == -1)
            || (self.unit_map == ELECTRIC_POTENTIAL_MAP | ELECTRIC_CURRENT_MAP
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1
                && self.exp[ELECTRIC_CURRENT_INDEX] == 1)
            || (self.unit_map == MASS_MAP | LENGTH_MAP | TIME_MAP
                && self.exp[MASS_INDEX] == 1
                && self.exp[LENGTH_INDEX] == 2
                && self.exp[TIME_INDEX] == -3)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of electric charge
    ///
    /// `electric charge`
    ///
    /// `electric current * time`
    ///
    /// `electric capacitance * electric potential`
    pub fn is_electric_charge(&self) -> bool {
        if (self.unit_map == ELECTRIC_CHARGE_MAP && self.exp[ELECTRIC_CHARGE_INDEX] == 1)
            || (self.unit_map == ELECTRIC_CURRENT_MAP | TIME_MAP
                && self.exp[ELECTRIC_CURRENT_INDEX] == 1
                && self.exp[TIME_INDEX] == 1)
            || (self.unit_map == CAPACITANCE_MAP | ELECTRIC_POTENTIAL_MAP
                && self.exp[CAPACITANCE_INDEX] == 1
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of electric current
    ///
    /// `electric current`
    pub fn is_electric_current(&self) -> bool {
        if self.unit_map == ELECTRIC_CURRENT_MAP && self.exp[ELECTRIC_CURRENT_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of electric potential
    ///
    /// `electric potential`
    ///
    /// `power / electric current`
    ///
    /// `energy / electric charge`
    pub fn is_electric_potential(&self) -> bool {
        if (self.unit_map == ELECTRIC_POTENTIAL_MAP && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1)
            || (self.unit_map == POWER_MAP | ELECTRIC_CURRENT_MAP
                && self.exp[POWER_INDEX] == 1
                && self.exp[ELECTRIC_CURRENT_INDEX] == -1)
            || (self.unit_map == ENERGY_MAP | ELECTRIC_CHARGE_MAP
                && self.exp[ENERGY_INDEX] == 1
                && self.exp[ELECTRIC_CHARGE_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of electric capacitance
    ///
    /// `electric capacitance`
    ///
    /// `electric charge / electric potential`
    ///
    /// `electric charge^2 / energy`
    pub fn is_capacitance(&self) -> bool {
        if (self.unit_map == CAPACITANCE_MAP && self.exp[CAPACITANCE_INDEX] == 1)
            || (self.unit_map == ELECTRIC_CHARGE_MAP | ELECTRIC_POTENTIAL_MAP
                && self.exp[ELECTRIC_CHARGE_INDEX] == 1
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == -1)
            || (self.unit_map == ELECTRIC_CHARGE_MAP | ENERGY_MAP
                && self.exp[ELECTRIC_CHARGE_INDEX] == 2
                && self.exp[ENERGY_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of electric resistance
    ///
    /// `electric resistance`
    ///
    /// `1/electric conductance`
    ///
    /// `electric potential / electric current`
    pub fn is_resistance(&self) -> bool {
        if (self.unit_map == RESISTANCE_MAP && self.exp[RESISTANCE_INDEX] == 1)
            || (self.unit_map == ELECTRIC_CONDUCTANCE_MAP
                && self.exp[ELECTRIC_CONDUCTANCE_INDEX] == -1)
            || (self.unit_map == ELECTRIC_CURRENT_MAP | ELECTRIC_POTENTIAL_MAP
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1
                && self.exp[ELECTRIC_CURRENT_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of electric conductance
    ///
    /// `electric conductance`
    ///
    /// `1/electric resistance`
    ///
    /// `electric current / electric potential`
    pub fn is_conductance(&self) -> bool {
        if (self.unit_map == ELECTRIC_CONDUCTANCE_MAP && self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 1)
            || (self.unit_map == RESISTANCE_MAP && self.exp[RESISTANCE_INDEX] == -1)
            || (self.unit_map == ELECTRIC_CURRENT_MAP | ELECTRIC_POTENTIAL_MAP
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == -1
                && self.exp[ELECTRIC_CURRENT_INDEX] == 1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of magnetic flux
    ///
    /// `magnetic flux`
    ///
    /// `energy / electric current`
    ///
    /// `magnetic flux density * area`
    ///
    /// `electric potential * time`
    pub fn is_magnetic_flux(&self) -> bool {
        if (self.unit_map == MAGNETIC_FLUX_MAP && self.exp[MAGNETIC_FLUX_INDEX] == 1)
            || (self.unit_map == ENERGY_MAP | ELECTRIC_CURRENT_MAP
                && self.exp[ENERGY_INDEX] == 1
                && self.exp[ELECTRIC_CURRENT_INDEX] == -1)
            || (self.unit_map == MAGNETIC_FLUX_DENSITY_MAP | LENGTH_MAP
                && self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 1
                && self.exp[LENGTH_INDEX] == 2)
            || (self.unit_map == ELECTRIC_POTENTIAL_MAP | TIME_MAP
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1
                && self.exp[TIME_INDEX] == 1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of magnetic flux density
    ///
    /// `magnetic flux density`
    ///
    /// `electric potential * time / area`
    ///
    /// `magnetic flux / area`
    ///
    /// `force / (electric current * length)`
    pub fn is_magnetic_flux_density(&self) -> bool {
        if (self.unit_map == MAGNETIC_FLUX_DENSITY_MAP
            && self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 1)
            || (self.unit_map == ELECTRIC_POTENTIAL_MAP | TIME_MAP | LENGTH_MAP
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1
                && self.exp[TIME_INDEX] == 1
                && self.exp[LENGTH_INDEX] == -2)
            || (self.unit_map == MAGNETIC_FLUX_MAP | LENGTH_MAP
                && self.exp[MAGNETIC_FLUX_INDEX] == 1
                && self.exp[LENGTH_INDEX] == -2)
            || (self.unit_map == FORCE_MAP | ELECTRIC_CURRENT_MAP | LENGTH_MAP
                && self.exp[FORCE_INDEX] == 1
                && self.exp[ELECTRIC_CURRENT_INDEX] == -1
                && self.exp[LENGTH_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of electric inductance
    ///
    /// `inductance`
    ///
    /// `electric potential * time / electric current`
    ///
    /// `electric resistance * time`
    ///
    /// `magnetic flux / electric current`
    pub fn is_inductance(&self) -> bool {
        if (self.unit_map == INDUCTANCE_MAP && self.exp[INDUCTANCE_INDEX] == 1)
            || (self.unit_map == ELECTRIC_POTENTIAL_MAP | TIME_MAP | ELECTRIC_CURRENT_MAP
                && self.exp[ELECTRIC_POTENTIAL_INDEX] == 1
                && self.exp[TIME_INDEX] == 1
                && self.exp[ELECTRIC_CURRENT_INDEX] == -1)
            || (self.unit_map == RESISTANCE_MAP | TIME_MAP
                && self.exp[RESISTANCE_INDEX] == 1
                && self.exp[TIME_INDEX] == 1)
            || (self.unit_map == MAGNETIC_FLUX_MAP | ELECTRIC_CURRENT_MAP
                && self.exp[MAGNETIC_FLUX_INDEX] == 1
                && self.exp[ELECTRIC_CURRENT_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of luminous flux
    ///
    /// `luminous flux`
    ///
    /// `luminous intensity / solid angle`
    pub fn is_luminous_flux(&self) -> bool {
        if (self.unit_map == LUMINOUS_FLUX_MAP && self.exp[LUMINOUS_FLUX_INDEX] == 1)
            || (self.unit_map == LUMINOUS_INTENSITY_MAP | SOLID_ANGLE_MAP
                && self.exp[LUMINOUS_INTENSITY_INDEX] == 1
                && self.exp[SOLID_ANGLE_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of illuminance
    ///
    /// `illuminance`
    ///
    /// `luminous flux / area`
    pub fn is_illuminance(&self) -> bool {
        if (self.unit_map == ILLUMINANCE_MAP && self.exp[ILLUMINANCE_INDEX] == 1)
            || (self.unit_map == LUMINOUS_FLUX_MAP | LENGTH_MAP
                && self.exp[LUMINOUS_FLUX_INDEX] == 1
                && self.exp[LENGTH_INDEX] == -2)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of radioactivity
    ///
    /// `radioactivity`
    pub fn is_radioactivity(&self) -> bool {
        if self.unit_map == RADIOACTIVITY_MAP && self.exp[RADIOACTIVITY_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of absorbed dose of ionizing radiation
    ///
    /// `absorbed dose`
    pub fn is_absorbed_dose(&self) -> bool {
        if self.unit_map == ABSORBED_DOSE_MAP && self.exp[ABSORBED_DOSE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of an equivalent dose of ionizing radiation
    ///
    /// `equivalent dose`
    pub fn is_equivalent_dose(&self) -> bool {
        if self.unit_map == RADIOACTIVITY_EXPOSURE_MAP
            && self.exp[RADIOACTIVITY_EXPOSURE_INDEX] == 1
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of catalytic activity
    ///
    /// `catalytic activity`
    ///
    /// `substance / time`
    pub fn is_catalytic_activity(&self) -> bool {
        if (self.unit_map == CATALYTIC_ACTIVITY_MAP && self.exp[CATALYTIC_ACTIVITY_INDEX] == 1)
            || (self.unit_map == SUBSTANCE_MAP | TIME_MAP
                && self.exp[SUBSTANCE_INDEX] == 1
                && self.exp[TIME_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is an angle
    ///
    /// `angle`
    pub fn is_angle(&self) -> bool {
        if self.unit_map == ANGLE_MAP && self.exp[ANGLE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a solid angle
    ///
    /// `solid angle`
    pub fn is_solid_angle(&self) -> bool {
        if self.unit_map == SOLID_ANGLE_MAP && self.exp[SOLID_ANGLE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is an angle measured in radians
    ///
    /// `angle`
    pub fn is_radians(&self) -> bool {
        if self.unit_map == ANGLE_MAP
            && self.exp[ANGLE_INDEX] == 1
            && self.v_angle == Some(UnitAngle::Radian(Metric::None))
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of information
    ///
    /// `information`
    pub fn is_information(&self) -> bool {
        if self.unit_map == INFORMATION_MAP && self.exp[INFORMATION_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of luminous intensity
    ///
    /// `luminous intensity`
    pub fn is_luminous_intensity(&self) -> bool {
        if self.unit_map == LUMINOUS_INTENSITY_MAP && self.exp[LUMINOUS_INTENSITY_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of sound
    ///
    /// `sound`
    pub fn is_sound(&self) -> bool {
        if self.unit_map == SOUND_MAP && self.exp[SOUND_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of substance
    ///
    /// `substance`
    pub fn is_substance(&self) -> bool {
        if self.unit_map == SUBSTANCE_MAP && self.exp[SUBSTANCE_INDEX] == 1 {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a jerk
    ///
    /// `length / time^3`
    ///
    /// `school bully`
    pub fn is_jerk(&self) -> bool {
        if self.unit_map == LENGTH_MAP | TIME_MAP
            && self.exp[LENGTH_INDEX] == 1
            && self.exp[TIME_INDEX] == -3
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a snap
    ///
    /// `length / time^4`
    pub fn is_snap(&self) -> bool {
        if self.unit_map == LENGTH_MAP | TIME_MAP
            && self.exp[LENGTH_INDEX] == 1
            && self.exp[TIME_INDEX] == -4
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of angular velocity
    ///
    /// `angle / time`
    pub fn is_angular_velocity(&self) -> bool {
        if self.unit_map == ANGLE_MAP | TIME_MAP
            && self.exp[ANGLE_INDEX] == 1
            && self.exp[TIME_INDEX] == -1
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of angular acceleration
    ///
    /// `angle / time^2`
    pub fn is_angular_acceleration(&self) -> bool {
        if self.unit_map == ANGLE_MAP | TIME_MAP
            && self.exp[ANGLE_INDEX] == 1
            && self.exp[TIME_INDEX] == -2
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of frequency drift
    ///
    /// `frequency / time`
    pub fn is_frequency_drift(&self) -> bool {
        if self.unit_map == FREQUENCY_MAP | TIME_MAP
            && self.exp[FREQUENCY_INDEX] == 1
            && self.exp[TIME_INDEX] == -1
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of flow
    ///
    /// `volume / time`
    pub fn is_flow(&self) -> bool {
        if (self.unit_map == LENGTH_MAP | TIME_MAP
            && self.exp[LENGTH_INDEX] == 3
            && self.exp[TIME_INDEX] == -1)
            || (self.unit_map == VOLUME_MAP | TIME_MAP
                && self.exp[VOLUME_INDEX] == 1
                && self.exp[TIME_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a yank
    ///
    /// `force / time`
    pub fn is_yank(&self) -> bool {
        if self.unit_map == FORCE_MAP | TIME_MAP
            && self.exp[FORCE_INDEX] == 1
            && self.exp[TIME_INDEX] == -1
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of angular momentum
    ///
    /// `force * length * time`
    pub fn is_angular_momentum(&self) -> bool {
        if self.unit_map == FORCE_MAP | LENGTH_MAP | TIME_MAP
            && self.exp[FORCE_INDEX] == 1
            && self.exp[TIME_INDEX] == 1
            && self.exp[LENGTH_INDEX] == 1
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of torque
    ///
    /// `force * length`
    ///
    /// `energy / angle`
    pub fn is_torque(&self) -> bool {
        if (self.unit_map == FORCE_MAP | LENGTH_MAP
            && self.exp[FORCE_INDEX] == 1
            && self.exp[LENGTH_INDEX] == 1)
            || (self.unit_map == ENERGY_MAP | ANGLE_MAP
                && self.exp[ENERGY_INDEX] == 1
                && self.exp[ANGLE_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if a [`Value`] is a measurement of energy density
    ///
    /// `energy / volume`
    pub fn is_energy_density(&self) -> bool {
        if (self.unit_map == ENERGY_MAP | LENGTH_MAP
            && self.exp[ENERGY_INDEX] == 1
            && self.exp[LENGTH_INDEX] == -3)
            || (self.unit_map == ENERGY_MAP | VOLUME_MAP
                && self.exp[ENERGY_INDEX] == 1
                && self.exp[VOLUME_INDEX] == -1)
        {
            return true;
        }
        false
    }

    /// Returns `true` if two `Values` have comparable, not equal, unit types
    pub(in crate::values) fn __equivalent(&self, other: &Value) -> bool {
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

    /// Checks if the [`Value`] unit types are the same
    pub(in crate::values) fn __equal(&self, other: &Value) -> bool {
        if self.unit_map != other.unit_map {
            return false;
        }
        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                return false;
            }
            let region: usize = 1 << i;
            if region & self.unit_map != 0 {
                match region {
                    LENGTH_MAP => {
                        if self.v_length.unwrap() != other.v_length.unwrap() {
                            return false;
                        }
                    }
                    TIME_MAP => {
                        if self.v_time.unwrap() != other.v_time.unwrap() {
                            return false;
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
                        if self.v_electric_potential.unwrap() != other.v_electric_potential.unwrap()
                        {
                            return false;
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance.unwrap()
                            != other.v_electric_conductance.unwrap()
                        {
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
                        if self.v_magnetic_flux_density.unwrap()
                            != other.v_magnetic_flux_density.unwrap()
                        {
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
                        if self.v_luminous_flux_intensity.unwrap()
                            != other.v_luminous_flux_intensity.unwrap()
                        {
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
                        if self.v_radioactivity_exposure.unwrap()
                            != other.v_radioactivity_exposure.unwrap()
                        {
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

    /// Returns only the unit portion of the value as a string
    pub fn unit_string(&self) -> String {
        let mut nums: Vec<String> = vec![];
        let mut denoms: Vec<String> = vec![];

        for i in 0..31_usize {
            let region: usize = 1 << i;
            if self.unit_map & region == 0 {
                continue;
            }
            let u = match region {
                LENGTH_MAP => self.v_length.unwrap().to_string(),
                TIME_MAP => self.v_time.unwrap().to_string(),
                MASS_MAP => self.v_mass.unwrap().to_string(),
                ELECTRIC_CURRENT_MAP => self.v_electric_current.unwrap().to_string(),
                ELECTRIC_CHARGE_MAP => self.v_electric_charge.unwrap().to_string(),
                ELECTRIC_POTENTIAL_MAP => self.v_electric_potential.unwrap().to_string(),
                ELECTRIC_CONDUCTANCE_MAP => self.v_electric_conductance.unwrap().to_string(),
                CAPACITANCE_MAP => self.v_capacitance.unwrap().to_string(),
                RESISTANCE_MAP => self.v_resistance.unwrap().to_string(),
                INDUCTANCE_MAP => self.v_inductance.unwrap().to_string(),
                MAGNETIC_FLUX_MAP => self.v_magnetic_flux.unwrap().to_string(),
                MAGNETIC_FLUX_DENSITY_MAP => self.v_magnetic_flux_density.unwrap().to_string(),
                TEMPERATURE_MAP => self.v_temperature.unwrap().to_string(),
                SUBSTANCE_MAP => self.v_substance.unwrap().to_string(),
                LUMINOUS_INTENSITY_MAP => self.v_luminous_flux_intensity.unwrap().to_string(),
                LUMINOUS_FLUX_MAP => self.v_luminous_flux.unwrap().to_string(),
                ILLUMINANCE_MAP => self.v_illuminance.unwrap().to_string(),
                VOLUME_MAP => self.v_volume.unwrap().to_string(),
                PRESSURE_MAP => self.v_pressure.unwrap().to_string(),
                ANGLE_MAP => self.v_angle.unwrap().to_string(),
                FREQUENCY_MAP => self.v_frequency.unwrap().to_string(),
                FORCE_MAP => self.v_force.unwrap().to_string(),
                ENERGY_MAP => self.v_energy.unwrap().to_string(),
                POWER_MAP => self.v_power.unwrap().to_string(),
                RADIOACTIVITY_MAP => self.v_radioactivity.unwrap().to_string(),
                ABSORBED_DOSE_MAP => self.v_ab_dose.unwrap().to_string(),
                RADIOACTIVITY_EXPOSURE_MAP => self.v_radioactivity_exposure.unwrap().to_string(),
                CATALYTIC_ACTIVITY_MAP => self.v_catalytic.unwrap().to_string(),
                SOUND_MAP => self.v_sound.unwrap().to_string(),
                INFORMATION_MAP => self.v_information.unwrap().to_string(),
                SOLID_ANGLE_MAP => self.v_solid_angle.unwrap().to_string(),
                _ => String::from(""),
            };
            if self.exp[i] < -1 {
                denoms.push(format!("{}^{}", u, self.exp[i]))
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

        let final_str: String;

        if !final_num.is_empty() && !final_denom.is_empty() {
            final_str = format!("{}/{}", final_num, final_denom);
        } else if !final_num.is_empty() && final_denom.is_empty() {
            final_str = final_num;
        } else if final_num.is_empty() && !final_denom.is_empty() {
            final_str = format!("1/{}", final_denom);
        } else {
            final_str = String::from("");
        }

        final_str
    }
}
