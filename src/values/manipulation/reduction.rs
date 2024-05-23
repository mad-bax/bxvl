use crate::{
    constants::{
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
    values::Value,
};

impl Value {
    /// Reduces a [`Value`]'s unit complexity.
    ///
    /// When a [`Value`] has a specific type that is composed from base units such as `Newtons`;
    /// it can be reduced to those base units.
    ///
    /// # Example
    /// ```rust
    /// use v3::{values::Value, units::{Metric, UnitForce}};
    ///
    /// let mut f:Value = 3.0 * UnitForce::Newton(Metric::None);
    ///
    /// match f.reduce("kg*m/s^2") {
    ///     Ok(_) => {}
    ///     Err(e) => panic!("{}", e)
    /// }
    /// ```
    /// `f` will now be equal to `3.0 kg*m/s^2`
    pub fn reduce(&mut self, other: &str) -> Result<(), V3Error> {
        if !self.is_reducible() {
            return Err(V3Error::UnitReductionError(format!(
                "[reduce] Value {} is not reducible",
                self
            )));
        }
        let temp: Value = match Value::new(1.0, other) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
        if self._reduce(&temp) {
            return Ok(());
        }
        Err(V3Error::UnitReductionError(format!(
            "[reduce] Value {} cannot be reduced to {}",
            self, other
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
        )
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
            } else if other.unit_map & ELECTRIC_CONDUCTANCE_MAP > 0 {
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
