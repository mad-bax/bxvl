use crate::{
    consts::{
        CAPACITANCE_INDEX, CAPACITANCE_MAP, CATALYTIC_ACTIVITY_INDEX, CATALYTIC_ACTIVITY_MAP,
        ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP, ELECTRIC_CONDUCTANCE_INDEX,
        ELECTRIC_CONDUCTANCE_MAP, ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP,
        ELECTRIC_POTENTIAL_INDEX, ELECTRIC_POTENTIAL_MAP, ENERGY_INDEX, ENERGY_MAP, FORCE_INDEX,
        FORCE_MAP, FREQUENCY_INDEX, FREQUENCY_MAP, ILLUMINANCE_INDEX, ILLUMINANCE_MAP,
        INDUCTANCE_INDEX, INDUCTANCE_MAP, LENGTH_INDEX, LENGTH_MAP, LUMINOUS_FLUX_INDEX,
        LUMINOUS_FLUX_MAP, LUMINOUS_INTENSITY_INDEX, LUMINOUS_INTENSITY_MAP,
        MAGNETIC_FLUX_DENSITY_INDEX, MAGNETIC_FLUX_DENSITY_MAP, MAGNETIC_FLUX_INDEX,
        MAGNETIC_FLUX_MAP, MASS_INDEX, MASS_MAP, POWER_INDEX, POWER_MAP, PRESSURE_INDEX,
        PRESSURE_MAP, RESISTANCE_INDEX, RESISTANCE_MAP, SOLID_ANGLE_INDEX, SOLID_ANGLE_MAP,
        SUBSTANCE_INDEX, SUBSTANCE_MAP, TIME_INDEX, TIME_MAP,
    },
    errors::V3Error,
    units::{
        Metric, UnitCatalyticActivity, UnitElectricCapacitance, UnitElectricCharge,
        UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
        UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency,
        UnitIlluminance, UnitLength, UnitLuminousFlux, UnitLuminousIntensity, UnitMagneticFlux,
        UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure, UnitSolidAngle, UnitSubstance,
        UnitTime,
    },
    value::Value,
};

impl Value {
    /// Reduces a [`Value`]'s unit complexity.
    ///
    /// When a [`Value`] has a specific type that is composed from base units such as `Newtons`;
    /// it can be reduced to those base units.
    ///
    /// # Example
    /// ```rust
    /// use bxvl::{value::Value, units::{Metric, UnitForce}};
    ///
    /// let mut f:Value = 3.0 * UnitForce::Newton(Metric::None);
    ///
    /// match f.reduce("kg*m/s^2") {
    ///     Ok(_) => {}
    ///     Err(e) => panic!("{}", e)
    /// };
    ///
    /// assert_eq!(f.to_string(), "3 m*kg/s^2");
    /// ```
    pub fn reduce(&mut self, other: &str) -> Result<(), V3Error> {
        if !self.is_reducible() {
            return Err(V3Error::UnitReductionError(format!(
                "[reduce] Value {self} is not reducible"
            )));
        }
        let temp: Value = Value::new(1.0, other)?;
        if self._reduce(&temp) {
            return Ok(());
        }
        Err(V3Error::UnitReductionError(format!(
            "[reduce] Value {self} cannot be reduced to {other}"
        )))
    }

    /// Creates a new [`Value`] with reduced unit complexity.
    ///
    /// When a [`Value`] has a specific type that is composed from base units such as `Newtons`;
    /// it can be reduced to those base units.
    ///
    /// # Example
    /// ```rust
    /// use bxvl::{value::Value, units::{Metric, UnitForce}};
    ///
    /// let f:Value = 3.0 * UnitForce::Newton(Metric::None);
    ///
    /// let f_new = match f.reduction("kg*m/s^2") {
    ///     Ok(v) => v,
    ///     Err(e) => panic!("{}", e)
    /// };
    ///
    /// assert_eq!(f_new.to_string(), "3 m*kg/s^2");
    /// ```
    pub fn reduction(&self, other: &str) -> Result<Value, V3Error> {
        if !self.is_reducible() {
            return Err(V3Error::UnitReductionError(format!(
                "[reduce] Value {self} is not reducible"
            )));
        }

        let mut ret = *self;

        let temp: Value = Value::new(1.0, other)?;
        if ret._reduce(&temp) {
            return Ok(ret);
        }
        Err(V3Error::UnitReductionError(format!(
            "[reduce] Value {self} cannot be reduced to {other}"
        )))
    }

    /// Returns if a [`Value`]'s unit type is reducible.
    pub fn is_reducible(&self) -> bool {
        matches!(
            self.unit_map,
            FORCE_MAP
                | PRESSURE_MAP
                | ENERGY_MAP
                | FREQUENCY_MAP
                | POWER_MAP
                | ELECTRIC_CHARGE_MAP
                | ELECTRIC_POTENTIAL_MAP
                | RESISTANCE_MAP
                | ELECTRIC_CONDUCTANCE_MAP
                | MAGNETIC_FLUX_MAP
                | MAGNETIC_FLUX_DENSITY_MAP
                | INDUCTANCE_MAP
                | LUMINOUS_FLUX_MAP
                | ILLUMINANCE_MAP
                | CAPACITANCE_MAP
                | CATALYTIC_ACTIVITY_MAP
        )
    }

    /// Reduces the [`UnitPressure`] in a given [`Value`]
    ///
    /// When a [`Value`] has a specific type that is composed from base units such as `Pascals`;
    /// it can be reduced to those base units.
    ///
    /// # Example
    /// ```rust
    /// use bxvl::{value::Value, units::{Metric, UnitPressure}};
    ///
    /// let f:Value = 3.0 * UnitPressure::Pascal(Metric::None);
    ///
    /// let f_new = match f.reduction("kg/(m*s^2)") {
    ///     Ok(v) => v,
    ///     Err(e) => panic!("{}", e),
    /// };
    ///
    /// assert_eq!(f_new.to_string(), "3 kg/m*s^2");
    /// ```
    pub fn reduce_pressure(&self) -> Option<Value> {
        if self.unit_map & PRESSURE_MAP == 0 {
            return None;
        }

        let mut temp = 1.0 * UnitMass::Gram(Metric::Kilo)
            / UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[PRESSURE_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[PRESSURE_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[PRESSURE_INDEX];

        let mut ret = (*self >> UnitPressure::Pascal(Metric::None)).unwrap();

        ret.v_pressure = None;
        ret.exp[PRESSURE_INDEX] = 0;
        ret.unit_map &= !PRESSURE_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitEnergy`] in a given [`Value`]
    pub fn reduce_energy(&self) -> Option<Value> {
        if self.unit_map & ENERGY_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            * UnitMass::Gram(Metric::Kilo)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[ENERGY_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[ENERGY_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[ENERGY_INDEX];

        let mut ret = (*self >> UnitEnergy::Joule(Metric::None)).unwrap();

        ret.v_energy = None;
        ret.exp[ENERGY_INDEX] = 0;
        ret.unit_map &= !ENERGY_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitForce`] in a given [`Value`]
    pub fn reduce_force(&self) -> Option<Value> {
        if self.unit_map & FORCE_MAP == 0 {
            return None;
        }

        let mut temp = 1.0 * UnitMass::Gram(Metric::Kilo) * UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[FORCE_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[FORCE_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[FORCE_INDEX];

        let mut ret = (*self >> UnitForce::Newton(Metric::None)).unwrap();

        ret.v_force = None;
        ret.exp[FORCE_INDEX] = 0;
        ret.unit_map &= !FORCE_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitPower`] in a given [`Value`]
    pub fn reduce_power(&self) -> Option<Value> {
        if self.unit_map & POWER_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            * UnitMass::Gram(Metric::Kilo)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[POWER_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[POWER_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[POWER_INDEX];

        let mut ret = (*self >> UnitPower::Watt(Metric::None)).unwrap();

        ret.v_power = None;
        ret.exp[POWER_INDEX] = 0;
        ret.unit_map &= !POWER_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitElectricCharge`] in a given [`Value`]
    pub fn reduce_electric_charge(&self) -> Option<Value> {
        if self.unit_map & ELECTRIC_CHARGE_MAP == 0 {
            return None;
        }

        let mut temp =
            1.0 * UnitTime::Second(Metric::None) * UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[ELECTRIC_CHARGE_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[ELECTRIC_CHARGE_INDEX];

        let mut ret = (*self >> UnitElectricCharge::Coulomb(Metric::None)).unwrap();

        ret.v_electric_charge = None;
        ret.exp[ELECTRIC_CHARGE_INDEX] = 0;
        ret.unit_map &= !ELECTRIC_CHARGE_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitElectricPotential`] in a given [`Value`]
    pub fn reduce_electric_potential(&self) -> Option<Value> {
        if self.unit_map & ELECTRIC_POTENTIAL_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            * UnitMass::Gram(Metric::Kilo)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[ELECTRIC_POTENTIAL_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[ELECTRIC_POTENTIAL_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[ELECTRIC_POTENTIAL_INDEX];
        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[ELECTRIC_POTENTIAL_INDEX];

        let mut ret = (*self >> UnitElectricPotential::Volt(Metric::None)).unwrap();

        ret.v_electric_potential = None;
        ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
        ret.unit_map &= !ELECTRIC_POTENTIAL_INDEX;

        Some(ret * temp)
    }

    /// Reduces the [`UnitElectricCapacitance`] in a given [`Value`]
    pub fn reduce_electric_capacitance(&self) -> Option<Value> {
        if self.unit_map & CAPACITANCE_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            / UnitMass::Gram(Metric::Kilo)
            / UnitLength::Meter(Metric::None)
            / UnitLength::Meter(Metric::None)
            * UnitTime::Second(Metric::None)
            * UnitTime::Second(Metric::None)
            * UnitTime::Second(Metric::None)
            * UnitTime::Second(Metric::None)
            * UnitElectricCurrent::Ampere(Metric::None)
            * UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[CAPACITANCE_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[CAPACITANCE_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[CAPACITANCE_INDEX];
        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[CAPACITANCE_INDEX];

        let mut ret = (*self >> UnitElectricCapacitance::Farad(Metric::None)).unwrap();

        ret.v_capacitance = None;
        ret.exp[CAPACITANCE_INDEX] = 0;
        ret.unit_map &= !CAPACITANCE_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitElectricResistance`] in a given [`Value`]
    pub fn reduce_electric_resistance(&self) -> Option<Value> {
        if self.unit_map & RESISTANCE_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            * UnitMass::Gram(Metric::Kilo)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[RESISTANCE_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[RESISTANCE_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[RESISTANCE_INDEX];
        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[RESISTANCE_INDEX];

        let mut ret = (*self >> UnitElectricResistance::Ohm(Metric::None)).unwrap();

        ret.v_resistance = None;
        ret.exp[RESISTANCE_INDEX] = 0;
        ret.unit_map &= !RESISTANCE_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitElectricConductance`] in a given [`Value`]
    pub fn reduce_electric_conductance(&self) -> Option<Value> {
        if self.unit_map & ELECTRIC_CONDUCTANCE_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            / UnitMass::Gram(Metric::Kilo)
            / UnitLength::Meter(Metric::None)
            / UnitLength::Meter(Metric::None)
            * UnitTime::Second(Metric::None)
            * UnitTime::Second(Metric::None)
            * UnitTime::Second(Metric::None)
            * UnitElectricCurrent::Ampere(Metric::None)
            * UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[ELECTRIC_CONDUCTANCE_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[ELECTRIC_CONDUCTANCE_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[ELECTRIC_CONDUCTANCE_INDEX];
        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[ELECTRIC_CONDUCTANCE_INDEX];

        let mut ret = (*self >> UnitElectricConductance::Siemens(Metric::None)).unwrap();

        ret.v_electric_conductance = None;
        ret.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
        ret.unit_map &= !ELECTRIC_CONDUCTANCE_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitMagneticFlux`] in a given [`Value`]
    pub fn reduce_magnetic_flux(&self) -> Option<Value> {
        if self.unit_map & MAGNETIC_FLUX_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            * UnitMass::Gram(Metric::Kilo)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[MAGNETIC_FLUX_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[MAGNETIC_FLUX_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[MAGNETIC_FLUX_INDEX];
        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[MAGNETIC_FLUX_INDEX];

        let mut ret = (*self >> UnitMagneticFlux::Weber(Metric::None)).unwrap();

        ret.v_magnetic_flux = None;
        ret.exp[MAGNETIC_FLUX_INDEX] = 0;
        ret.unit_map &= !MAGNETIC_FLUX_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitMagneticFluxDensity`] in a given [`Value`]
    pub fn reduce_magnetic_flux_density(&self) -> Option<Value> {
        if self.unit_map & MAGNETIC_FLUX_DENSITY_MAP == 0 {
            return None;
        }

        let mut temp = 1.0 * UnitMass::Gram(Metric::Kilo)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[MAGNETIC_FLUX_DENSITY_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[MAGNETIC_FLUX_DENSITY_INDEX];
        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[MAGNETIC_FLUX_DENSITY_INDEX];

        let mut ret = (*self >> UnitMagneticFluxDensity::Tesla(Metric::None)).unwrap();

        ret.v_magnetic_flux_density = None;
        ret.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
        ret.unit_map &= !MAGNETIC_FLUX_DENSITY_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitElectricInductance`] in a given [`Value`]
    pub fn reduce_electric_inductance(&self) -> Option<Value> {
        if self.unit_map & INDUCTANCE_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            * UnitMass::Gram(Metric::Kilo)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None);

        temp.exp[MASS_INDEX] *= self.exp[INDUCTANCE_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[INDUCTANCE_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[INDUCTANCE_INDEX];
        temp.exp[ELECTRIC_CURRENT_INDEX] *= self.exp[INDUCTANCE_INDEX];

        let mut ret = (*self >> UnitElectricInductance::Henry(Metric::None)).unwrap();

        ret.v_inductance = None;
        ret.exp[INDUCTANCE_INDEX] = 0;
        ret.unit_map &= !INDUCTANCE_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitCatalyticActivity`] in a given [`Value`]
    pub fn reduce_catalytic_activity(&self) -> Option<Value> {
        if self.unit_map & CATALYTIC_ACTIVITY_MAP == 0 {
            return None;
        }

        let mut temp = 1.0 * UnitSubstance::Mole(Metric::None) / UnitTime::Second(Metric::None);

        temp.exp[SUBSTANCE_INDEX] *= self.exp[CATALYTIC_ACTIVITY_INDEX];
        temp.exp[TIME_INDEX] *= self.exp[CATALYTIC_ACTIVITY_INDEX];

        let mut ret = (*self >> UnitCatalyticActivity::Katal(Metric::None)).unwrap();

        ret.v_catalytic = None;
        ret.exp[CATALYTIC_ACTIVITY_INDEX] = 0;
        ret.unit_map &= !CATALYTIC_ACTIVITY_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitLuminousFlux`] in a given [`Value`]
    pub fn reduce_luminous_flux(&self) -> Option<Value> {
        if self.unit_map & LUMINOUS_FLUX_MAP == 0 {
            return None;
        }

        let mut temp = 1.0 * UnitLuminousIntensity::Candela(Metric::None)
            / UnitSolidAngle::Steradian(Metric::None);

        temp.exp[LUMINOUS_INTENSITY_INDEX] *= self.exp[LUMINOUS_FLUX_INDEX];
        temp.exp[SOLID_ANGLE_INDEX] *= self.exp[LUMINOUS_FLUX_INDEX];

        let mut ret = (*self >> UnitLuminousFlux::Lumen(Metric::None)).unwrap();

        ret.v_luminous_flux = None;
        ret.exp[LUMINOUS_FLUX_INDEX] = 0;
        ret.unit_map &= !LUMINOUS_FLUX_MAP;

        Some(ret * temp)
    }

    /// Reduces the [`UnitIlluminance`] in a given [`Value`]
    pub fn reduce_illuminance(&self) -> Option<Value> {
        if self.unit_map & ILLUMINANCE_MAP == 0 {
            return None;
        }

        let mut temp = 1.0
            * UnitLuminousIntensity::Candela(Metric::None)
            * UnitSolidAngle::Steradian(Metric::None)
            / UnitLength::Meter(Metric::None)
            / UnitLength::Meter(Metric::None);

        temp.exp[LUMINOUS_INTENSITY_INDEX] *= self.exp[ILLUMINANCE_INDEX];
        temp.exp[LENGTH_INDEX] *= self.exp[ILLUMINANCE_INDEX];
        temp.exp[SOLID_ANGLE_INDEX] *= self.exp[ILLUMINANCE_INDEX];

        let mut ret = (*self >> UnitIlluminance::Lux(Metric::None)).unwrap();

        ret.v_illuminance = None;
        ret.exp[ILLUMINANCE_INDEX] = 0;
        ret.unit_map &= !ILLUMINANCE_MAP;

        Some(ret * temp)
    }

    /// Actual reduce function that operates on a [`Value`] type
    fn _reduce(&mut self, other: &Value) -> bool {
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
            if other.unit_map & FORCE_MAP > 0 {
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
            } else {
                *self >>= UnitPressure::Pascal(Metric::None);
                self.v_mass = Some(UnitMass::Gram(Metric::Kilo));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_pressure = None;

                self.exp[TIME_INDEX] = -2;
                self.exp[LENGTH_INDEX] = -1;
                self.exp[MASS_INDEX] = 1;
                self.exp[PRESSURE_INDEX] = 0;
                self.unit_map = TIME_MAP | MASS_MAP | LENGTH_MAP;
                *self >>= *other;
                return true;
            }
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
                self.exp[ELECTRIC_CHARGE_INDEX] = 1;
                self.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
                self.exp[ENERGY_INDEX] = 0;
                self.unit_map = ELECTRIC_CHARGE_MAP | ELECTRIC_POTENTIAL_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & POWER_MAP > 0 {
                *self >>= UnitEnergy::Joule(Metric::None);
                self.v_power = Some(UnitPower::Watt(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_energy = None;
                self.exp[POWER_INDEX] = 1;
                self.exp[TIME_INDEX] = 1;
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
                self.exp[ENERGY_INDEX] = 0;
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
                self.exp[POWER_INDEX] = 0;
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
            } else if other.unit_map & CAPACITANCE_MAP > 0 {
                *self >>= UnitElectricCharge::Coulomb(Metric::None);
                self.v_capacitance = Some(UnitElectricCapacitance::Farad(Metric::None));
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
                *self >>= UnitElectricCapacitance::Farad(Metric::None);
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
                *self >>= UnitElectricCapacitance::Farad(Metric::None);
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
                *self >>= UnitElectricResistance::Ohm(Metric::None);
                self.v_electric_conductance = Some(UnitElectricConductance::Siemens(Metric::None));
                self.v_resistance = None;
                self.exp[ELECTRIC_CONDUCTANCE_INDEX] = -1;
                self.exp[RESISTANCE_INDEX] = 0;
                self.unit_map = ELECTRIC_CONDUCTANCE_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & ELECTRIC_CURRENT_MAP > 0 {
                *self >>= UnitElectricResistance::Ohm(Metric::None);
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
                self.v_resistance = Some(UnitElectricResistance::Ohm(Metric::None));
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
                self.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
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
                self.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
                self.unit_map = FORCE_MAP | ELECTRIC_CURRENT_MAP | LENGTH_MAP;
                *self >>= *other;
                return true;
            }
        } else if self.unit_map == INDUCTANCE_MAP && other.is_inductance() {
            if other.unit_map & ELECTRIC_POTENTIAL_MAP > 0 {
                *self >>= UnitElectricInductance::Henry(Metric::None);
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
                *self >>= UnitElectricInductance::Henry(Metric::None);
                self.v_inductance = None;
                self.v_resistance = Some(UnitElectricResistance::Ohm(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.exp[RESISTANCE_INDEX] = 1;
                self.exp[TIME_INDEX] = 1;
                self.exp[INDUCTANCE_INDEX] = 0;
                self.unit_map = RESISTANCE_MAP | TIME_MAP;
                *self >>= *other;
                return true;
            } else if other.unit_map & MAGNETIC_FLUX_MAP > 0 {
                *self >>= UnitElectricInductance::Henry(Metric::None);
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
        } else if self.unit_map == LUMINOUS_FLUX_MAP && other.is_luminous_flux() {
            *self >>= UnitLuminousFlux::Lumen(Metric::None);
            self.v_luminous_flux_intensity = Some(UnitLuminousIntensity::Candela(Metric::None));
            self.v_solid_angle = Some(UnitSolidAngle::Steradian(Metric::None));
            self.v_luminous_flux = None;
            self.exp[LUMINOUS_FLUX_INDEX] = 0;
            self.exp[LUMINOUS_INTENSITY_INDEX] = 1;
            self.exp[SOLID_ANGLE_INDEX] = -1;
            self.unit_map = LUMINOUS_INTENSITY_MAP | SOLID_ANGLE_MAP;
            *self >>= *other;
            return true;
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
}

#[cfg(test)]
mod reduction_testing {
    use crate::units::{
        Metric, UnitCatalyticActivity, UnitElectricCapacitance, UnitElectricCharge,
        UnitElectricConductance, UnitElectricInductance, UnitElectricPotential,
        UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency, UnitIlluminance, UnitLength,
        UnitLuminousFlux, UnitMagneticFlux, UnitMagneticFluxDensity, UnitPower, UnitPressure,
    };

    #[test]
    fn reduce_catalytic_activity() {
        let mut t1 = 4.0 * UnitCatalyticActivity::Katal(Metric::None);
        t1.reduce("mol/s").unwrap();
        assert_eq!(t1.to_string(), "4 mol/s");
        assert!(t1.is_catalytic_activity());
    }

    #[test]
    fn reduce_illuminance() {
        let mut t1 = 4.0 * UnitIlluminance::Lux(Metric::None);
        t1.reduce("lm/m^2").unwrap();
        assert_eq!(t1.to_string(), "4 lm/m^2");
        assert!(t1.is_illuminance());
    }

    #[test]
    fn reduce_luminous_flux() {
        let mut t1 = 4.0 * UnitLuminousFlux::Lumen(Metric::None);
        t1.reduce("cd/sr").unwrap();
        assert_eq!(t1.to_string(), "4 cd/sr");
        assert!(t1.is_luminous_flux());
    }

    #[test]
    fn reduce_inductance() {
        let mut t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        t1.reduce("V*s/A").unwrap();
        assert_eq!(t1.to_string(), "4 s*V/A");
        assert!(t1.is_inductance());

        let mut t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        t1.reduce("s*O").unwrap();
        assert_eq!(t1.to_string(), "4 s*Ω");
        assert!(t1.is_inductance());

        let mut t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        t1.reduce("Wb/A").unwrap();
        assert_eq!(t1.to_string(), "4 Wb/A");
        assert!(t1.is_inductance());
    }

    #[test]
    fn reduce_magnetic_flux_density() {
        let mut t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        t1.reduce("V*s/m^2").unwrap();
        assert_eq!(t1.to_string(), "4 s*V/m^2");
        assert!(t1.is_magnetic_flux_density());

        let mut t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        t1.reduce("Wb/m^2").unwrap();
        assert_eq!(t1.to_string(), "4 Wb/m^2");
        assert!(t1.is_magnetic_flux_density());

        let mut t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        t1.reduce("N/A*m").unwrap();
        assert_eq!(t1.to_string(), "4 N/m*A");
        assert!(t1.is_magnetic_flux_density());
    }

    #[test]
    fn reduce_magnetic_flux() {
        let mut t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        t1.reduce("J/A").unwrap();
        assert_eq!(t1.to_string(), "4 J/A");
        assert!(t1.is_magnetic_flux());

        let mut t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        t1.reduce("T*m^2").unwrap();
        assert_eq!(t1.to_string(), "4 m^2*T");
        assert!(t1.is_magnetic_flux());

        let mut t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        t1.reduce("s*V").unwrap();
        assert_eq!(t1.to_string(), "4 s*V");
        assert!(t1.is_magnetic_flux());
    }

    #[test]
    fn reduce_conductance() {
        let mut t1 = 4.0 * UnitElectricConductance::Siemens(Metric::None);
        t1.reduce("1/Ω").unwrap();
        assert_eq!(t1.to_string(), "4 1/Ω");
        assert!(t1.is_conductance());

        let mut t1 = 4.0 * UnitElectricConductance::Siemens(Metric::None);
        t1.reduce("A/V").unwrap();
        assert_eq!(t1.to_string(), "4 A/V");
        assert!(t1.is_conductance());
    }

    #[test]
    fn reduce_resistance() {
        let mut t1 = 4.0 * UnitElectricResistance::Ohm(Metric::None);
        t1.reduce("1/S").unwrap();
        assert_eq!(t1.to_string(), "4 1/S");
        assert!(t1.is_resistance());

        let mut t1 = 4.0 * UnitElectricResistance::Ohm(Metric::None);
        t1.reduce("V/A").unwrap();
        assert_eq!(t1.to_string(), "4 V/A");
        assert!(t1.is_resistance());
    }

    #[test]
    fn reduce_capacitance() {
        let mut t1 = 4.0 * UnitElectricCapacitance::Farad(Metric::None);
        t1.reduce("C/V").unwrap();
        assert_eq!(t1.to_string(), "4 C/V");
        assert!(t1.is_capacitance());

        let mut t1 = 4.0 * UnitElectricCapacitance::Farad(Metric::None);
        t1.reduce("C^2/J").unwrap();
        assert_eq!(t1.to_string(), "4 C^2/J");
        assert!(t1.is_capacitance());
    }

    #[test]
    fn reduce_electric_potential() {
        let mut t1 = 4.0 * UnitElectricPotential::Volt(Metric::None);
        t1.reduce("W/A").unwrap();
        assert_eq!(t1.to_string(), "4 W/A");
        assert!(t1.is_electric_potential());

        let mut t1 = 4.0 * UnitElectricPotential::Volt(Metric::None);
        t1.reduce("J/C").unwrap();
        assert_eq!(t1.to_string(), "4 J/C");
        assert!(t1.is_electric_potential());
    }

    #[test]
    fn reduce_electric_charge() {
        let mut t1 = 4.0 * UnitElectricCharge::Coulomb(Metric::None);
        t1.reduce("A*s").unwrap();
        assert_eq!(t1.to_string(), "4 s*A");
        assert!(t1.is_electric_charge());

        let mut t1 = 4.0 * UnitElectricCharge::Coulomb(Metric::None);
        t1.reduce("F*V").unwrap();
        assert_eq!(t1.to_string(), "4 V*F");
        assert!(t1.is_electric_charge());
    }

    #[test]
    fn reduce_power() {
        let mut t1 = 4.0 * UnitPower::Watt(Metric::None);
        t1.reduce("J/s").unwrap();
        assert_eq!(t1.to_string(), "4 J/s");
        assert!(t1.is_power());

        let mut t1 = 4.0 * UnitPower::Watt(Metric::None);
        t1.reduce("V*A").unwrap();
        assert_eq!(t1.to_string(), "4 A*V");
        assert!(t1.is_power());

        let mut t1 = 4.0 * UnitPower::Watt(Metric::None);
        t1.reduce("m^2*kg/s^3").unwrap();
        assert_eq!(t1.to_string(), "4 m^2*kg/s^3");
        assert!(t1.is_power());
    }

    #[test]
    fn reduce_frequency() {
        let mut t1 = 4.0 * UnitFrequency::Hertz(Metric::None);
        t1.reduce("s^-1").unwrap();
        assert_eq!(t1.to_string(), "4 1/s");
        assert!(t1.is_frequency());
    }

    #[test]
    fn reduce_energy() {
        let mut t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        t1.reduce("N*m").unwrap();
        assert_eq!(t1.to_string(), "4 m*N");
        assert!(t1.is_energy());

        let mut t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        t1.reduce("V*C").unwrap();
        assert_eq!(t1.to_string(), "4 C*V");
        assert!(t1.is_energy());

        let mut t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        t1.reduce("W*s").unwrap();
        assert_eq!(t1.to_string(), "4 s*W");
        assert!(t1.is_energy());

        let mut t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        t1.reduce("m^2*kg/s^2").unwrap();
        assert_eq!(t1.to_string(), "4 m^2*kg/s^2");
        assert!(t1.is_energy());
    }

    #[test]
    fn reduce_force() {
        let mut t1 = 4.0 * UnitForce::Newton(Metric::None);
        t1.reduce("g*m/s^2").unwrap();
        assert_eq!(t1.to_string(), "4000 m*g/s^2");
    }

    #[test]
    fn reduce_pressure() {
        let mut t1 = 1.0 * UnitPressure::Bar(Metric::None);
        t1.reduce("N/m^2").unwrap();
        assert_eq!(t1.to_string(), "100000 N/m^2");
    }

    #[test]
    fn is_not_reducible() {
        let t1 = 4.0 * UnitLength::Meter(Metric::None);
        let t2 = 4.0 * UnitForce::Newton(Metric::None);

        assert!(!t1.is_reducible());
        assert!(t2.is_reducible());
    }

    #[test]
    #[should_panic]
    fn reduce_bad() {
        let mut t1 = 4.0 * UnitLength::Meter(Metric::None);
        t1.reduce("s").unwrap();
    }

    #[test]
    #[should_panic]
    fn reduce_bad_text() {
        let mut t1 = 4.0 * UnitForce::Newton(Metric::None);
        t1.reduce("zz").unwrap();
    }

    #[test]
    #[should_panic]
    fn reduce_bad_map() {
        let mut t1 = 4.0 * UnitForce::Newton(Metric::None);
        t1.reduce("N").unwrap();
    }

    #[test]
    #[should_panic]
    fn reduce_bad_no_match() {
        let mut t1 = 4.0 * UnitForce::Newton(Metric::None);
        t1.reduce("g*m/s").unwrap();
    }

    #[test]
    fn reduction_catalytic_activity() {
        let t1 = 4.0 * UnitCatalyticActivity::Katal(Metric::None);
        let t2 = t1.reduction("mol/s").unwrap();
        assert_eq!(t2.to_string(), "4 mol/s");
        assert!(t2.is_catalytic_activity());

        let t3 = (4.0 * UnitCatalyticActivity::Katal(Metric::None))
            .reduce_catalytic_activity()
            .unwrap();
        assert_eq!(t3.to_string(), "4 mol/s");
        assert!(t3.is_catalytic_activity());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_catalytic_activity()
                .is_none()
        );
    }

    #[test]
    fn reduction_illuminance() {
        let t1 = 4.0 * UnitIlluminance::Lux(Metric::None);
        let t2 = t1.reduction("lm/m^2").unwrap();
        assert_eq!(t2.to_string(), "4 lm/m^2");
        assert!(t2.is_illuminance());

        let t3 = (4.0 * UnitIlluminance::Lux(Metric::None))
            .reduce_illuminance()
            .unwrap();
        assert_eq!(t3.to_string(), "4 cd*sr/m^2");
        assert!(t3.is_illuminance());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_illuminance()
                .is_none()
        );
    }

    #[test]
    fn reduction_luminous_flux() {
        let t1 = 4.0 * UnitLuminousFlux::Lumen(Metric::None);
        let t2 = t1.reduction("cd/sr").unwrap();
        assert_eq!(t2.to_string(), "4 cd/sr");
        assert!(t2.is_luminous_flux());

        let t3 = (4.0 * UnitLuminousFlux::Lumen(Metric::None))
            .reduce_luminous_flux()
            .unwrap();
        assert_eq!(t3.to_string(), "4 cd/sr");
        assert!(t3.is_luminous_flux());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_luminous_flux()
                .is_none()
        );
    }

    #[test]
    fn reduction_inductance() {
        let t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        let t2 = t1.reduction("V*s/A").unwrap();
        assert_eq!(t2.to_string(), "4 s*V/A");
        assert!(t2.is_inductance());

        let t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        let t2 = t1.reduction("s*O").unwrap();
        assert_eq!(t2.to_string(), "4 s*Ω");
        assert!(t2.is_inductance());

        let t1 = 4.0 * UnitElectricInductance::Henry(Metric::None);
        let t2 = t1.reduction("Wb/A").unwrap();
        assert_eq!(t2.to_string(), "4 Wb/A");
        assert!(t2.is_inductance());

        let t3 = (4.0 * UnitElectricInductance::Henry(Metric::None))
            .reduce_electric_inductance()
            .unwrap();
        assert_eq!(t3.to_string(), "4 m^2*kg/s^2*A^2");
        assert!(t3.is_inductance());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_electric_inductance()
                .is_none()
        );
    }

    #[test]
    fn reduction_magnetic_flux_density() {
        let t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t2 = t1.reduction("V*s/m^2").unwrap();
        assert_eq!(t2.to_string(), "4 s*V/m^2");
        assert!(t2.is_magnetic_flux_density());

        let t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t2 = t1.reduction("Wb/m^2").unwrap();
        assert_eq!(t2.to_string(), "4 Wb/m^2");
        assert!(t2.is_magnetic_flux_density());

        let t1 = 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t2 = t1.reduction("N/A*m").unwrap();
        assert_eq!(t2.to_string(), "4 N/m*A");
        assert!(t2.is_magnetic_flux_density());

        let t3 = (4.0 * UnitMagneticFluxDensity::Tesla(Metric::None))
            .reduce_magnetic_flux_density()
            .unwrap();
        assert_eq!(t3.to_string(), "4 kg/s^2*A");
        assert!(t3.is_magnetic_flux_density());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_magnetic_flux_density()
                .is_none()
        );
    }

    #[test]
    fn reduction_magnetic_flux() {
        let t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        let t2 = t1.reduction("J/A").unwrap();
        assert_eq!(t2.to_string(), "4 J/A");
        assert!(t2.is_magnetic_flux());

        let t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        let t2 = t1.reduction("T*m^2").unwrap();
        assert_eq!(t2.to_string(), "4 m^2*T");
        assert!(t2.is_magnetic_flux());

        let t1 = 4.0 * UnitMagneticFlux::Weber(Metric::None);
        let t2 = t1.reduction("s*V").unwrap();
        assert_eq!(t2.to_string(), "4 s*V");
        assert!(t2.is_magnetic_flux());

        let t3 = (4.0 * UnitMagneticFlux::Weber(Metric::None))
            .reduce_magnetic_flux()
            .unwrap();
        assert_eq!(t3.to_string(), "4 m^2*kg/s^2*A");
        assert!(t3.is_magnetic_flux());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_magnetic_flux()
                .is_none()
        );
    }

    #[test]
    fn reduction_conductance() {
        let t1 = 4.0 * UnitElectricConductance::Siemens(Metric::None);
        let t2 = t1.reduction("1/Ω").unwrap();
        assert_eq!(t2.to_string(), "4 1/Ω");
        assert!(t2.is_conductance());

        let t1 = 4.0 * UnitElectricConductance::Siemens(Metric::None);
        let t2 = t1.reduction("A/V").unwrap();
        assert_eq!(t2.to_string(), "4 A/V");
        assert!(t2.is_conductance());

        let t3 = (4.0 * UnitElectricConductance::Siemens(Metric::None))
            .reduce_electric_conductance()
            .unwrap();
        assert_eq!(t3.to_string(), "4 s^3*A^2/m^2*kg");
        assert!(t3.is_conductance());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_electric_conductance()
                .is_none()
        );
    }

    #[test]
    fn reduction_resistance() {
        let t1 = 4.0 * UnitElectricResistance::Ohm(Metric::None);
        let t2 = t1.reduction("1/S").unwrap();
        assert_eq!(t2.to_string(), "4 1/S");
        assert!(t2.is_resistance());

        let t1 = 4.0 * UnitElectricResistance::Ohm(Metric::None);
        let t2 = t1.reduction("V/A").unwrap();
        assert_eq!(t2.to_string(), "4 V/A");
        assert!(t2.is_resistance());

        let t3 = (4.0 * UnitElectricResistance::Ohm(Metric::None))
            .reduce_electric_resistance()
            .unwrap();
        assert_eq!(t3.to_string(), "4 m^2*kg/s^3*A^2");
        assert!(t3.is_resistance());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_electric_resistance()
                .is_none()
        );
    }

    #[test]
    fn reduction_capacitance() {
        let t1 = 4.0 * UnitElectricCapacitance::Farad(Metric::None);
        let t2 = t1.reduction("C/V").unwrap();
        assert_eq!(t2.to_string(), "4 C/V");
        assert!(t2.is_capacitance());

        let t1 = 4.0 * UnitElectricCapacitance::Farad(Metric::None);
        let t2 = t1.reduction("C^2/J").unwrap();
        assert_eq!(t2.to_string(), "4 C^2/J");
        assert!(t2.is_capacitance());

        let t3 = (4.0 * UnitElectricCapacitance::Farad(Metric::None))
            .reduce_electric_capacitance()
            .unwrap();
        assert_eq!(t3.to_string(), "4 s^4*A^2/m^2*kg");
        assert!(t3.is_capacitance());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_electric_capacitance()
                .is_none()
        );
    }

    #[test]
    fn reduction_electric_potential() {
        let t1 = 4.0 * UnitElectricPotential::Volt(Metric::None);
        let t2 = t1.reduction("W/A").unwrap();
        assert_eq!(t2.to_string(), "4 W/A");
        assert!(t2.is_electric_potential());

        let t1 = 4.0 * UnitElectricPotential::Volt(Metric::None);
        let t2 = t1.reduction("J/C").unwrap();
        assert_eq!(t2.to_string(), "4 J/C");
        assert!(t2.is_electric_potential());

        let t3 = (4.0 * UnitElectricPotential::Volt(Metric::None))
            .reduce_electric_potential()
            .unwrap();
        assert_eq!(t3.to_string(), "4 m^2*kg/s^3*A");
        assert!(t3.is_electric_potential());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_electric_potential()
                .is_none()
        );
    }

    #[test]
    fn reduction_electric_charge() {
        let t1 = 4.0 * UnitElectricCharge::Coulomb(Metric::None);
        let t2 = t1.reduction("A*s").unwrap();
        assert_eq!(t2.to_string(), "4 s*A");
        assert!(t2.is_electric_charge());

        let t1 = 4.0 * UnitElectricCharge::Coulomb(Metric::None);
        let t2 = t1.reduction("F*V").unwrap();
        assert_eq!(t2.to_string(), "4 V*F");
        assert!(t2.is_electric_charge());

        let t3 = (4.0 * UnitElectricCharge::Coulomb(Metric::None))
            .reduce_electric_charge()
            .unwrap();
        assert_eq!(t3.to_string(), "4 s*A");
        assert!(t3.is_electric_charge());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_electric_charge()
                .is_none()
        );
    }

    #[test]
    fn reduction_power() {
        let t1 = 4.0 * UnitPower::Watt(Metric::None);
        let t2 = t1.reduction("J/s").unwrap();
        assert_eq!(t2.to_string(), "4 J/s");
        assert!(t2.is_power());

        let t1 = 4.0 * UnitPower::Watt(Metric::None);
        let t2 = t1.reduction("V*A").unwrap();
        assert_eq!(t2.to_string(), "4 A*V");
        assert!(t2.is_power());

        let t1 = 4.0 * UnitPower::Watt(Metric::None);
        let t2 = t1.reduction("m^2*kg/s^3").unwrap();
        assert_eq!(t2.to_string(), "4 m^2*kg/s^3");
        assert!(t2.is_power());

        let t3 = (4.0 * UnitPower::Watt(Metric::None))
            .reduce_power()
            .unwrap();
        assert_eq!(t3.to_string(), "4 m^2*kg/s^3");
        assert!(t3.is_power());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_power()
                .is_none()
        );
    }

    #[test]
    fn reduction_frequency() {
        let t1 = 4.0 * UnitFrequency::Hertz(Metric::None);
        let t2 = t1.reduction("s^-1").unwrap();
        assert_eq!(t2.to_string(), "4 1/s");
        assert!(t2.is_frequency());
    }

    #[test]
    fn reduction_energy() {
        let t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        let t2 = t1.reduction("N*m").unwrap();
        assert_eq!(t2.to_string(), "4 m*N");
        assert!(t2.is_energy());

        let t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        let t2 = t1.reduction("V*C").unwrap();
        assert_eq!(t2.to_string(), "4 C*V");
        assert!(t2.is_energy());

        let t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        let t2 = t1.reduction("W*s").unwrap();
        assert_eq!(t2.to_string(), "4 s*W");
        assert!(t2.is_energy());

        let t1 = 4.0 * UnitEnergy::Joule(Metric::None);
        let t2 = t1.reduction("m^2*kg/s^2").unwrap();
        assert_eq!(t2.to_string(), "4 m^2*kg/s^2");
        assert!(t2.is_energy());

        let t3 = (4.0 * UnitEnergy::Joule(Metric::None))
            .reduce_energy()
            .unwrap();
        assert_eq!(t3.to_string(), "4 m^2*kg/s^2");
        assert!(t3.is_energy());

        assert!(
            (4.0 * UnitForce::Newton(Metric::None))
                .reduce_energy()
                .is_none()
        );
    }

    #[test]
    fn reduction_force() {
        let t1 = 4.0 * UnitForce::Newton(Metric::None);
        let t2 = t1.reduction("g*m/s^2").unwrap();
        assert_eq!(t2.to_string(), "4000 m*g/s^2");
        assert!(t2.is_force());

        let t3 = (4.0 * UnitForce::Newton(Metric::None))
            .reduce_force()
            .unwrap();
        assert_eq!(t3.to_string(), "4 m*kg/s^2");
        assert!(t3.is_force());

        assert!(
            (4.0 * UnitPressure::Pascal(Metric::None))
                .reduce_force()
                .is_none()
        );
    }

    #[test]
    fn reduction_pressure() {
        let t1 = 1.0 * UnitPressure::Bar(Metric::None);
        let t2 = t1.reduction("kg/m*s^2").unwrap();
        assert_eq!(t2.to_string(), "100000 kg/m*s^2");
        assert!(t2.is_pressure());

        let t3 = (1.0 * UnitPressure::Pascal(Metric::None))
            .reduce_pressure()
            .unwrap();
        assert_eq!(t3.to_string(), "1 kg/m*s^2");
        assert!(t3.is_pressure());

        assert!(
            (1.0 * UnitForce::Newton(Metric::None))
                .reduce_pressure()
                .is_none()
        );
    }

    #[test]
    #[should_panic]
    fn reduction_bad() {
        let t1 = 4.0 * UnitLength::Meter(Metric::None);
        let _ = t1.reduction("s").unwrap();
    }

    #[test]
    #[should_panic]
    fn reduction_bad_text() {
        let t1 = 4.0 * UnitForce::Newton(Metric::None);
        let _ = t1.reduction("zz").unwrap();
    }

    #[test]
    #[should_panic]
    fn reduction_bad_map() {
        let t1 = 4.0 * UnitForce::Newton(Metric::None);
        let _ = t1.reduction("N").unwrap();
    }

    #[test]
    #[should_panic]
    fn reduction_bad_no_match() {
        let t1 = 4.0 * UnitForce::Newton(Metric::None);
        let _ = t1.reduction("g*m/s").unwrap();
    }
}
