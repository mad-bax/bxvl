use crate::{
    constants::{
        CAPACITANCE_INDEX, CAPACITANCE_MAP, CATALYTIC_ACTIVITY_INDEX, CATALYTIC_ACTIVITY_MAP,
        ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP, ELECTRIC_CONDUCTANCE_INDEX,
        ELECTRIC_CONDUCTANCE_MAP, ELECTRIC_CURRENT_INDEX, ELECTRIC_POTENTIAL_INDEX,
        ELECTRIC_POTENTIAL_MAP, ENERGY_INDEX, ENERGY_MAP, FORCE_INDEX, FORCE_MAP,
        ILLUMINANCE_INDEX, ILLUMINANCE_MAP, INDUCTANCE_INDEX, INDUCTANCE_MAP, LENGTH_INDEX,
        LUMINOUS_FLUX_INDEX, LUMINOUS_FLUX_MAP, LUMINOUS_INTENSITY_INDEX,
        MAGNETIC_FLUX_DENSITY_INDEX, MAGNETIC_FLUX_DENSITY_MAP, MAGNETIC_FLUX_INDEX,
        MAGNETIC_FLUX_MAP, MASS_INDEX, POWER_INDEX, POWER_MAP, PRESSURE_INDEX, PRESSURE_MAP,
        RESISTANCE_INDEX, RESISTANCE_MAP, SOLID_ANGLE_INDEX, SUBSTANCE_INDEX, TIME_INDEX, TIME_MAP,
    },
    errors::V3Error,
    units::{
        Metric, UnitCatalyticActivity, UnitElectricCapacitance, UnitElectricCharge,
        UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
        UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitIlluminance,
        UnitLength, UnitLuminousFlux, UnitLuminousIntensity, UnitMagneticFlux,
        UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure, UnitSolidAngle, UnitSubstance,
        UnitTime,
    },
    values::Value,
};

impl Value {
    /// Combines unit types in a [`Value`] if applicable.
    ///
    /// When multiplying different [`Value`]s together, there are specific combinations that
    /// can create more complex unit types which are supported by the [`Value`] type.
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
    /// use v3::units::{UnitLength, UnitMass, UnitTime, UnitForce, Metric};
    /// let mass:Value = 4.5 * UnitMass::Gram(Metric::Kilo);
    /// let acc:Value = 9.81 / UnitTime::Second(Metric::None) / UnitTime::Second(Metric::None) * UnitLength::Meter(Metric::None);
    /// let mut f:Value = match (mass*acc).complex() {
    ///     Ok(t) => t,
    ///     Err(e) => panic!("{}", e)
    /// };
    /// assert!(f==44.145 * UnitForce::Newton(Metric::None));
    /// ```
    pub fn complex(&self) -> Result<Value, V3Error> {
        let mut ret: Value = *self;
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
            if ret.unit_map & FORCE_MAP == FORCE_MAP {
                (ret >>= UnitForce::Newton(Metric::None));
                (ret >>= UnitLength::Meter(Metric::None));
                ret.exp[FORCE_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.v_force = None;
                ret.v_length = None;
            } else {
                ret >>= UnitMass::Gram(Metric::Kilo);
                ret >>= UnitLength::Meter(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret.exp[MASS_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_mass = None;
                ret.v_time = None;
                ret.v_length = None;
            }
            ret.unit_map = PRESSURE_MAP;
            ret.exp[PRESSURE_INDEX] = 1;
            ret.v_pressure = Some(UnitPressure::Pascal(Metric::None));
        } else if ret.is_energy() && ret.unit_map != ENERGY_MAP {
            if ret.unit_map & FORCE_MAP == FORCE_MAP {
                ret >>= UnitForce::Newton(Metric::None);
                ret >>= UnitLength::Meter(Metric::None);
                ret.exp[FORCE_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.v_force = None;
                ret.v_length = None;
            } else if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                ret >>= UnitElectricCharge::Coulomb(Metric::None);
                ret >>= UnitElectricPotential::Volt(Metric::None);
                ret.exp[ELECTRIC_CHARGE_INDEX] = 0;
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_charge = None;
            } else if ret.unit_map & POWER_MAP == POWER_MAP {
                ret >>= UnitPower::Watt(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret.exp[POWER_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_power = None;
                ret.v_time = None;
            } else {
                ret >>= UnitMass::Gram(Metric::Kilo);
                ret >>= UnitLength::Meter(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret.exp[MASS_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_mass = None;
                ret.v_length = None;
                ret.v_time = None;
            }
            ret.unit_map = ENERGY_MAP;
            ret.exp[ENERGY_INDEX] = 1;
            ret.v_energy = Some(UnitEnergy::Joule(Metric::None));
        } else if ret.is_power() && ret.unit_map != POWER_MAP {
            if ret.unit_map & ENERGY_MAP == ENERGY_MAP {
                ret >>= UnitEnergy::Joule(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret.exp[ENERGY_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
                ret.v_energy = None;
                ret.v_time = None;
            } else if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                ret >>= UnitElectricPotential::Volt(Metric::None);
                ret >>= UnitElectricCurrent::Ampere(Metric::None);
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.exp[ELECTRIC_CURRENT_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_current = None;
            } else {
                ret >>= UnitMass::Gram(Metric::Kilo);
                ret >>= UnitLength::Meter(Metric::None);
                ret >>= UnitTime::Second(Metric::None);
                ret.exp[MASS_INDEX] = 0;
                ret.exp[LENGTH_INDEX] = 0;
                ret.exp[TIME_INDEX] = 0;
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
                (ret >>= UnitElectricCapacitance::Farad(Metric::None));
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
            }
            ret.unit_map = ELECTRIC_POTENTIAL_MAP;
            ret.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
            ret.v_electric_potential = Some(UnitElectricPotential::Volt(Metric::None));
        } else if ret.is_capacitance() && self.unit_map != CAPACITANCE_MAP {
            if ret.unit_map & ELECTRIC_POTENTIAL_MAP == ELECTRIC_POTENTIAL_MAP {
                (ret >>= UnitElectricCharge::Coulomb(Metric::None));
                (ret >>= UnitElectricPotential::Volt(Metric::None));
                ret.exp[ELECTRIC_CHARGE_INDEX] = 0;
                ret.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
                ret.v_electric_potential = None;
                ret.v_electric_charge = None;
            } else if ret.unit_map & ENERGY_MAP == ENERGY_MAP {
                (ret >>= UnitElectricCharge::Coulomb(Metric::None));
                (ret >>= UnitEnergy::Joule(Metric::None));
                ret.exp[ELECTRIC_CHARGE_INDEX] = 0;
                ret.exp[ENERGY_INDEX] = 0;
                ret.v_electric_charge = None;
                ret.v_energy = None;
            }
            ret.unit_map = CAPACITANCE_MAP;
            ret.exp[CAPACITANCE_INDEX] = 1;
            ret.v_capacitance = Some(UnitElectricCapacitance::Farad(Metric::None));
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
            }

            ret.unit_map = RESISTANCE_MAP;
            ret.exp[RESISTANCE_INDEX] = 1;
            ret.v_resistance = Some(UnitElectricResistance::Ohm(Metric::None));
        } else if ret.is_conductance() && self.unit_map != ELECTRIC_CONDUCTANCE_MAP {
            if ret.unit_map & RESISTANCE_MAP == RESISTANCE_MAP {
                (ret >>= UnitElectricResistance::Ohm(Metric::None));
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
                ret >>= UnitElectricResistance::Ohm(Metric::None);
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
            }

            ret.unit_map = INDUCTANCE_MAP;
            ret.exp[INDUCTANCE_INDEX] = 1;
            ret.v_inductance = Some(UnitElectricInductance::Henry(Metric::None));
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
}

#[cfg(test)]
mod complexity_testing {
    use crate::units::{
        Metric, UnitElectricCapacitance, UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitLength, UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure, UnitTime
    };

    #[test]
    fn complexity_magnetic_flux_density3() {
        let t1 = 4.5 * UnitForce::Newton(Metric::None);
        let t2 = 2.0 / UnitLength::Meter(Metric::None);
        let t3 = 2.0 / UnitElectricCurrent::Ampere(Metric::None);
        let res = (t1 * t2 * t3).complex().unwrap();
        assert!(res == 18.0 * UnitMagneticFluxDensity::Tesla(Metric::None));
    }

    #[test]
    fn complexity_magnetic_flux_density2() {
        let t1 = 4.5 * UnitMagneticFlux::Weber(Metric::None);
        let t2 = 2.0 / UnitLength::Meter(Metric::None) / UnitLength::Meter(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitMagneticFluxDensity::Tesla(Metric::None));
    }

    #[test]
    fn complexity_magnetic_flux_density1() {
        let t1 = 4.5 * UnitElectricPotential::Volt(Metric::None);
        let t2 = 2.0 * UnitTime::Second(Metric::None);
        let t3 = 2.0 / UnitLength::Meter(Metric::None) / UnitLength::Meter(Metric::None);
        let res = (t1 * t2 * t3).complex().unwrap();
        assert!(res == 18.0 * UnitMagneticFluxDensity::Tesla(Metric::None));
    }

    #[test]
    fn complexity_magnetic_flux1() {
        let t1 = 4.5 / UnitElectricCurrent::Ampere(Metric::None);
        let t2 = 2.0 * UnitEnergy::Joule(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitMagneticFlux::Weber(Metric::None));
    }

    #[test]
    fn complexity_magnetic_flux2() {
        let t1 = 4.5 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t2 = 2.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitMagneticFlux::Weber(Metric::None));
    }

    #[test]
    fn complexity_magnetic_flux3() {
        let t1 = 4.5 * UnitElectricPotential::Volt(Metric::None);
        let t2 = 2.0 * UnitTime::Second(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitMagneticFlux::Weber(Metric::None));
    }

    #[test]
    fn complexity_electric_conductance1() {
        let t1 = 2.0 / UnitElectricResistance::Ohm(Metric::None);
        let res = t1.complex().unwrap();
        assert!(res == 2.0 * UnitElectricConductance::Siemens(Metric::None));
    }

    #[test]
    fn complexity_electric_conductance2() {
        let t1 = 4.5 * UnitElectricCurrent::Ampere(Metric::None);
        let t2 = 2.0 / UnitElectricPotential::Volt(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricConductance::Siemens(Metric::None));
    }

    #[test]
    fn complexity_electric_resistance1() {
        let t1 = 2.0 / UnitElectricConductance::Siemens(Metric::None);
        let res = t1.complex().unwrap();
        assert!(res == 2.0 * UnitElectricResistance::Ohm(Metric::None));
    }

    #[test]
    fn complexity_electric_resistance2() {
        let t1 = 4.5 / UnitElectricCurrent::Ampere(Metric::None);
        let t2 = 2.0 * UnitElectricPotential::Volt(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricResistance::Ohm(Metric::None));
    }

    #[test]
    fn complexity_electric_capacitance1() {
        let t1 = 4.5 * UnitElectricCharge::Coulomb(Metric::None);
        let t2 = 2.0 / UnitElectricPotential::Volt(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricCapacitance::Farad(Metric::None));
    }

    #[test]
    fn complexity_electric_capacitance2() {
        let t1 = 4.5
            * UnitElectricCharge::Coulomb(Metric::None)
            * UnitElectricCharge::Coulomb(Metric::None);
        let t2 = 2.0 / UnitEnergy::Joule(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricCapacitance::Farad(Metric::None));
    }

    #[test]
    fn complexity_electric_potential1() {
        let t1 = 4.5 * UnitPower::Watt(Metric::None);
        let t2 = 2.0 / UnitElectricCurrent::Ampere(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricPotential::Volt(Metric::None));
    }

    #[test]
    fn complexity_electric_potential2() {
        let t1 = 4.5 * UnitEnergy::Joule(Metric::None);
        let t2 = 2.0 / UnitElectricCharge::Coulomb(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricPotential::Volt(Metric::None));
    }

    #[test]
    fn complexity_electric_charge1() {
        let t1 = 4.5 * UnitTime::Second(Metric::None);
        let t2 = 2.0 * UnitElectricCurrent::Ampere(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricCharge::Coulomb(Metric::None));
    }

    #[test]
    fn complexity_electric_charge2() {
        let t1 = 4.5 * UnitElectricCapacitance::Farad(Metric::None);
        let t2 = 2.0 * UnitElectricPotential::Volt(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitElectricCharge::Coulomb(Metric::None));
    }

    #[test]
    fn complexity_power1() {
        let t1 = 4.5 * UnitEnergy::Joule(Metric::None);
        let t2 = 2.0 / UnitTime::Second(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitPower::Watt(Metric::None));
    }

    #[test]
    fn complexity_power2() {
        let t1 = 4.5 * UnitElectricPotential::Volt(Metric::None);
        let t2 = 2.0 * UnitElectricCurrent::Ampere(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitPower::Watt(Metric::None));
    }

    #[test]
    fn complexity_power3() {
        let t1 = 4.5 * UnitMass::Gram(Metric::Kilo);
        let t2 = 2.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t3 = 5.0
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None);
        let res = (t1 * t2 * t3).complex().unwrap();
        assert!(res == 45.0 * UnitPower::Watt(Metric::None));
    }

    #[test]
    fn complexity_force() {
        let mass = 4.5 * UnitMass::Gram(Metric::Kilo);
        let acc = 9.81 / UnitTime::Second(Metric::None) / UnitTime::Second(Metric::None)
            * UnitLength::Meter(Metric::None);
        let f = (mass * acc).complex().unwrap();
        assert!(f == 44.145 * UnitForce::Newton(Metric::None));
    }

    #[test]
    fn complexity_pressure1() {
        let t1 = 4.5 * UnitForce::Newton(Metric::None);
        let t2 = 2.0 / UnitLength::Meter(Metric::None) / UnitLength::Meter(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitPressure::Pascal(Metric::None));
    }

    #[test]
    fn complexity_pressure2() {
        let t1 = 4.5 / UnitTime::Second(Metric::None) / UnitTime::Second(Metric::None);
        let t2 = 2.0 / UnitLength::Meter(Metric::None);
        let t3 = 5.0 * UnitMass::Gram(Metric::Kilo);
        let res = (t1 * t2 * t3).complex().unwrap();
        assert!(res == 45.0 * UnitPressure::Pascal(Metric::None));
    }

    #[test]
    fn complexity_energy1() {
        let t1 = 4.5 * UnitForce::Newton(Metric::None);
        let t2 = 2.0 * UnitLength::Meter(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitEnergy::Joule(Metric::None));
    }

    #[test]
    fn complexity_energy2() {
        let t1 = 4.5 * UnitElectricPotential::Volt(Metric::None);
        let t2 = 2.0 * UnitElectricCharge::Coulomb(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitEnergy::Joule(Metric::None));
    }

    #[test]
    fn complexity_energy3() {
        let t1 = 4.5 * UnitPower::Watt(Metric::None);
        let t2 = 2.0 * UnitTime::Second(Metric::None);
        let res = (t1 * t2).complex().unwrap();
        assert!(res == 9.0 * UnitEnergy::Joule(Metric::None));
    }

    #[test]
    fn complexity_energy4() {
        let t1 = 4.5 / UnitTime::Second(Metric::None) / UnitTime::Second(Metric::None);
        let t2 = 2.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t3 = 5.0 * UnitMass::Gram(Metric::Kilo);
        let res = (t1 * t2 * t3).complex().unwrap();
        assert!(res == 45.0 * UnitEnergy::Joule(Metric::None));
    }
}
