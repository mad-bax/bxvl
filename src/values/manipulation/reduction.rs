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
                | CATALYTIC_ACTIVITY_MAP
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
    use crate::units::{Metric, UnitCatalyticActivity, UnitElectricCapacitance, UnitElectricCharge, UnitElectricConductance, UnitElectricInductance, UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency, UnitIlluminance, UnitLength, UnitLuminousFlux, UnitMagneticFlux, UnitMagneticFluxDensity, UnitPower, UnitPressure};

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
}
