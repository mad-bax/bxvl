use crate::constants::*;
use crate::values::Value;
use std::fmt::Display;

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                _ => {
                    unreachable!("[fmt] Unreachable");
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

        if final_str.chars().count() > 0 {
            write!(f, "{} {}", self.val, final_str)
        } else {
            write!(f, "{}", self.val)
        }
    }
}

#[cfg(test)]
mod value_string_testing {
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
    #[should_panic]
    fn value_display_corrupt_unit_map() {
        let mut v_1 = 3.4 * UnitLength::Inch / UnitTime::Second(Metric::None);
        v_1.unit_map = usize::MAX;
        v_1.to_string();
    }

    #[test]
    fn value_display_exp() {
        let v_1 = 3.4 * UnitLength::Inch / UnitTime::Second(Metric::None);
        assert_eq!(v_1.to_string(), "3.4 in/s");
        let v_2 = 3.4 / UnitForce::Newton(Metric::None);
        assert_eq!(v_2.to_string(), "3.4 1/N");
        let v_3 = 5.1 * UnitEnergy::Joule(Metric::None) * UnitEnergy::Joule(Metric::None);
        assert_eq!(v_3.to_string(), "5.1 J^2");
        let v_4 = 5.1 / UnitMass::Gram(Metric::None) / UnitMass::Gram(Metric::None);
        assert_eq!(v_4.to_string(), "5.1 1/g^2");
        let v_5 = 7.2 * UnitNone::None;
        assert_eq!(v_5.to_string(), "7.2");
    }

    #[test]
    fn value_display_units() {
        assert_eq!(
            (1.1 * UnitAbsorbedDose::Gray(Metric::None)).to_string(),
            "1.1 Gy"
        );
        assert_eq!(
            (1.1 * UnitAngle::Radian(Metric::None)).to_string(),
            "1.1 rad"
        );
        assert_eq!((1.1 * UnitAngle::Moa).to_string(), "1.1 moa");
        assert_eq!((1.1 * UnitAngle::Degree).to_string(), "1.1 °");
        assert_eq!(
            (1.1 * UnitCatalyticActivity::Katal(Metric::None)).to_string(),
            "1.1 kat"
        );
        assert_eq!(
            (1.1 * UnitElectricCapacitance::Farad(Metric::None)).to_string(),
            "1.1 F"
        );
        assert_eq!(
            (1.1 * UnitElectricCharge::Coulomb(Metric::None)).to_string(),
            "1.1 C"
        );
        assert_eq!(
            (1.1 * UnitElectricConductance::Siemens(Metric::None)).to_string(),
            "1.1 S"
        );
        assert_eq!(
            (1.1 * UnitElectricCurrent::Ampere(Metric::None)).to_string(),
            "1.1 A"
        );
        assert_eq!(
            (1.1 * UnitElectricInductance::Henry(Metric::None)).to_string(),
            "1.1 H"
        );
        assert_eq!(
            (1.1 * UnitElectricPotential::Volt(Metric::None)).to_string(),
            "1.1 V"
        );
        assert_eq!(
            (1.1 * UnitElectricResistance::Ohm(Metric::None)).to_string(),
            "1.1 Ω"
        );
        assert_eq!(
            (1.1 * UnitFrequency::Hertz(Metric::None)).to_string(),
            "1.1 Hz"
        );
        assert_eq!(
            (1.1 * UnitIlluminance::Lux(Metric::None)).to_string(),
            "1.1 lx"
        );
        assert_eq!(
            (1.1 * UnitInformation::Byte(Metric::None)).to_string(),
            "1.1 b"
        );
        assert_eq!(
            (1.1 * UnitLuminousFlux::Lumen(Metric::None)).to_string(),
            "1.1 lm"
        );
        assert_eq!(
            (1.1 * UnitLuminousIntensity::Candela(Metric::None)).to_string(),
            "1.1 cd"
        );
        assert_eq!(
            (1.1 * UnitMagneticFlux::Weber(Metric::None)).to_string(),
            "1.1 Wb"
        );
        assert_eq!(
            (1.1 * UnitMagneticFluxDensity::Tesla(Metric::None)).to_string(),
            "1.1 T"
        );
        assert_eq!((1.1 * UnitPower::Watt(Metric::None)).to_string(), "1.1 W");
        assert_eq!(
            (1.1 * UnitPressure::Bar(Metric::None)).to_string(),
            "1.1 bar"
        );
        assert_eq!(
            (1.1 * UnitRadioactivity::Becquerel(Metric::None)).to_string(),
            "1.1 Bq"
        );
        assert_eq!(
            (1.1 * UnitRadioactivityExposure::Sievert(Metric::None)).to_string(),
            "1.1 Sv"
        );
        assert_eq!(
            (1.1 * UnitSolidAngle::Steradian(Metric::None)).to_string(),
            "1.1 sr"
        );
        assert_eq!((1.1 * UnitSound::Bel(Metric::None)).to_string(), "1.1 B");
        assert_eq!(
            (1.1 * UnitSubstance::Mole(Metric::None)).to_string(),
            "1.1 mol"
        );
        assert_eq!(
            (1.1 * UnitTemperature::Kelvin(Metric::None)).to_string(),
            "1.1 K"
        );
        assert_eq!((1.1 * UnitVolume::Liter(Metric::None)).to_string(), "1.1 l");
    }
}
