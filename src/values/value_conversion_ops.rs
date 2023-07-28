use std::ops::{Shr, ShrAssign};

use crate::values::Value;
use crate::errors::V3Error;
use crate::constants::*;
use crate::units::{
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
    UnitMagneticFlux,
    UnitMagneticFluxDensity,
    UnitMass,
    UnitPower,
    UnitPressure,
    UnitRadioactivity,
    UnitRadioactivityExposure,
    UnitResistance,
    UnitSolidAngle,
    UnitSound,
    UnitSubstance,
    UnitTemperature,
    UnitTime,
    UnitVolume,
};

impl Shr<Value> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:Value) -> Self::Output {
        if self.__equivalent(&other) {
            let mut ret:Value = self;
            ret._convert(&other)?;
            return Ok(ret);
        }
        Err(V3Error::ValueConversionError("[shr] Incompatible types"))
    }
}

impl Shr<&str> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:&str) -> Self::Output {
        let n:Value = Value::new(1.0, other)?;
        self >> n
    }
}

impl Shr<String> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:String) -> Self::Output {
        let n:Value = Value::new(1.0, other.as_str())?;
        self >> n
    }
}

impl Shr<UnitLength> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitLength) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & LENGTH_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_length.unwrap().convert(&other).powi(self.exp[LENGTH_INDEX]);
        n.v_length = Some(other);
        Ok(n)
    }
}

impl Shr<UnitAbsorbedDose> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitAbsorbedDose) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ABSORBED_DOSE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_ab_dose.unwrap().convert(&other).powi(self.exp[ABSORBED_DOSE_INDEX]);
        n.v_ab_dose = Some(other);
        Ok(n)
    }
}

impl Shr<UnitAngle> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitAngle) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ANGLE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_angle.unwrap().convert(&other).powi(self.exp[ANGLE_INDEX]);
        n.v_angle = Some(other);
        Ok(n)
    }
}

impl Shr<UnitSolidAngle> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitSolidAngle) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & SOLID_ANGLE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_solid_angle.unwrap().convert(&other).powi(self.exp[SOLID_ANGLE_INDEX]);
        n.v_solid_angle = Some(other);
        Ok(n)
    }
}

impl Shr<UnitCapacitance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitCapacitance) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & CAPACITANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_capacitance.unwrap().convert(&other).powi(self.exp[CAPACITANCE_INDEX]);
        n.v_capacitance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitCatalyticActivity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitCatalyticActivity) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & CATALYTIC_ACTIVITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_catalytic.unwrap().convert(&other).powi(self.exp[CATALYTIC_ACTIVITY_INDEX]);
        n.v_catalytic = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricCharge> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricCharge) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ELECTRIC_CHARGE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_electric_charge.unwrap().convert(&other).powi(self.exp[ELECTRIC_CHARGE_INDEX]);
        n.v_electric_charge = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricConductance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricConductance) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ELECTRIC_CONDUCTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_electric_conductance.unwrap().convert(&other).powi(self.exp[ELECTRIC_CONDUCTANCE_INDEX]);
        n.v_electric_conductance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricCurrent> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricCurrent) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ELECTRIC_CURRENT_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_electric_current.unwrap().convert(&other).powi(self.exp[ELECTRIC_CURRENT_INDEX]);
        n.v_electric_current = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricPotential> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricPotential) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ELECTRIC_POTENTIAL_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_electric_potential.unwrap().convert(&other).powi(self.exp[ELECTRIC_POTENTIAL_INDEX]);
        n.v_electric_potential = Some(other);
        Ok(n)
    }
}

impl Shr<UnitEnergy> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitEnergy) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ENERGY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_energy.unwrap().convert(&other).powi(self.exp[ENERGY_INDEX]);
        n.v_energy = Some(other);
        Ok(n)
    }
}

impl Shr<UnitForce> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitForce) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & FORCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_force.unwrap().convert(&other).powi(self.exp[FORCE_INDEX]);
        n.v_force = Some(other);
        Ok(n)
    }
}

impl Shr<UnitFrequency> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitFrequency) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & FREQUENCY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_frequency.unwrap().convert(&other).powi(self.exp[FREQUENCY_INDEX]);
        n.v_frequency = Some(other);
        Ok(n)
    }
}

impl Shr<UnitIlluminance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitIlluminance) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & ILLUMINANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_illuminance.unwrap().convert(&other).powi(self.exp[ILLUMINANCE_INDEX]);
        n.v_illuminance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitInductance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitInductance) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & INDUCTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_inductance.unwrap().convert(&other).powi(self.exp[INDUCTANCE_INDEX]);
        n.v_inductance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitInformation> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitInformation) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & INFORMATION_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_information.unwrap().convert(&other).powi(self.exp[INFORMATION_INDEX]);
        n.v_information = Some(other);
        Ok(n)
    }
}

impl Shr<UnitLuminousFlux> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitLuminousFlux) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & LUMINOUS_FLUX_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_luminous_flux.unwrap().convert(&other).powi(self.exp[LUMINOUS_FLUX_INDEX]);
        n.v_luminous_flux = Some(other);
        Ok(n)
    }
}

impl Shr<UnitLuminousIntensity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitLuminousIntensity) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & LUMINOUS_INTENSITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_luminous_flux_intensity.unwrap().convert(&other).powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
        n.v_luminous_flux_intensity = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMagneticFlux> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitMagneticFlux) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & MAGNETIC_FLUX_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_magnetic_flux.unwrap().convert(&other).powi(self.exp[MAGNETIC_FLUX_INDEX]);
        n.v_magnetic_flux = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMagneticFluxDensity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitMagneticFluxDensity) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & MAGNETIC_FLUX_DENSITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_magnetic_flux_density.unwrap().convert(&other).powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
        n.v_magnetic_flux_density = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMass> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitMass) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & MASS_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_mass.unwrap().convert(&other).powi(self.exp[MASS_INDEX]);
        n.v_mass = Some(other);
        Ok(n)
    }
}

impl Shr<UnitPower> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitPower) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & POWER_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_power.unwrap().convert(&other).powi(self.exp[POWER_INDEX]);
        n.v_power = Some(other);
        Ok(n)
    }
}

impl Shr<UnitPressure> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitPressure) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & PRESSURE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_pressure.unwrap().convert(&other).powi(self.exp[PRESSURE_INDEX]);
        n.v_pressure = Some(other);
        Ok(n)
    }
}

impl Shr<UnitRadioactivity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitRadioactivity) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & RADIOACTIVITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_radioactivity.unwrap().convert(&other).powi(self.exp[RADIOACTIVITY_INDEX]);
        n.v_radioactivity = Some(other);
        Ok(n)
    }
}

impl Shr<UnitRadioactivityExposure> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitRadioactivityExposure) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & RADIOACTIVITY_EXPOSURE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_radioactivity_exposure.unwrap().convert(&other).powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
        n.v_radioactivity_exposure = Some(other);
        Ok(n)
    }
}

impl Shr<UnitResistance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitResistance) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & RESISTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_resistance.unwrap().convert(&other).powi(self.exp[RESISTANCE_INDEX]);
        n.v_resistance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitSound> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitSound) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & SOUND_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_sound.unwrap().convert(&other).powi(self.exp[SOUND_INDEX]);
        n.v_sound = Some(other);
        Ok(n)
    }
}

impl Shr<UnitSubstance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitSubstance) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & SUBSTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_substance.unwrap().convert(&other).powi(self.exp[SUBSTANCE_INDEX]);
        n.v_substance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitTemperature> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitTemperature) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & TEMPERATURE_MAP != TEMPERATURE_MAP {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        if self.exp[TEMPERATURE_INDEX] != 1 || self.exp[TEMPERATURE_INDEX] != -1 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val = n.v_temperature.unwrap().convert(&other, n.val).powi(self.exp[TEMPERATURE_INDEX]);
        n.v_temperature = Some(other);
        Ok(n)
    }
}

impl Shr<UnitTime> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitTime) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & TIME_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_time.unwrap().convert(&other).powi(self.exp[TIME_INDEX]);
        n.v_time = Some(other);
        Ok(n)
    }
}

impl Shr<UnitVolume> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitVolume) -> Self::Output {
        let mut n:Value = self;
        if self.unit_map & LENGTH_MAP != 0 && self.exp[LENGTH_INDEX] == 3 {
            n.val = self.val*self.v_length.unwrap().convert_liter(&other);
            n.v_volume = Some(other);
            n.unit_map = VOLUME_MAP;
            n.exp[VOLUME_INDEX] = 1;
            return Ok(n);
        } else if self.unit_map & VOLUME_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_volume.unwrap().convert(&other).powi(self.exp[VOLUME_INDEX]);
        n.v_volume = Some(other);
        Ok(n)
    }
}

impl ShrAssign<Value> for Value {
    fn shr_assign(&mut self, other:Value) {
        if self.__equivalent(&other) {
            match self._convert(&other) {
                Ok(_) => {},
                Err(_) => panic!("[shr_assign] Incompatible value types: {}, {}", self, other)
            }
        } else {
            panic!("[shr_assign] Incompatible value types: {}, {}", self, other);
        }
    }
}

impl ShrAssign<&str> for Value {
    fn shr_assign(&mut self, other:&str) {
        let n:Value = match Value::new(1.0, other) {
            Ok(t) => t,
            Err(_) => panic!("[shr_assign] Incompatible value types")
        };
        *self >>= n;
    }
}

impl ShrAssign<String> for Value {
    fn shr_assign(&mut self, other:String) {
        let n:Value = match Value::new(1.0, other.as_str()) {
            Ok(t) => t,
            Err(_) => panic!("[shr_assign] Incompatible value types")
        };
        *self >>= n;
    }
}

impl ShrAssign<UnitLength> for Value {
    fn shr_assign(&mut self, other:UnitLength) {
        if self.unit_map & LENGTH_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_length.unwrap().convert(&other).powi(self.exp[LENGTH_INDEX]);
        self.v_length = Some(other);
    }
}

impl ShrAssign<UnitAbsorbedDose> for Value {
    fn shr_assign(&mut self, other:UnitAbsorbedDose) {
        if self.unit_map & ABSORBED_DOSE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_ab_dose.unwrap().convert(&other).powi(self.exp[ABSORBED_DOSE_INDEX]);
        self.v_ab_dose = Some(other);
    }
}

impl ShrAssign<UnitAngle> for Value {
    fn shr_assign(&mut self, other:UnitAngle) {
        if self.unit_map & ANGLE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_angle.unwrap().convert(&other).powi(self.exp[ANGLE_INDEX]);
        self.v_angle = Some(other);
    }
}

impl ShrAssign<UnitSolidAngle> for Value {
    fn shr_assign(&mut self, other:UnitSolidAngle) {
        if self.unit_map & SOLID_ANGLE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_solid_angle.unwrap().convert(&other).powi(self.exp[SOLID_ANGLE_INDEX]);
        self.v_solid_angle = Some(other);
    }
}

impl ShrAssign<UnitCapacitance> for Value {
    fn shr_assign(&mut self, other:UnitCapacitance) {
        if self.unit_map & CAPACITANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_capacitance.unwrap().convert(&other).powi(self.exp[CAPACITANCE_INDEX]);
        self.v_capacitance = Some(other);
    }
}

impl ShrAssign<UnitCatalyticActivity> for Value {
    fn shr_assign(&mut self, other:UnitCatalyticActivity) {
        if self.unit_map & CATALYTIC_ACTIVITY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_catalytic.unwrap().convert(&other).powi(self.exp[CATALYTIC_ACTIVITY_INDEX]);
        self.v_catalytic = Some(other);
    }
}

impl ShrAssign<UnitElectricCharge> for Value {
    fn shr_assign(&mut self, other:UnitElectricCharge) {
        if self.unit_map & ELECTRIC_CHARGE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_electric_charge.unwrap().convert(&other).powi(self.exp[ELECTRIC_CHARGE_INDEX]);
        self.v_electric_charge = Some(other);
    }
}

impl ShrAssign<UnitElectricConductance> for Value {
    fn shr_assign(&mut self, other:UnitElectricConductance) {
        if self.unit_map & ELECTRIC_CONDUCTANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_electric_conductance.unwrap().convert(&other).powi(self.exp[ELECTRIC_CONDUCTANCE_INDEX]);
        self.v_electric_conductance = Some(other);
    }
}

impl ShrAssign<UnitElectricCurrent> for Value {
    fn shr_assign(&mut self, other:UnitElectricCurrent) {
        if self.unit_map & ELECTRIC_CURRENT_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_electric_current.unwrap().convert(&other).powi(self.exp[ELECTRIC_CURRENT_INDEX]);
        self.v_electric_current = Some(other);
    }
}

impl ShrAssign<UnitElectricPotential> for Value {
    fn shr_assign(&mut self, other:UnitElectricPotential) {
        if self.unit_map & ELECTRIC_POTENTIAL_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_electric_potential.unwrap().convert(&other).powi(self.exp[ELECTRIC_POTENTIAL_INDEX]);
        self.v_electric_potential = Some(other);
    }
}

impl ShrAssign<UnitEnergy> for Value {
    fn shr_assign(&mut self, other:UnitEnergy) {
        if self.unit_map & ENERGY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_energy.unwrap().convert(&other).powi(self.exp[ENERGY_INDEX]);
        self.v_energy = Some(other);
    }
}

impl ShrAssign<UnitForce> for Value {
    fn shr_assign(&mut self, other:UnitForce) {
        if self.unit_map & FORCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_force.unwrap().convert(&other).powi(self.exp[FORCE_INDEX]);
        self.v_force = Some(other);
    }
}

impl ShrAssign<UnitFrequency> for Value {
    fn shr_assign(&mut self, other:UnitFrequency) {
        if self.unit_map & FREQUENCY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_frequency.unwrap().convert(&other).powi(self.exp[FREQUENCY_INDEX]);
        self.v_frequency = Some(other);
    }
}

impl ShrAssign<UnitIlluminance> for Value {
    fn shr_assign(&mut self, other:UnitIlluminance) {
        if self.unit_map & ILLUMINANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_illuminance.unwrap().convert(&other).powi(self.exp[ILLUMINANCE_INDEX]);
        self.v_illuminance = Some(other);
    }
}

impl ShrAssign<UnitInductance> for Value {
    fn shr_assign(&mut self, other:UnitInductance) {
        if self.unit_map & INDUCTANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_inductance.unwrap().convert(&other).powi(self.exp[INDUCTANCE_INDEX]);
        self.v_inductance = Some(other);
    }
}

impl ShrAssign<UnitInformation> for Value {
    fn shr_assign(&mut self, other:UnitInformation) {
        if self.unit_map & INFORMATION_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_information.unwrap().convert(&other).powi(self.exp[INFORMATION_INDEX]);
        self.v_information = Some(other);
    }
}

impl ShrAssign<UnitLuminousFlux> for Value {
    fn shr_assign(&mut self, other:UnitLuminousFlux) {
        if self.unit_map & LUMINOUS_FLUX_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_luminous_flux.unwrap().convert(&other).powi(self.exp[LUMINOUS_FLUX_INDEX]);
        self.v_luminous_flux = Some(other);
    }
}

impl ShrAssign<UnitLuminousIntensity> for Value {
    fn shr_assign(&mut self, other:UnitLuminousIntensity) {
        if self.unit_map & LUMINOUS_INTENSITY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_luminous_flux_intensity.unwrap().convert(&other).powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
        self.v_luminous_flux_intensity = Some(other);
    }
}

impl ShrAssign<UnitMagneticFlux> for Value {
    fn shr_assign(&mut self, other:UnitMagneticFlux) {
        if self.unit_map & MAGNETIC_FLUX_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_magnetic_flux.unwrap().convert(&other).powi(self.exp[MAGNETIC_FLUX_INDEX]);
        self.v_magnetic_flux = Some(other);
    }
}

impl ShrAssign<UnitMagneticFluxDensity> for Value {
    fn shr_assign(&mut self, other:UnitMagneticFluxDensity) {
        if self.unit_map & MAGNETIC_FLUX_DENSITY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_magnetic_flux_density.unwrap().convert(&other).powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
        self.v_magnetic_flux_density = Some(other);
    }
}

impl ShrAssign<UnitMass> for Value {
    fn shr_assign(&mut self, other:UnitMass) {
        if self.unit_map & MASS_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_mass.unwrap().convert(&other).powi(self.exp[MASS_INDEX]);
        self.v_mass = Some(other);
    }
}

impl ShrAssign<UnitPower> for Value {
    fn shr_assign(&mut self, other:UnitPower) {
        if self.unit_map & POWER_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_power.unwrap().convert(&other).powi(self.exp[POWER_INDEX]);
        self.v_power = Some(other);
    }
}

impl ShrAssign<UnitPressure> for Value {
    fn shr_assign(&mut self, other:UnitPressure) {
        if self.unit_map & PRESSURE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_pressure.unwrap().convert(&other).powi(self.exp[PRESSURE_INDEX]);
        self.v_pressure = Some(other);
    }
}

impl ShrAssign<UnitRadioactivity> for Value {
    fn shr_assign(&mut self, other:UnitRadioactivity) {
        if self.unit_map & RADIOACTIVITY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_radioactivity.unwrap().convert(&other).powi(self.exp[RADIOACTIVITY_INDEX]);
        self.v_radioactivity = Some(other);
    }
}

impl ShrAssign<UnitRadioactivityExposure> for Value {
    fn shr_assign(&mut self, other:UnitRadioactivityExposure) {
        if self.unit_map & RADIOACTIVITY_EXPOSURE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_radioactivity_exposure.unwrap().convert(&other).powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
        self.v_radioactivity_exposure = Some(other);
    }
}

impl ShrAssign<UnitResistance> for Value {
    fn shr_assign(&mut self, other:UnitResistance) {
        if self.unit_map & RESISTANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_resistance.unwrap().convert(&other).powi(self.exp[RESISTANCE_INDEX]);
        self.v_resistance = Some(other);
    }
}

impl ShrAssign<UnitSound> for Value {
    fn shr_assign(&mut self, other:UnitSound) {
        if self.unit_map & SOUND_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_sound.unwrap().convert(&other).powi(self.exp[SOUND_INDEX]);
        self.v_sound = Some(other);
    }
}

impl ShrAssign<UnitSubstance> for Value {
    fn shr_assign(&mut self, other:UnitSubstance) {
        if self.unit_map & SUBSTANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_substance.unwrap().convert(&other).powi(self.exp[SUBSTANCE_INDEX]);
        self.v_substance = Some(other);
    }
}

impl ShrAssign<UnitTemperature> for Value {
    fn shr_assign(&mut self, other:UnitTemperature) {
        if self.unit_map & TEMPERATURE_MAP != TEMPERATURE_MAP {
            panic!("[shr_assign] Incompatible value types");
        }
        if self.exp[TEMPERATURE_INDEX] != 1 || self.exp[TEMPERATURE_INDEX] != -1 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val = self.v_temperature.unwrap().convert(&other, self.val).powi(self.exp[TEMPERATURE_INDEX]);
        self.v_temperature = Some(other);
    }
}

impl ShrAssign<UnitTime> for Value {
    fn shr_assign(&mut self, other:UnitTime) {
        if self.unit_map & TIME_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self.v_time.unwrap().convert(&other).powi(self.exp[TIME_INDEX]);
        self.v_time = Some(other);
    }
}

impl ShrAssign<UnitVolume> for Value {
    fn shr_assign(&mut self, other:UnitVolume) {
        if self.unit_map & LENGTH_MAP != 0 && self.exp[LENGTH_INDEX] == 3 {
            self.val = self.val*self.v_length.unwrap().convert_liter(&other);
            self.v_volume = Some(other);
            self.unit_map = VOLUME_MAP;
            self.exp[LENGTH_INDEX] = 0;
            self.exp[VOLUME_INDEX] = 1;
            return;
        } else if self.unit_map & VOLUME_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }

        self.val *= self.v_volume.unwrap().convert(&other).powi(self.exp[VOLUME_INDEX]);
        self.v_volume = Some(other);
    }
}