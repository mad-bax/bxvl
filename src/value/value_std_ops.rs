use crate::{constants::*, units::Convert, value::Value};

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        if !self.__equal(other) {
            return false;
        }

        self.val == other.val
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        if self.unit_map != other.unit_map {
            return None;
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0
            && self.unit_map != TEMPERATURE_MAP
            && self.v_temperature != other.v_temperature
        {
            return None;
        }

        let mut cmp_val: f64 = other.val;

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                return None;
            }

            let region: usize = 1 << i;
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
                            cmp_val *= other
                                .v_electric_current
                                .unwrap()
                                .convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other
                                .v_electric_charge
                                .unwrap()
                                .convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other
                                .v_electric_potential
                                .unwrap()
                                .convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other
                                .v_electric_conductance
                                .unwrap()
                                .convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other
                                .v_capacitance
                                .unwrap()
                                .convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other
                                .v_resistance
                                .unwrap()
                                .convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance {
                            cmp_val *= other
                                .v_inductance
                                .unwrap()
                                .convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other
                                .v_magnetic_flux
                                .unwrap()
                                .convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other
                                .v_magnetic_flux_density
                                .unwrap()
                                .convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other
                                .v_temperature
                                .unwrap()
                                .convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other
                                .v_substance
                                .unwrap()
                                .convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other
                                .v_luminous_flux_intensity
                                .unwrap()
                                .convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other
                                .v_luminous_flux
                                .unwrap()
                                .convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other
                                .v_illuminance
                                .unwrap()
                                .convert(&self.v_illuminance.unwrap());
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
                            cmp_val *= other
                                .v_frequency
                                .unwrap()
                                .convert(&self.v_frequency.unwrap());
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
                            cmp_val *= other
                                .v_radioactivity
                                .unwrap()
                                .convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other
                                .v_radioactivity_exposure
                                .unwrap()
                                .convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other
                                .v_catalytic
                                .unwrap()
                                .convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other
                                .v_information
                                .unwrap()
                                .convert(&self.v_information.unwrap());
                        }
                    }
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other
                                .v_solid_angle
                                .unwrap()
                                .convert(&self.v_solid_angle.unwrap());
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
    fn eq(&self, other: &f64) -> bool {
        self.val == *other
    }
}

impl PartialOrd<f64> for Value {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(other)
    }
}

impl PartialEq<Value> for f64 {
    fn eq(&self, other: &Value) -> bool {
        self == &other.val
    }
}

impl PartialOrd<Value> for f64 {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.val)
    }
}

#[cfg(test)]
mod std_ops_testing {
    use crate::units::{
        Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
        UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
        UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency,
        UnitIlluminance, UnitInformation, UnitLength, UnitLuminousFlux, UnitLuminousIntensity,
        UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure,
        UnitRadioactivity, UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance,
        UnitTemperature, UnitTime, UnitVolume,
    };

    #[test]
    fn f64_ord() {
        let t1 = 5.4 * UnitLength::Meter(Metric::None);
        assert!(6.0 > t1);
        assert!(6.0 >= t1);
        assert!(5.0 < t1);
        assert!(5.0 <= t1);

        assert!(t1 < 6.0);
        assert!(t1 <= 6.0);
        assert!(t1 > 5.0);
        assert!(t1 >= 5.0);

        assert!(t1 == 5.4);
        assert!(5.4 == t1);
    }

    #[test]
    fn value_eq_borrow() {
        let t1 = 5.4 * UnitLength::Meter(Metric::None);
        #[allow(unused_mut)]
        let mut t2 = 5.4 * UnitLength::Meter(Metric::None);
        #[allow(unused_mut)]
        let mut t3 = 6.0 * UnitLength::Meter(Metric::None);

        assert!(t1 == t2);
        assert!(t2 == t1);
        assert!(t1 != t3);
        assert!(t3 != t1);
    }

    #[test]
    #[should_panic]
    fn value_ords_bad_map() {
        let t1 = 5.4 * UnitLength::Meter(Metric::None);
        let t2 = 5.0 * UnitMass::Ounce;

        assert!(t1 > t2);
    }

    #[test]
    #[should_panic]
    fn value_ords_bad_temp() {
        let t1 = 5.4 * UnitTemperature::Celsius(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 5.0 * UnitTemperature::Kelvin(Metric::None) * UnitLength::Meter(Metric::None);

        assert!(t1 > t2);
    }

    #[test]
    #[should_panic]
    fn value_ords_bad_exp() {
        let t1 = 5.4 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let t2 = 5.0 * UnitLength::Meter(Metric::None);

        assert!(t1 > t2);
    }

    #[test]
    fn value_ords() {
        let t1 = 5.4 * UnitTemperature::Kelvin(Metric::None);
        let t2 = 5.0 * UnitTemperature::Kelvin(Metric::None);
        let t3 = 6.0 * UnitTemperature::Kelvin(Metric::None);
        let t4 = 0.0054 * UnitTemperature::Kelvin(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitSound::Bel(Metric::None);
        let t2 = 5.0 * UnitSound::Bel(Metric::None);
        let t3 = 6.0 * UnitSound::Bel(Metric::None);
        let t4 = 0.0054 * UnitSound::Bel(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitSolidAngle::Steradian(Metric::None);
        let t2 = 5.0 * UnitSolidAngle::Steradian(Metric::None);
        let t3 = 6.0 * UnitSolidAngle::Steradian(Metric::None);
        let t4 = 0.0054 * UnitSolidAngle::Steradian(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitVolume::Liter(Metric::None);
        let t2 = 5.0 * UnitVolume::Liter(Metric::None);
        let t3 = 6.0 * UnitVolume::Liter(Metric::None);
        let t4 = 0.0054 * UnitVolume::Liter(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitTime::Second(Metric::None);
        let t2 = 5.0 * UnitTime::Second(Metric::None);
        let t3 = 6.0 * UnitTime::Second(Metric::None);
        let t4 = 0.0054 * UnitTime::Second(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitSubstance::Mole(Metric::None);
        let t2 = 5.0 * UnitSubstance::Mole(Metric::None);
        let t3 = 6.0 * UnitSubstance::Mole(Metric::None);
        let t4 = 0.0054 * UnitSubstance::Mole(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitMass::Gram(Metric::None);
        let t2 = 5.0 * UnitMass::Gram(Metric::None);
        let t3 = 6.0 * UnitMass::Gram(Metric::None);
        let t4 = 0.0054 * UnitMass::Gram(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitRadioactivity::Becquerel(Metric::None);
        let t2 = 5.0 * UnitRadioactivity::Becquerel(Metric::None);
        let t3 = 6.0 * UnitRadioactivity::Becquerel(Metric::None);
        let t4 = 0.0054 * UnitRadioactivity::Becquerel(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitRadioactivityExposure::Sievert(Metric::None);
        let t2 = 5.0 * UnitRadioactivityExposure::Sievert(Metric::None);
        let t3 = 6.0 * UnitRadioactivityExposure::Sievert(Metric::None);
        let t4 = 0.0054 * UnitRadioactivityExposure::Sievert(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitPower::Watt(Metric::None);
        let t2 = 5.0 * UnitPower::Watt(Metric::None);
        let t3 = 6.0 * UnitPower::Watt(Metric::None);
        let t4 = 0.0054 * UnitPower::Watt(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitPressure::Bar(Metric::None);
        let t2 = 5.0 * UnitPressure::Bar(Metric::None);
        let t3 = 6.0 * UnitPressure::Bar(Metric::None);
        let t4 = 0.0054 * UnitPressure::Bar(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t2 = 5.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t3 = 6.0 * UnitMagneticFluxDensity::Tesla(Metric::None);
        let t4 = 0.0054 * UnitMagneticFluxDensity::Tesla(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitMagneticFlux::Weber(Metric::None);
        let t2 = 5.0 * UnitMagneticFlux::Weber(Metric::None);
        let t3 = 6.0 * UnitMagneticFlux::Weber(Metric::None);
        let t4 = 0.0054 * UnitMagneticFlux::Weber(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitLuminousIntensity::Candela(Metric::None);
        let t2 = 5.0 * UnitLuminousIntensity::Candela(Metric::None);
        let t3 = 6.0 * UnitLuminousIntensity::Candela(Metric::None);
        let t4 = 0.0054 * UnitLuminousIntensity::Candela(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitLuminousFlux::Lumen(Metric::None);
        let t2 = 5.0 * UnitLuminousFlux::Lumen(Metric::None);
        let t3 = 6.0 * UnitLuminousFlux::Lumen(Metric::None);
        let t4 = 0.0054 * UnitLuminousFlux::Lumen(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 4.0 * UnitInformation::Byte(Metric::None);
        let t2 = 2.0 * UnitInformation::Byte(Metric::None);
        let t3 = 8.0 * UnitInformation::Byte(Metric::None);
        let t4 = 0.00390625 * UnitInformation::Byte(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitIlluminance::Lux(Metric::None);
        let t2 = 5.0 * UnitIlluminance::Lux(Metric::None);
        let t3 = 6.0 * UnitIlluminance::Lux(Metric::None);
        let t4 = 0.0054 * UnitIlluminance::Lux(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitFrequency::Hertz(Metric::None);
        let t2 = 5.0 * UnitFrequency::Hertz(Metric::None);
        let t3 = 6.0 * UnitFrequency::Hertz(Metric::None);
        let t4 = 0.0054 * UnitFrequency::Hertz(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitForce::Newton(Metric::None);
        let t2 = 5.0 * UnitForce::Newton(Metric::None);
        let t3 = 6.0 * UnitForce::Newton(Metric::None);
        let t4 = 0.0054 * UnitForce::Newton(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitEnergy::Joule(Metric::None);
        let t2 = 5.0 * UnitEnergy::Joule(Metric::None);
        let t3 = 6.0 * UnitEnergy::Joule(Metric::None);
        let t4 = 0.0054 * UnitEnergy::Joule(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitElectricResistance::Ohm(Metric::None);
        let t2 = 5.0 * UnitElectricResistance::Ohm(Metric::None);
        let t3 = 6.0 * UnitElectricResistance::Ohm(Metric::None);
        let t4 = 0.0054 * UnitElectricResistance::Ohm(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitElectricPotential::Volt(Metric::None);
        let t2 = 5.0 * UnitElectricPotential::Volt(Metric::None);
        let t3 = 6.0 * UnitElectricPotential::Volt(Metric::None);
        let t4 = 0.0054 * UnitElectricPotential::Volt(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitElectricInductance::Henry(Metric::None);
        let t2 = 5.0 * UnitElectricInductance::Henry(Metric::None);
        let t3 = 6.0 * UnitElectricInductance::Henry(Metric::None);
        let t4 = 0.0054 * UnitElectricInductance::Henry(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitElectricCurrent::Ampere(Metric::None);
        let t2 = 5.0 * UnitElectricCurrent::Ampere(Metric::None);
        let t3 = 6.0 * UnitElectricCurrent::Ampere(Metric::None);
        let t4 = 0.0054 * UnitElectricCurrent::Ampere(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitElectricConductance::Siemens(Metric::None);
        let t2 = 5.0 * UnitElectricConductance::Siemens(Metric::None);
        let t3 = 6.0 * UnitElectricConductance::Siemens(Metric::None);
        let t4 = 0.0054 * UnitElectricConductance::Siemens(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitElectricCharge::Coulomb(Metric::None);
        let t2 = 5.0 * UnitElectricCharge::Coulomb(Metric::None);
        let t3 = 6.0 * UnitElectricCharge::Coulomb(Metric::None);
        let t4 = 0.0054 * UnitElectricCharge::Coulomb(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitElectricCapacitance::Farad(Metric::None);
        let t2 = 5.0 * UnitElectricCapacitance::Farad(Metric::None);
        let t3 = 6.0 * UnitElectricCapacitance::Farad(Metric::None);
        let t4 = 0.0054 * UnitElectricCapacitance::Farad(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitLength::Meter(Metric::None);
        let t2 = 5.0 * UnitLength::Meter(Metric::None);
        let t3 = 6.0 * UnitLength::Meter(Metric::None);
        let t4 = 0.0054 * UnitLength::Meter(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitAbsorbedDose::Gray(Metric::None);
        let t2 = 5.0 * UnitAbsorbedDose::Gray(Metric::None);
        let t3 = 6.0 * UnitAbsorbedDose::Gray(Metric::None);
        let t4 = 0.0054 * UnitAbsorbedDose::Gray(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitAngle::Radian(Metric::None);
        let t2 = 5.0 * UnitAngle::Radian(Metric::None);
        let t3 = 6.0 * UnitAngle::Radian(Metric::None);
        let t4 = 0.0054 * UnitAngle::Radian(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);

        let t1 = 5.4 * UnitCatalyticActivity::Katal(Metric::None);
        let t2 = 5.0 * UnitCatalyticActivity::Katal(Metric::None);
        let t3 = 6.0 * UnitCatalyticActivity::Katal(Metric::None);
        let t4 = 0.0054 * UnitCatalyticActivity::Katal(Metric::Kilo);

        assert!(t1 < t3 && t1 > t2);
        assert!(t1 <= t3 && t1 >= t2);
        assert!(t3 > t1 && t3 > t2);
        assert!(t3 >= t1 && t3 >= t2);
        assert!(t2 < t1 && t2 < t3);
        assert!(t2 <= t1 && t2 <= t3);

        assert!(t4 > t2 && t4 < t3);
        assert!(t4 >= t2 && t4 <= t3);
        assert!(t4 >= t1);
        assert!(t4 <= t1);
    }
}
