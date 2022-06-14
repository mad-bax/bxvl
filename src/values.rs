use crate::units::*;
use crate::constants::*;
use crate::errors::V3Error;

use std::fmt::Display;
use std::io::Error;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::str::FromStr;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[macro_export]
macro_rules! value {
    ($v:expr, $u:expr) => {
        Value::new($v as f64, &$u.to_string()).unwrap()
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Value {
    pub val:f64,                    // The numerical value
    unit_map:usize,                 // Which units are selected
    exp:[i32;30],                   // The exponent of those units 
    v_ab_dose:Option<UnitAbsorbedDose>,          // The units
    v_angle:Option<UnitAngle>,
    v_capacitance:Option<UnitCapacitance>,
    v_catalytic:Option<UnitCatalyticActivity>,
    v_electric_charge:Option<UnitElectricCharge>,
    v_electric_conductance:Option<UnitElectricConductance>,
    v_electric_current:Option<UnitElectricCurrent>,
    v_electric_potential:Option<UnitElectricPotential>,
    v_energy:Option<UnitEnergy>,
    v_force:Option<UnitForce>,
    v_frequency:Option<UnitFrequency>,
    v_illuminance:Option<UnitIlluminance>,
    v_inductance:Option<UnitInductance>,
    v_information:Option<UnitInformation>,
    v_length:Option<UnitLength>,
    v_luminous_flux:Option<UnitLuminousFlux>,
    v_luminous_flux_intensity:Option<UnitLuminousIntensity>,
    v_mass:Option<UnitMass>,
    v_power:Option<UnitPower>,
    v_pressure:Option<UnitPressure>,
    v_radioactivity:Option<UnitRadioactivity>,
    v_radioactivity_exposure:Option<UnitRadioactivityExposure>,
    v_resistance:Option<UnitResistance>,
    v_sound:Option<UnitSound>,
    v_substance:Option<UnitSubstance>,
    v_temperature:Option<UnitTemperature>,
    v_time:Option<UnitTime>,
    v_volume:Option<UnitVolume>,
    v_magnetic_flux:Option<UnitMagneticFlux>,
    v_magnetic_flux_density:Option<UnitMagneticFluxDensity>
}

impl FromStr for Value {
    type Err = V3Error;
    fn from_str(s:&str) -> Result<Value, V3Error> {
        if !s.contains(char::is_whitespace) {
            let val:Value = match s.parse::<f64>() {
                Ok(t) => Value::new(t, "").unwrap(),
                Err(_) => {
                    return Err(V3Error::ParsingError("float conversion"));
                }
            };
            return Ok(val);
        }
        let temp:Vec<&str> = s.split(' ').collect();
        let v:f64 = match temp[0].parse::<f64>() {
            Ok(t) => t,
            Err(_) => {
                    return Err(V3Error::ParsingError("float conversion"));
            }
        };
        Value::new(v, temp[1])
    }
}

impl PartialEq for Value {
    fn eq(&self, other:&Value) -> bool {
        if !self.__equal(other) {
            return false;
        }

        self.val == other.val
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other:&Value) -> Option<std::cmp::Ordering> {
        if self.unit_map != other.unit_map {
            return None;
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map != TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..30_usize {
            if self.exp[i] != other.exp[i] {
                return None;
            }

            let region:usize = 1<<i;
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
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
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
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
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
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }

        self.val.partial_cmp(&cmp_val)
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other:&f64) -> bool {
        self.val == *other
    }
}

impl PartialOrd<f64> for Value {
    fn partial_cmp(&self, other:&f64) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(other)
    }
}

impl Add<f64> for Value {
    type Output = Value;
    fn add(self, rhs:f64) -> Value {
        let mut n:Value = self;
        n.val += rhs;
        n
    }
}

impl AddAssign<f64> for Value {
    fn add_assign(&mut self, rhs:f64) {
        self.val += rhs;
    }
}

impl Sub<f64> for Value {
    type Output = Value;
    fn sub(self, rhs:f64) -> Value {
        let mut n:Value = self;
        n.val -= rhs;
        n
    }
}

impl SubAssign<f64> for Value {
    fn sub_assign(&mut self, rhs:f64) {
        self.val -= rhs;
    }
}

impl Mul<f64> for Value {
    type Output = Value;
    fn mul(self, rhs:f64) -> Value {
        let mut n:Value = self;
        n.val *= rhs;
        n
    }
}

impl MulAssign<f64> for Value {
    fn mul_assign(&mut self, rhs:f64) {
        self.val *= rhs;
    }
}

impl Div<f64> for Value {
    type Output = Value;
    fn div(self, rhs: f64) -> Value { 
        let mut n:Value = self;
        n.val /= rhs;
        n
    }
}

impl DivAssign<f64> for Value {
    fn div_assign(&mut self, rhs:f64) {
        self.val /= rhs;
    }
}

impl Shr<Value> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:Value) -> Self::Output {
        if self.__equivalent(&other) {
            let mut ret:Value = self.clone();
            ret._convert(&other)?;
            return Ok(ret);
        }
        Err(V3Error::ValueConversionError("Incompatable types"))
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
        let mut n:Value = self.clone();
        if self.unit_map & LENGTH_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_length.unwrap().convert(&other);
        n.v_length = Some(other);
        Ok(n)
    }
}

impl Shr<UnitAbsorbedDose> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitAbsorbedDose) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ABSORBED_DOSE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_ab_dose.unwrap().convert(&other);
        n.v_ab_dose = Some(other);
        Ok(n)
    }
}

impl Shr<UnitAngle> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitAngle) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ANGLE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_angle.unwrap().convert(&other);
        n.v_angle = Some(other);
        Ok(n)
    }
}

impl Shr<UnitCapacitance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitCapacitance) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & CAPACITANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_capacitance.unwrap().convert(&other);
        n.v_capacitance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitCatalyticActivity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitCatalyticActivity) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & CATALYTIC_ACTIVITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_catalytic.unwrap().convert(&other);
        n.v_catalytic = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricCharge> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricCharge) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ELECTRIC_CHARGE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_electric_charge.unwrap().convert(&other);
        n.v_electric_charge = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricConductance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricConductance) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ELECTRIC_CONDUCTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_electric_conductance.unwrap().convert(&other);
        n.v_electric_conductance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricCurrent> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricCurrent) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ELECTRIC_CURRENT_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_electric_current.unwrap().convert(&other);
        n.v_electric_current = Some(other);
        Ok(n)
    }
}

impl Shr<UnitElectricPotential> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitElectricPotential) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ELECTRIC_POTENTIAL_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_electric_potential.unwrap().convert(&other);
        n.v_electric_potential = Some(other);
        Ok(n)
    }
}

impl Shr<UnitEnergy> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitEnergy) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ENERGY_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_energy.unwrap().convert(&other);
        n.v_energy = Some(other);
        Ok(n)
    }
}

impl Shr<UnitForce> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitForce) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & FORCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_force.unwrap().convert(&other);
        n.v_force = Some(other);
        Ok(n)
    }
}

impl Shr<UnitFrequency> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitFrequency) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & FREQUENCY_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_frequency.unwrap().convert(&other);
        n.v_frequency = Some(other);
        Ok(n)
    }
}

impl Shr<UnitIlluminance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitIlluminance) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & ILLUMINANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_illuminance.unwrap().convert(&other);
        n.v_illuminance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitInductance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitInductance) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & INDUCTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_inductance.unwrap().convert(&other);
        n.v_inductance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitInformation> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitInformation) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & INFORMATION_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_information.unwrap().convert(&other);
        n.v_information = Some(other);
        Ok(n)
    }
}

impl Shr<UnitLuminousFlux> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitLuminousFlux) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & LUMINOUS_FLUX_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_luminous_flux.unwrap().convert(&other);
        n.v_luminous_flux = Some(other);
        Ok(n)
    }
}

impl Shr<UnitLuminousIntensity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitLuminousIntensity) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & LUMINOUS_INTENSITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_luminous_flux_intensity.unwrap().convert(&other);
        n.v_luminous_flux_intensity = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMagneticFlux> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitMagneticFlux) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & MAGNETRIC_FLUX_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_magnetic_flux.unwrap().convert(&other);
        n.v_magnetic_flux = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMagneticFluxDensity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitMagneticFluxDensity) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & MAGNETRIC_FLUX_DENSITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_magnetic_flux_density.unwrap().convert(&other);
        n.v_magnetic_flux_density = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMass> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitMass) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & MASS_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_mass.unwrap().convert(&other);
        n.v_mass = Some(other);
        Ok(n)
    }
}

impl Shr<UnitPower> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitPower) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & POWER_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_power.unwrap().convert(&other);
        n.v_power = Some(other);
        Ok(n)
    }
}

impl Shr<UnitPressure> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitPressure) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & PRESSURE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_pressure.unwrap().convert(&other);
        n.v_pressure = Some(other);
        Ok(n)
    }
}

impl Shr<UnitRadioactivity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitRadioactivity) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & RADIOACTIVITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_radioactivity.unwrap().convert(&other);
        n.v_radioactivity = Some(other);
        Ok(n)
    }
}

impl Shr<UnitRadioactivityExposure> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitRadioactivityExposure) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & RADIOACTIVITY_EXPOSURE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_radioactivity_exposure.unwrap().convert(&other);
        n.v_radioactivity_exposure = Some(other);
        Ok(n)
    }
}

impl Shr<UnitResistance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitResistance) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & RESISTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_resistance.unwrap().convert(&other);
        n.v_resistance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitSound> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitSound) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & SOUND_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_sound.unwrap().convert(&other);
        n.v_sound = Some(other);
        Ok(n)
    }
}

impl Shr<UnitSubstance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitSubstance) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & SUBSTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_substance.unwrap().convert(&other);
        n.v_substance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitTemperature> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitTemperature) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & TEMPERATURE_MAP != TEMPERATURE_MAP {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        if self.exp[TEMPERATURE_INDEX] != 1 || self.exp[TEMPERATURE_INDEX] != -1 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val = n.v_temperature.unwrap().convert(&other, n.val);
        n.v_temperature = Some(other);
        Ok(n)
    }
}

impl Shr<UnitTime> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitTime) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & TIME_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_time.unwrap().convert(&other);
        n.v_time = Some(other);
        Ok(n)
    }
}

impl Shr<UnitVolume> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other:UnitVolume) -> Self::Output {
        let mut n:Value = self.clone();
        if self.unit_map & VOLUME_MAP == 0 {
            return Err(V3Error::ValueConversionError("Incompatable types"));
        }
        n.val *= n.v_volume.unwrap().convert(&other);
        n.v_volume = Some(other);
        Ok(n)
    }
}

impl ShrAssign<Value> for Value {
    fn shr_assign(&mut self, other:Value) {
        if self.__equivalent(&other) {
            match self._convert(&other) {
                Ok(_) => {},
                Err(_) => panic!("Incompatable value types")
            }
        } else {
            panic!("Incompatable value types");
        }
    }
}

impl ShrAssign<&str> for Value {
    fn shr_assign(&mut self, other:&str) {
        let n:Value = match Value::new(1.0, other) {
            Ok(t) => t,
            Err(_) => panic!("Incompatable value types")
        };
        *self >>= n;
    }
}

impl ShrAssign<String> for Value {
    fn shr_assign(&mut self, other:String) {
        let n:Value = match Value::new(1.0, other.as_str()) {
            Ok(t) => t,
            Err(_) => panic!("Incompatable value types")
        };
        *self >>= n;
    }
}

impl ShrAssign<UnitLength> for Value {
    fn shr_assign(&mut self, other:UnitLength) {
        if self.unit_map & LENGTH_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_length.unwrap().convert(&other);
        self.v_length = Some(other);
    }
}

impl ShrAssign<UnitAbsorbedDose> for Value {
    fn shr_assign(&mut self, other:UnitAbsorbedDose) {
        if self.unit_map & ABSORBED_DOSE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_ab_dose.unwrap().convert(&other);
        self.v_ab_dose = Some(other);
    }
}

impl ShrAssign<UnitAngle> for Value {
    fn shr_assign(&mut self, other:UnitAngle) {
        if self.unit_map & ANGLE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_angle.unwrap().convert(&other);
        self.v_angle = Some(other);
    }
}

impl ShrAssign<UnitCapacitance> for Value {
    fn shr_assign(&mut self, other:UnitCapacitance) {
        if self.unit_map & CAPACITANCE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_capacitance.unwrap().convert(&other);
        self.v_capacitance = Some(other);
    }
}

impl ShrAssign<UnitCatalyticActivity> for Value {
    fn shr_assign(&mut self, other:UnitCatalyticActivity) {
        if self.unit_map & CATALYTIC_ACTIVITY_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_catalytic.unwrap().convert(&other);
        self.v_catalytic = Some(other);
    }
}

impl ShrAssign<UnitElectricCharge> for Value {
    fn shr_assign(&mut self, other:UnitElectricCharge) {
        if self.unit_map & ELECTRIC_CHARGE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_electric_charge.unwrap().convert(&other);
        self.v_electric_charge = Some(other);
    }
}

impl ShrAssign<UnitElectricConductance> for Value {
    fn shr_assign(&mut self, other:UnitElectricConductance) {
        if self.unit_map & ELECTRIC_CONDUCTANCE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_electric_conductance.unwrap().convert(&other);
        self.v_electric_conductance = Some(other);
    }
}

impl ShrAssign<UnitElectricCurrent> for Value {
    fn shr_assign(&mut self, other:UnitElectricCurrent) {
        if self.unit_map & ELECTRIC_CURRENT_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_electric_current.unwrap().convert(&other);
        self.v_electric_current = Some(other);
    }
}

impl ShrAssign<UnitElectricPotential> for Value {
    fn shr_assign(&mut self, other:UnitElectricPotential) {
        if self.unit_map & ELECTRIC_POTENTIAL_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_electric_potential.unwrap().convert(&other);
        self.v_electric_potential = Some(other);
    }
}

impl ShrAssign<UnitEnergy> for Value {
    fn shr_assign(&mut self, other:UnitEnergy) {
        if self.unit_map & ENERGY_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_energy.unwrap().convert(&other);
        self.v_energy = Some(other);
    }
}

impl ShrAssign<UnitForce> for Value {
    fn shr_assign(&mut self, other:UnitForce) {
        if self.unit_map & FORCE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_force.unwrap().convert(&other);
        self.v_force = Some(other);
    }
}

impl ShrAssign<UnitFrequency> for Value {
    fn shr_assign(&mut self, other:UnitFrequency) {
        if self.unit_map & FREQUENCY_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_frequency.unwrap().convert(&other);
        self.v_frequency = Some(other);
    }
}

impl ShrAssign<UnitIlluminance> for Value {
    fn shr_assign(&mut self, other:UnitIlluminance) {
        if self.unit_map & ILLUMINANCE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_illuminance.unwrap().convert(&other);
        self.v_illuminance = Some(other);
    }
}

impl ShrAssign<UnitInductance> for Value {
    fn shr_assign(&mut self, other:UnitInductance) {
        if self.unit_map & INDUCTANCE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_inductance.unwrap().convert(&other);
        self.v_inductance = Some(other);
    }
}

impl ShrAssign<UnitInformation> for Value {
    fn shr_assign(&mut self, other:UnitInformation) {
        if self.unit_map & INFORMATION_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_information.unwrap().convert(&other);
        self.v_information = Some(other);
    }
}

impl ShrAssign<UnitLuminousFlux> for Value {
    fn shr_assign(&mut self, other:UnitLuminousFlux) {
        if self.unit_map & LUMINOUS_FLUX_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_luminous_flux.unwrap().convert(&other);
        self.v_luminous_flux = Some(other);
    }
}

impl ShrAssign<UnitLuminousIntensity> for Value {
    fn shr_assign(&mut self, other:UnitLuminousIntensity) {
        if self.unit_map & LUMINOUS_INTENSITY_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_luminous_flux_intensity.unwrap().convert(&other);
        self.v_luminous_flux_intensity = Some(other);
    }
}

impl ShrAssign<UnitMagneticFlux> for Value {
    fn shr_assign(&mut self, other:UnitMagneticFlux) {
        if self.unit_map & MAGNETRIC_FLUX_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_magnetic_flux.unwrap().convert(&other);
        self.v_magnetic_flux = Some(other);
    }
}

impl ShrAssign<UnitMagneticFluxDensity> for Value {
    fn shr_assign(&mut self, other:UnitMagneticFluxDensity) {
        if self.unit_map & MAGNETRIC_FLUX_DENSITY_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_magnetic_flux_density.unwrap().convert(&other);
        self.v_magnetic_flux_density = Some(other);
    }
}

impl ShrAssign<UnitMass> for Value {
    fn shr_assign(&mut self, other:UnitMass) {
        if self.unit_map & MASS_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_mass.unwrap().convert(&other);
        self.v_mass = Some(other);
    }
}

impl ShrAssign<UnitPower> for Value {
    fn shr_assign(&mut self, other:UnitPower) {
        if self.unit_map & POWER_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_power.unwrap().convert(&other);
        self.v_power = Some(other);
    }
}

impl ShrAssign<UnitPressure> for Value {
    fn shr_assign(&mut self, other:UnitPressure) {
        if self.unit_map & PRESSURE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_pressure.unwrap().convert(&other);
        self.v_pressure = Some(other);
    }
}

impl ShrAssign<UnitRadioactivity> for Value {
    fn shr_assign(&mut self, other:UnitRadioactivity) {
        if self.unit_map & RADIOACTIVITY_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_radioactivity.unwrap().convert(&other);
        self.v_radioactivity = Some(other);
    }
}

impl ShrAssign<UnitRadioactivityExposure> for Value {
    fn shr_assign(&mut self, other:UnitRadioactivityExposure) {
        if self.unit_map & RADIOACTIVITY_EXPOSURE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_radioactivity_exposure.unwrap().convert(&other);
        self.v_radioactivity_exposure = Some(other);
    }
}

impl ShrAssign<UnitResistance> for Value {
    fn shr_assign(&mut self, other:UnitResistance) {
        if self.unit_map & RESISTANCE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_resistance.unwrap().convert(&other);
        self.v_resistance = Some(other);
    }
}

impl ShrAssign<UnitSound> for Value {
    fn shr_assign(&mut self, other:UnitSound) {
        if self.unit_map & SOUND_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_sound.unwrap().convert(&other);
        self.v_sound = Some(other);
    }
}

impl ShrAssign<UnitSubstance> for Value {
    fn shr_assign(&mut self, other:UnitSubstance) {
        if self.unit_map & SUBSTANCE_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_substance.unwrap().convert(&other);
        self.v_substance = Some(other);
    }
}

impl ShrAssign<UnitTemperature> for Value {
    fn shr_assign(&mut self, other:UnitTemperature) {
        if self.unit_map & TEMPERATURE_MAP != TEMPERATURE_MAP {
            panic!("Incompatable value types");
        }
        if self.exp[TEMPERATURE_INDEX] != 1 || self.exp[TEMPERATURE_INDEX] != -1 {
            panic!("Incompatable value types");
        }
        self.val = self.v_temperature.unwrap().convert(&other, self.val);
        self.v_temperature = Some(other);
    }
}

impl ShrAssign<UnitTime> for Value {
    fn shr_assign(&mut self, other:UnitTime) {
        if self.unit_map & TIME_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_time.unwrap().convert(&other);
        self.v_time = Some(other);
    }
}

impl ShrAssign<UnitVolume> for Value {
    fn shr_assign(&mut self, other:UnitVolume) {
        if self.unit_map & VOLUME_MAP == 0 {
            panic!("Incompatable value types");
        }
        self.val *= self.v_volume.unwrap().convert(&other);
        self.v_volume = Some(other);
    }
}

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, other:Value) -> Value {
        if self.unit_map != other.unit_map {
            // Error
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map != TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..30_usize {
            if self.exp[i] != other.exp[i] {
                // Error
            }

            let region:usize = 1<<i;
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
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
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
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
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
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }

        let mut n:Value = self;
        n.val += cmp_val;
        n
    }
}

impl AddAssign<Value> for Value {
    fn add_assign(&mut self, other:Value) {
        if self.unit_map != other.unit_map {
            // Error
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..30_usize {
            if self.exp[i] != other.exp[i] {
                // Error
            }

            let region:usize = 1<<i;
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
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
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
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
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
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }

        self.val += cmp_val;
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn sub(self, other:Value) -> Value {
        if self.unit_map != other.unit_map {
            // Error
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..30_usize {
            if self.exp[i] != other.exp[i] {
                // Error
            }

            let region:usize = 1<<i;
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
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
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
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
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
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }

        let mut n:Value = self;
        n.val -= cmp_val;
        n
    }
}

impl SubAssign<Value> for Value {
    fn sub_assign(&mut self, other:Value) {
        if self.unit_map != other.unit_map {
            // Error
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..30_usize {
            if self.exp[i] != other.exp[i] {
                // Error
            }

            let region:usize = 1<<i;
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
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
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
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
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
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }

        self.val -= cmp_val;
    }
}

impl Mul<Value> for Value {
    type Output = Value;
    fn mul(self, other:Value) -> Value {
        let mut n:Value = self;
        n.unit_map = 0;

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        let mut cmp_val:f64 = other.val;
        for i in 0..30_usize {
            n.exp[i] = self.exp[i] + other.exp[i];
            let region:usize = 1<<i;
            let in_other:bool = region & other.unit_map != 0;
            let in_self:bool = self.unit_map & region != 0;
            let must_assign:bool = !in_self && in_other;

            if n.exp[i] != 0 {
                n.unit_map |= region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            n.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other.v_length.unwrap().convert(&self.v_length.unwrap());
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            n.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other.v_time.unwrap().convert(&self.v_time.unwrap());
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            n.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other.v_mass.unwrap().convert(&self.v_mass.unwrap());
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            n.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            n.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            n.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            n.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            n.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            n.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            n.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if must_assign {
                            n.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            n.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            n.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            n.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            n.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            n.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            n.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            n.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other.v_volume.unwrap().convert(&self.v_volume.unwrap());
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            n.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other.v_pressure.unwrap().convert(&self.v_pressure.unwrap());
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            n.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other.v_angle.unwrap().convert(&self.v_angle.unwrap());
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            n.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
                        }
            
                    }
                    FORCE_MAP => {
                        if must_assign {
                            n.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other.v_force.unwrap().convert(&self.v_force.unwrap());
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            n.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other.v_energy.unwrap().convert(&self.v_energy.unwrap());
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            n.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other.v_power.unwrap().convert(&self.v_power.unwrap());
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            n.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            n.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            n.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            n.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            n.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            n.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }
        n.val *= cmp_val;
        n
    }
}

impl MulAssign<Value> for Value {
    fn mul_assign(&mut self, other:Value) {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }
        
        self.unit_map = 0;

        let mut cmp_val:f64 = other.val;
        for i in 0..30_usize {
            self.exp[i] += other.exp[i];
            let region:usize = 1<<i;

            let in_other:bool = region & other.unit_map != 0;
            let in_self:bool = self.unit_map & region != 0;
            let must_assign:bool = !in_self && in_other;

            if self.exp[i] != 0 {
                self.unit_map |= region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            self.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other.v_length.unwrap().convert(&self.v_length.unwrap());
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            self.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other.v_time.unwrap().convert(&self.v_time.unwrap());
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            self.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other.v_mass.unwrap().convert(&self.v_mass.unwrap());
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            self.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            self.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            self.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            self.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            self.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            self.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            self.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if must_assign {
                            self.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            self.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            self.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            self.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            self.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            self.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            self.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            self.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other.v_volume.unwrap().convert(&self.v_volume.unwrap());
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            self.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other.v_pressure.unwrap().convert(&self.v_pressure.unwrap());
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            self.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other.v_angle.unwrap().convert(&self.v_angle.unwrap());
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            self.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
                        }
            
                    }
                    FORCE_MAP => {
                        if must_assign {
                            self.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other.v_force.unwrap().convert(&self.v_force.unwrap());
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            self.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other.v_energy.unwrap().convert(&self.v_energy.unwrap());
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            self.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other.v_power.unwrap().convert(&self.v_power.unwrap());
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            self.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            self.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            self.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            self.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            self.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            self.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }
        self.val *= cmp_val;
    }
}

impl Div<Value> for Value {
    type Output = Value;
    fn div(self, other:Value) -> Value {
        let mut n:Value = self;
        n.unit_map = 0;

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        let mut cmp_val:f64 = other.val;
        for i in 0..30_usize {
            n.exp[i] = self.exp[i] - other.exp[i];
            let region:usize = 1<<i;
            let in_other:bool = region & other.unit_map != 0;
            let in_self:bool = self.unit_map & region != 0;
            let must_assign:bool = !in_self && in_other;

            if n.exp[i] != 0 {
                n.unit_map |= region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            n.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other.v_length.unwrap().convert(&self.v_length.unwrap());
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            n.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other.v_time.unwrap().convert(&self.v_time.unwrap());
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            n.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other.v_mass.unwrap().convert(&self.v_mass.unwrap());
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            n.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            n.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            n.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            n.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            n.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            n.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            n.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if must_assign {
                            n.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            n.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            n.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            n.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            n.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            n.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            n.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            n.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other.v_volume.unwrap().convert(&self.v_volume.unwrap());
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            n.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other.v_pressure.unwrap().convert(&self.v_pressure.unwrap());
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            n.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other.v_angle.unwrap().convert(&self.v_angle.unwrap());
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            n.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
                        }
            
                    }
                    FORCE_MAP => {
                        if must_assign {
                            n.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other.v_force.unwrap().convert(&self.v_force.unwrap());
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            n.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other.v_energy.unwrap().convert(&self.v_energy.unwrap());
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            n.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other.v_power.unwrap().convert(&self.v_power.unwrap());
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            n.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            n.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            n.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            n.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            n.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            n.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }
        n.val /= cmp_val;
        n
    }
}

impl DivAssign<Value> for Value {
    fn div_assign(&mut self, other:Value) {
        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP {
            if self.v_temperature != other.v_temperature {
                // Error cannot convert as part of larger unit
            }
        }

        self.unit_map = 0;

        let mut cmp_val:f64 = other.val;
        for i in 0..30_usize {
            self.exp[i] -= other.exp[i];
            let region:usize = 1<<i;
            let in_other:bool = region & other.unit_map != 0;
            let in_self:bool = self.unit_map & region != 0;
            let must_assign:bool = !in_self && in_other;

            if self.exp[i] != 0 {
                self.unit_map |= region;
            }

            if in_self && !in_other {
                continue;
            } else if in_other {
                match region {
                    LENGTH_MAP => {
                        if must_assign {
                            self.v_length = other.v_length;
                        } else if self.v_length != other.v_length {
                            cmp_val *= other.v_length.unwrap().convert(&self.v_length.unwrap());
                        }
                    }
                    TIME_MAP => {
                        if must_assign {
                            self.v_time = other.v_time;
                        } else if self.v_time != other.v_time {
                            cmp_val *= other.v_time.unwrap().convert(&self.v_time.unwrap());
                        }
                    }
                    MASS_MAP => {
                        if must_assign {
                            self.v_mass = other.v_mass;
                        } else if self.v_mass != other.v_mass {
                            cmp_val *= other.v_mass.unwrap().convert(&self.v_mass.unwrap());
                        }
                    }
                    ELECTRIC_CURRENT_MAP => {
                        if must_assign {
                            self.v_electric_current = other.v_electric_current;
                        } else if self.v_electric_current != other.v_electric_current {
                            cmp_val *= other.v_electric_current.unwrap().convert(&self.v_electric_current.unwrap());
                        }
                    }
                    ELECTRIC_CHARGE_MAP => {
                        if must_assign {
                            self.v_electric_charge = other.v_electric_charge;
                        } else if self.v_electric_charge != other.v_electric_charge {
                            cmp_val *= other.v_electric_charge.unwrap().convert(&self.v_electric_charge.unwrap());
                        }
                    }
                    ELECTRIC_POTENTIAL_MAP => {
                        if must_assign {
                            self.v_electric_potential = other.v_electric_potential;
                        } else if self.v_electric_potential != other.v_electric_potential {
                            cmp_val *= other.v_electric_potential.unwrap().convert(&self.v_electric_potential.unwrap());
                        }
                    }
                    ELECTRIC_CONDUCTANCE_MAP => {
                        if must_assign {
                            self.v_electric_conductance = other.v_electric_conductance;
                        } else if self.v_electric_conductance != other.v_electric_conductance {
                            cmp_val *= other.v_electric_conductance.unwrap().convert(&self.v_electric_conductance.unwrap());
                        }
                    }
                    CAPACITANCE_MAP => {
                        if must_assign {
                            self.v_capacitance = other.v_capacitance;
                        } else if self.v_capacitance != other.v_capacitance {
                            cmp_val *= other.v_capacitance.unwrap().convert(&self.v_capacitance.unwrap());
                        }
                    }
                    RESISTANCE_MAP => {
                        if must_assign {
                            self.v_resistance = other.v_resistance;
                        } else if self.v_resistance != other.v_resistance {
                            cmp_val *= other.v_resistance.unwrap().convert(&self.v_resistance.unwrap());
                        }
                    }
                    INDUCTANCE_MAP => {
                        if must_assign {
                            self.v_inductance = other.v_inductance;
                        } else if self.v_inductance != other.v_inductance { 
                            cmp_val *= other.v_inductance.unwrap().convert(&self.v_inductance.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_MAP => {
                        if must_assign {
                            self.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
                        if must_assign {
                            self.v_magnetic_flux_density = other.v_magnetic_flux_density;
                        } else if self.v_magnetic_flux_density != other.v_magnetic_flux_density {
                            cmp_val *= other.v_magnetic_flux_density.unwrap().convert(&self.v_magnetic_flux_density.unwrap());
                        }
                    }
                    TEMPERATURE_MAP => {
                        if must_assign {
                            self.v_temperature = other.v_temperature;
                        } else if self.v_temperature != other.v_temperature {
                            cmp_val = other.v_temperature.unwrap().convert(&self.v_temperature.unwrap(), cmp_val);
                        }
                    }
                    SUBSTANCE_MAP => {
                        if must_assign {
                            self.v_substance = other.v_substance;
                        } else if self.v_substance != other.v_substance {
                            cmp_val *= other.v_substance.unwrap().convert(&self.v_substance.unwrap());
                        }
                    }
                    LUMINOUS_INTENSITY_MAP => {
                        if must_assign {
                            self.v_luminous_flux_intensity = other.v_luminous_flux_intensity;
                        } else if self.v_luminous_flux_intensity != other.v_luminous_flux_intensity {
                            cmp_val *= other.v_luminous_flux_intensity.unwrap().convert(&self.v_luminous_flux_intensity.unwrap());
                        }
                    }
                    LUMINOUS_FLUX_MAP => {
                        if must_assign {
                            self.v_luminous_flux = other.v_luminous_flux;
                        } else if self.v_luminous_flux != other.v_luminous_flux {
                            cmp_val *= other.v_luminous_flux.unwrap().convert(&self.v_luminous_flux.unwrap());
                        }
                    }
                    ILLUMINANCE_MAP => {
                        if must_assign {
                            self.v_illuminance = other.v_illuminance;
                        } else if self.v_illuminance != other.v_illuminance {
                            cmp_val *= other.v_illuminance.unwrap().convert(&self.v_illuminance.unwrap());
                        }
                    }
                    VOLUME_MAP => {
                        if must_assign {
                            self.v_volume = other.v_volume;
                        } else if self.v_volume != other.v_volume {
                            cmp_val *= other.v_volume.unwrap().convert(&self.v_volume.unwrap());
                        }
                    }
                    PRESSURE_MAP => {
                        if must_assign {
                            self.v_pressure = other.v_pressure;
                        } else if self.v_pressure != other.v_pressure {
                            cmp_val *= other.v_pressure.unwrap().convert(&self.v_pressure.unwrap());
                        }
                    }
                    ANGLE_MAP => {
                        if must_assign {
                            self.v_angle = other.v_angle;
                        } else if self.v_angle != other.v_angle {
                            cmp_val *= other.v_angle.unwrap().convert(&self.v_angle.unwrap());
                        }
                    }
                    FREQUENCY_MAP => {
                        if must_assign {
                            self.v_frequency = other.v_frequency;
                        } else if self.v_frequency != other.v_frequency {
                            cmp_val *= other.v_frequency.unwrap().convert(&self.v_frequency.unwrap());
                        }
            
                    }
                    FORCE_MAP => {
                        if must_assign {
                            self.v_force = other.v_force;
                        } else if self.v_force != other.v_force {
                            cmp_val *= other.v_force.unwrap().convert(&self.v_force.unwrap());
                        }
                    }
                    ENERGY_MAP => {
                        if must_assign {
                            self.v_energy = other.v_energy;
                        } else if self.v_energy != other.v_energy {
                            cmp_val *= other.v_energy.unwrap().convert(&self.v_energy.unwrap());
                        }
                    }
                    POWER_MAP => {
                        if must_assign {
                            self.v_power = other.v_power;
                        } else if self.v_power != other.v_power {
                            cmp_val *= other.v_power.unwrap().convert(&self.v_power.unwrap());
                        }
                    }
                    RADIOACTIVITY_MAP => {
                        if must_assign {
                            self.v_radioactivity = other.v_radioactivity;
                        } else if self.v_radioactivity != other.v_radioactivity {
                            cmp_val *= other.v_radioactivity.unwrap().convert(&self.v_radioactivity.unwrap());
                        }
                    }
                    ABSORBED_DOSE_MAP => {
                        if must_assign {
                            self.v_ab_dose = other.v_ab_dose;
                        } else if self.v_ab_dose != other.v_ab_dose {
                            cmp_val *= other.v_ab_dose.unwrap().convert(&self.v_ab_dose.unwrap());
                        }
                    }
                    RADIOACTIVITY_EXPOSURE_MAP => {
                        if must_assign {
                            self.v_radioactivity_exposure = other.v_radioactivity_exposure;
                        } else if self.v_radioactivity_exposure != other.v_radioactivity_exposure {
                            cmp_val *= other.v_radioactivity_exposure.unwrap().convert(&self.v_radioactivity_exposure.unwrap());
                        }
                    }
                    CATALYTIC_ACTIVITY_MAP => {
                        if must_assign {
                            self.v_catalytic = other.v_catalytic;
                        } else if self.v_catalytic != other.v_catalytic {
                            cmp_val *= other.v_catalytic.unwrap().convert(&self.v_catalytic.unwrap());
                        }
                    }
                    SOUND_MAP => {
                        if must_assign {
                            self.v_sound = other.v_sound;
                        } else if self.v_sound != other.v_sound {
                            cmp_val *= other.v_sound.unwrap().convert(&self.v_sound.unwrap());
                        }
                    }
                    INFORMATION_MAP => {
                        if must_assign {
                            self.v_information = other.v_information;
                        } else if self.v_information != other.v_information {
                            cmp_val *= other.v_information.unwrap().convert(&self.v_information.unwrap());
                        }
                    }
                    _ => {
                        // error
                    }
                }
            }
        }
        self.val /= cmp_val;
    }
}

//impl Shr<Value> for Value {
//    type Output = Value;
//    fn shr(self, other:Value) -> Value {
        
//    }
//}

impl Value {
    pub fn new(val:f64, units:&str) -> Result<Value, V3Error> {
        let mut ret:Value = Value {
            val,
            unit_map:0,
            exp:[0;30],
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
            v_magnetic_flux_density : None
        };
        ret._create_unit(units)?;
        Ok(ret)
    }

    fn _radians(val:f64) -> Value {
        let mut ret:Value = Value {
            val,
            unit_map:ANGLE_MAP,
            exp:[0;30],
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
            v_magnetic_flux_density : None
        };
        ret.exp[ANGLE_INDEX] = 1;
        ret
    }

    pub fn convert(&mut self, other:&str) -> Result<(), V3Error> {
        let temp:Value = Value::new(0.0, other)?;
        self._convert(&temp)
    }

    fn _convert(&mut self, other:&Value) -> Result<(), V3Error> {

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
            return Err(V3Error::ValueConversionError("Error converting volume to cubic"));
        } else if self.unit_map == LENGTH_MAP && other.unit_map == VOLUME_MAP {
            if self.exp[LENGTH_INDEX] == 3 && other.exp[VOLUME_INDEX] == 1 {
                self.val *= f64::powf(self.v_length.unwrap().convert(&UnitLength::Meter(Metric::None)), 3.0);
                println!("{}", self.val);
                self.val *= self.v_length.unwrap().convert_liter(&other.v_volume.unwrap());
                self.exp[LENGTH_INDEX] = 0;
                self.exp[VOLUME_INDEX] = 1;
                self.unit_map = VOLUME_MAP;
                self.v_volume = other.v_volume;
                self.v_length = None;
                return Ok(());
            }
            return Err(V3Error::ValueConversionError("Error converting cubic to volume"));
        }

        if self.unit_map != other.unit_map {
            return Err(V3Error::ValueConversionError("Inequivalent unit types"));
        }

        // check against temperature 
        if self.unit_map & TEMPERATURE_MAP < self.unit_map {
            if self.v_temperature != other.v_temperature {
                return Err(V3Error::ValueConversionError("Temperature unit mismatch"));
            }
        } else if self.unit_map == TEMPERATURE_MAP {
            self.val = self.v_temperature.unwrap().convert(&other.v_temperature.unwrap(), self.val);
            self.v_temperature = other.v_temperature;
            return Ok(());
        }

        for i in 0..30_usize {
            if self.exp[i] != other.exp[i] {
                return Err(V3Error::ValueConversionError("Mismatched value exponents"));
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
                    MAGNETRIC_FLUX_MAP => {
                        tmp = self.v_magnetic_flux.unwrap().convert(&other.v_magnetic_flux.unwrap());
                        self.v_magnetic_flux = other.v_magnetic_flux;
                        tmp
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
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
                    _ => {
                        return Err(V3Error::UnknownError("Value conversion"));
                    }
                }, self.exp[i]);
            }
        }
        Ok(())
    }

    pub fn reduce(&mut self) {
        // converting to a force
        if self.unit_map == LENGTH_MAP | TIME_MAP | MASS_MAP {
            if self.exp[LENGTH_INDEX]*-2 == self.exp[TIME_INDEX] && self.exp[LENGTH_INDEX] == self.exp[MASS_INDEX] && self.exp[LENGTH_INDEX] == 1 {
                match self.v_length.unwrap() {
                    UnitLength::Meter(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_time.unwrap() {
                    UnitTime::Second(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_mass.unwrap() {
                    UnitMass::Gram(_) => {}
                    _ => {
                        return
                    }
                }
                self.val *= self.v_time.unwrap().convert(&UnitTime::Second(Metric::None));
                self.val *= self.v_length.unwrap().convert(&UnitLength::Meter(Metric::None));
                self.val *= self.v_mass.unwrap().convert(&UnitMass::Gram(Metric::Kilo));
                self.v_mass = None;
                self.v_length = None;
                self.v_time = None;
                self.v_force = Some(UnitForce::Newton(Metric::None));
                self.unit_map = FORCE_MAP;
                self.exp[LENGTH_INDEX] = 0;
                self.exp[TIME_INDEX] = 0;
                self.exp[MASS_INDEX] = 0;
                self.exp[FORCE_INDEX] = 1;
            }
        } else if self.unit_map == FORCE_MAP | LENGTH_MAP | TIME_MAP { // converting force to mass
            if self.exp[FORCE_INDEX]*2 == self.exp[TIME_INDEX] && self.exp[LENGTH_INDEX] == -1*self.exp[FORCE_INDEX] && self.exp[FORCE_INDEX] == 1 {
                match self.v_length.unwrap() {
                    UnitLength::Meter(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_time.unwrap() {
                    UnitTime::Second(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_force.unwrap() {
                    UnitForce::Newton(_) => {}
                    _ => {
                        return
                    }
                }
                self.val *= self.v_time.unwrap().convert(&UnitTime::Second(Metric::None));
                self.val /= self.v_length.unwrap().convert(&UnitLength::Meter(Metric::None));
                self.val *= self.v_force.unwrap().convert(&UnitForce::Newton(Metric::None));
                self.v_mass = Some(UnitMass::Gram(Metric::Kilo));
                self.v_length = None;
                self.v_time = None;
                self.v_force = None;
                self.unit_map = MASS_MAP;
                self.exp[LENGTH_INDEX] = 0;
                self.exp[TIME_INDEX] = 0;
                self.exp[MASS_INDEX] = 1;
                self.exp[FORCE_INDEX] = 0;
            }
        } else if self.unit_map == FORCE_MAP | MASS_MAP { // converting force to acceleration
            if self.exp[FORCE_INDEX] == 1 && self.exp[MASS_INDEX] == -1 {
                match self.v_force.unwrap() {
                    UnitForce::Newton(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_mass.unwrap() {
                    UnitMass::Gram(_) => {}
                    _ => {
                        return
                    }
                }
                self.val /= self.v_mass.unwrap().convert(&UnitMass::Gram(Metric::Kilo));
                self.v_mass = None;
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_force = None;
                self.unit_map = LENGTH_MAP | TIME_MAP;
                self.exp[LENGTH_INDEX] = 1;
                self.exp[TIME_INDEX] = -2;
                self.exp[MASS_INDEX] = 0;
                self.exp[FORCE_INDEX] = 0;
            }
        } else if self.unit_map == FORCE_MAP | TIME_MAP {
            if self.exp[LENGTH_INDEX]*-2 == self.exp[TIME_INDEX] && self.exp[LENGTH_INDEX] == self.exp[MASS_INDEX] {
                match self.v_time.unwrap() {
                    UnitTime::Second(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_force.unwrap() {
                    UnitForce::Newton(_) => {}
                    _ => {
                        return
                    }
                }
                self.val *= self.v_time.unwrap().convert(&UnitTime::Second(Metric::None));
                self.val *= self.v_force.unwrap().convert(&UnitForce::Newton(Metric::None));
                self.v_mass = Some(UnitMass::Gram(Metric::Kilo));
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_time = None;
                self.v_force = None;
                self.unit_map = LENGTH_MAP | MASS_MAP;
                self.exp[LENGTH_INDEX] = 1;
                self.exp[TIME_INDEX] = 0;
                self.exp[MASS_INDEX] = 1;
                self.exp[FORCE_INDEX] = 0;
            }
        } else if self.unit_map == FORCE_MAP | LENGTH_MAP {
            if self.exp[LENGTH_INDEX] == -1 && self.exp[FORCE_INDEX] == 1 {
                match self.v_length.unwrap() {
                    UnitLength::Meter(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_force.unwrap() {
                    UnitForce::Newton(_) => {}
                    _ => {
                        return
                    }
                }
                self.val /= self.v_length.unwrap().convert(&UnitLength::Meter(Metric::None));
                self.val *= self.v_force.unwrap().convert(&UnitForce::Newton(Metric::None));
                self.v_mass = Some(UnitMass::Gram(Metric::Kilo));
                self.v_length = None;
                self.v_time = Some(UnitTime::Second(Metric::None));
                self.v_force = None;
                self.unit_map = TIME_MAP | MASS_MAP;
                self.exp[LENGTH_INDEX] = 0;
                self.exp[TIME_INDEX] = -2;
                self.exp[MASS_INDEX] = 1;
                self.exp[FORCE_INDEX] = 0;
            }
        } else if self.unit_map == FORCE_MAP | TIME_MAP | MASS_MAP {
            if self.exp[TIME_INDEX] == -1 && self.exp[MASS_INDEX] == -1 && self.exp[FORCE_INDEX] == 1 {
                match self.v_force.unwrap() {
                    UnitForce::Newton(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_time.unwrap() {
                    UnitTime::Second(_) => {}
                    _ => {
                        return
                    }
                }
                match self.v_mass.unwrap() {
                    UnitMass::Gram(_) => {}
                    _ => {
                        return
                    }
                }
                self.val *= self.v_time.unwrap().convert(&UnitTime::Second(Metric::None));
                self.val *= self.v_force.unwrap().convert(&UnitForce::Newton(Metric::None));
                self.val /= self.v_mass.unwrap().convert(&UnitMass::Gram(Metric::Kilo));
                self.v_mass = None;
                self.v_length = Some(UnitLength::Meter(Metric::None));
                self.v_time = None;
                self.v_force = None;
                self.unit_map = LENGTH_MAP;
                self.exp[LENGTH_INDEX] = 1;
                self.exp[TIME_INDEX] = 0;
                self.exp[MASS_INDEX] = 0;
                self.exp[FORCE_INDEX] = 0;
            }
        }
    }

    pub fn inv(&mut self) {
        self.val = 1.0/self.val;
        for i in 0..self.exp.len() {
            self.exp[i] *= -1;
        }
    }

    pub fn to_radians(&mut self) {
        if self.unit_map != ANGLE_MAP && self.exp[ANGLE_INDEX] != 1 {
            // panic
        }
        self.val *= self.v_angle.unwrap().convert(&UnitAngle::Radian(Metric::None));
    }

    pub fn to_degrees(&mut self) {
        if self.unit_map != ANGLE_MAP && self.exp[ANGLE_INDEX] != 1 {
            // panic
        }
        self.val *= self.v_angle.unwrap().convert(&UnitAngle::Degree);
    }

    pub fn is_nan(&self) -> bool {
        self.val.is_nan()
    }

    pub fn is_finite(&self) -> bool {
        self.val.is_finite()
    }

    pub fn is_infinite(&self) -> bool {
        self.val.is_infinite()
    }

    pub fn is_normal(&self) -> bool {
        self.val.is_normal()
    }

    pub fn is_subnormal(&self)  -> bool {
        self.val.is_subnormal()
    }

    pub fn is_sign_positive(&self) -> bool {
        self.val.is_sign_positive()
    }

    pub fn is_sign_negative(&self) -> bool {
        self.val.is_sign_negative()
    }

    pub fn sqrt(&self) -> Value {
        let mut n:Value = *self;
        for i in 0..30_usize {
            n.exp[i] /= 2;
        }
        n.val = n.val.sqrt();
        n
    }

    pub fn cbrt(&self) -> Value {
        let mut n:Value = *self;
        for i in 0..30_usize {
            n.exp[i] /= 3;
        }
        n.val = n.val.cbrt();
        n
    }

    pub fn sin(&self) -> Value {
        Value::_radians(self.val.sin())
    }

    pub fn cos(&self) -> Value {
        Value::_radians(self.val.cos())
    }

    pub fn tan(&self) -> Value {
        Value::_radians(self.val.tan())
    }

    pub fn asin(&self) -> Value {
        Value::_radians(self.val.asin())
    }

    pub fn acos(&self) -> Value {
        Value::_radians(self.val.acos())
    }

    pub fn atan(&self) -> Value {
        Value::_radians(self.val.atan())
    }

    pub fn atan2(&self, other:&Value) -> Value {
        if other.unit_map != ANGLE_MAP && other.exp[ANGLE_INDEX] != 1 {
            // panic
        }
        let new_v:f64 = other.val * other.v_angle.unwrap().convert(&UnitAngle::Radian(Metric::None));
        Value::_radians(self.val.atan2(new_v))
    }

    pub fn is_length(&self) -> bool {
        if self.unit_map & LENGTH_MAP != self.unit_map {
            return false;
        }
        if self.exp[LENGTH_INDEX] != 1 {
            return false;
        }
        true
    }

    pub fn is_area(&self) -> bool {
        if self.unit_map & LENGTH_MAP != self.unit_map {
            return false;
        }
        if self.exp[LENGTH_INDEX] != 2 {
            return false;
        }
        true
    }

    pub fn is_volume(&self) -> bool {
        if self.unit_map & LENGTH_MAP == self.unit_map {
            if self.exp[LENGTH_INDEX] == 3 {
                return true;
            }
        } else if self.unit_map & VOLUME_MAP == self.unit_map {
            if self.exp[VOLUME_INDEX] == 1 {
                return true;
            }
        }
        false
    }

    pub fn is_velocity(&self) -> bool {
        if self.unit_map & (LENGTH_MAP | TIME_MAP) != self.unit_map {
            return false;
        }
        if self.exp[LENGTH_INDEX] != 1 || self.exp[TIME_INDEX] != -1 {
            return false;
        }
        true
    }

    pub fn is_acceleration(&self) -> bool {
        if self.unit_map & (LENGTH_MAP | TIME_MAP) != self.unit_map {
            return false;
        }
        if self.exp[LENGTH_INDEX] != 1 || self.exp[TIME_INDEX] != -2 {
            return false;
        }
        true
    }

    pub fn is_force(&self) -> bool {
        if self.unit_map & (MASS_MAP | LENGTH_MAP | TIME_MAP) == self.unit_map {
            if self.exp[LENGTH_INDEX] == 1 && self.exp[TIME_INDEX] == -2 && self.exp[MASS_INDEX] == 1 {
                return true;
            }
        } else if self.unit_map & FORCE_MAP == self.unit_map {
            if self.exp[FORCE_INDEX] == 1 {
                return true;
            }
        }
        false
    }

    pub fn is_momentum(&self) -> bool {
        if self.unit_map & (MASS_MAP | LENGTH_MAP | TIME_MAP) == self.unit_map {
            if self.exp[LENGTH_INDEX] == 1 && self.exp[TIME_INDEX] == -1 && self.exp[MASS_INDEX] == 1 {
                return true;
            }
        }
        false
    }

    pub fn is_time(&self) -> bool {
        if self.unit_map & TIME_MAP != self.unit_map {
            return false;
        }
        if self.exp[TIME_INDEX] != 1 {
            return false;
        }
        true
    }

    pub fn is_mass(&self) -> bool {
        if self.unit_map & MASS_MAP != self.unit_map {
            return false;
        }
        if self.exp[MASS_INDEX] != 1 {
            return false;
        }
        true
    }

    pub const fn const_earth_gravity() -> Value {
        let mut ret:Value = Value {
            val:VAL_EARTH_GRAV,
            unit_map:LENGTH_MAP | TIME_MAP,
            exp:[0;30],
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
            v_length: Some(UnitLength::Meter(Metric::None)),
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
            v_time: Some(UnitTime::Second(Metric::None)),
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[LENGTH_INDEX] = 1;
        ret.exp[TIME_INDEX] = -2;
        ret
    }

    pub const fn const_abs_zero() -> Value {
        let mut ret:Value = Value {
            val:VAL_ABS_ZERO,
            unit_map:TEMPERATURE_MAP,
            exp:[0;30],
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
            v_temperature: Some(UnitTemperature::Kelvin),
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[TEMPERATURE_INDEX] = 1;
        ret
    }

    pub const fn const_avagadros_number() -> Value {
        let mut ret:Value = Value {
            val:VAL_AVOGADROS,
            unit_map:SUBSTANCE_MAP,
            exp:[0;30],
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
            v_substance: Some(UnitSubstance::Mole(Metric::None)),
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[SUBSTANCE_INDEX] = -1;
        ret
    }

    pub const fn const_faraday() -> Value {
        let mut ret:Value = Value {
            val:VAL_FARADAY,
            unit_map:SUBSTANCE_MAP | ELECTRIC_CHARGE_MAP,
            exp:[0;30],
            v_ab_dose: None,
            v_angle: None,
            v_capacitance: None,
            v_catalytic: None,
            v_electric_charge: Some(UnitElectricCharge::Coulomb(Metric::None)),
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
            v_substance: Some(UnitSubstance::Mole(Metric::None)),
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[ELECTRIC_CHARGE_MAP] = 1;
        ret.exp[SUBSTANCE_INDEX] = -1;
        ret
    }

    pub const fn const_atomic_mass() -> Value {
        let mut ret:Value = Value {
            val:VAL_ATOMIC_MASS,
            unit_map:MASS_MAP,
            exp:[0;30],
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
            v_mass: Some(UnitMass::Gram(Metric::Kilo)),
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
            v_magnetic_flux_density : None
        };
        ret.exp[MASS_INDEX] = 1;
        ret
    }

    pub const fn const_molar_gas() -> Value {
        let mut ret:Value = Value {
            val:VAL_MOLAR_GAS,
            unit_map:SUBSTANCE_MAP | TEMPERATURE_MAP | ENERGY_MAP,
            exp:[0;30],
            v_ab_dose: None,
            v_angle: None,
            v_capacitance: None,
            v_catalytic: None,
            v_electric_charge: None,
            v_electric_conductance: None,
            v_electric_current: None,
            v_electric_potential: None,
            v_energy: Some(UnitEnergy::Joule(Metric::None)),
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
            v_substance: Some(UnitSubstance::Mole(Metric::None)),
            v_temperature: Some(UnitTemperature::Kelvin),
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[ENERGY_INDEX] = 1;
        ret.exp[TEMPERATURE_INDEX] = -1;
        ret.exp[SUBSTANCE_INDEX] = -1;
        ret
    }

    pub const fn const_coulomb() -> Value {
        let mut ret:Value = Value {
            val:VAL_COULOMBS,
            unit_map:SUBSTANCE_MAP,
            exp:[0;30],
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
            v_substance: Some(UnitSubstance::Mole(Metric::None)),
            v_temperature: None,
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[SUBSTANCE_INDEX] = -1;
        ret
    }

    pub const fn const_speed_of_light() -> Value {
        let mut ret:Value = Value {
            val:VAL_LIGHT_SPEED,
            unit_map:TIME_MAP | LENGTH_MAP,
            exp:[0;30],
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
            v_length: Some(UnitLength::Meter(Metric::None)),
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
            v_time: Some(UnitTime::Second(Metric::None)),
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[LENGTH_INDEX] = 1;
        ret.exp[TIME_INDEX] = -2;
        ret
    }

    pub const fn const_boltzmann() -> Value {
        let mut ret:Value = Value {
            val:VAL_BOLTZMANN,
            unit_map:ENERGY_MAP | TEMPERATURE_MAP,
            exp:[0;30],
            v_ab_dose: None,
            v_angle: None,
            v_capacitance: None,
            v_catalytic: None,
            v_electric_charge: None,
            v_electric_conductance: None,
            v_electric_current: None,
            v_electric_potential: None,
            v_energy: Some(UnitEnergy::Joule(Metric::None)),
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
            v_temperature: Some(UnitTemperature::Kelvin),
            v_time: None,
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[ENERGY_INDEX] = 1;
        ret.exp[TEMPERATURE_INDEX] = -1;
        ret
    }

    pub const fn const_newtonian_gravitation() -> Value {
        let mut ret:Value = Value {
            val:VAL_NEWTONIAN_GRAVITATION,
            unit_map:LENGTH_MAP | MASS_MAP | TIME_MAP,
            exp:[0;30],
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
            v_length: Some(UnitLength::Meter(Metric::None)),
            v_luminous_flux: None,
            v_luminous_flux_intensity: None,
            v_mass: Some(UnitMass::Gram(Metric::Kilo)),
            v_power: None,
            v_pressure: None,
            v_radioactivity: None,
            v_radioactivity_exposure: None,
            v_resistance: None,
            v_sound: None,
            v_substance: None,
            v_temperature: None,
            v_time: Some(UnitTime::Second(Metric::None)),
            v_volume: None,
            v_magnetic_flux : None,
            v_magnetic_flux_density : None
        };
        ret.exp[LENGTH_INDEX] = 3;
        ret.exp[TIME_INDEX] = -2;
        ret.exp[MASS_INDEX] = -1;
        ret
    }

    fn _create_unit(&mut self, units:&str) -> Result<(), V3Error>{
        let tokens:(Vec<String>, Vec<String>) = self._get_tokens(units, false)?;

        // do the numors first
        for t in tokens.0 {
            let mut expon:i32 = 1;
            let temp_split:Vec<&str> = t.split('^').collect();
            if temp_split.len() > 1 {
                expon = temp_split[1].parse::<i32>().unwrap();
            }
            self._parse_units(temp_split[0], expon)?;
        }

        // now the denoms
        for t in tokens.1 {
            let mut expon:i32 = -1;
            let temp_split:Vec<&str> = t.split('^').collect();
            if temp_split.len() > 1 {
                expon *= temp_split[1].parse::<i32>().unwrap();
            }
            self._parse_units(temp_split[0], expon)?;
        }

        Ok(())
    }

    fn _get_tokens(&self, block:&str, do_denom:bool) -> Result<(Vec<String>, Vec<String>), V3Error> {
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
            let c:char = block.chars().nth(index).unwrap();
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
                        let mut ret:(Vec<String>, Vec<String>) = self._get_tokens(&block[start_index..end_index], found_divisor)?;
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

    fn _parse_units(&mut self, unit:&str, exp:i32) -> Result<(), V3Error> {
        let l:usize = unit.chars().count();
        if l == 0 {
            return Ok(());
        }

        // first match it against known unique strings
        match unit {
            "mph" => {
                if exp != 1 || exp != -1 {
                    return Err(V3Error::ParsingError("MPH exponent"));
                }
                self.v_length = Some(UnitLength::Mile);
                self.exp[LENGTH_INDEX] = exp;
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = -1*exp;
                self.unit_map = LENGTH_MAP | TIME_MAP;
                return Ok(());
            }
            "kph" => {
                if exp != 1 || exp != -1 {
                    // error
                }
                self.v_length = Some(UnitLength::Meter(Metric::Kilo));
                self.exp[LENGTH_INDEX] = exp;
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = -1*exp;
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
            "h" | "hour" | "hours" => {
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
            self._get_quadrouple_letter(unit, exp, Metric::None)?;
        } else if l == 5 {
            self._get_pentuple_letter(unit, exp, Metric::None)?;
        } else {
            return Err(V3Error::UnsupportedUnit(format!("Unit {} exceeds parsing bounds", unit)));
        }
        Ok(())
    }

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
                self.exp[MAGNETRIC_FLUX_DENSITY_INDEX] = exp;
                self.unit_map |= MAGNETRIC_FLUX_DENSITY_MAP;
            }
            "N" => {
                self.v_force = Some(UnitForce::Newton(m));
                self.exp[FORCE_INDEX] = exp;
                self.unit_map |= FORCE_MAP;
            }
            "K" => {
                // if m, error
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
                return Err(V3Error::UnsupportedUnit(format!("Unsupported unit: {}", unit)));
            }
        }
        Ok(())
    }

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
                self.exp[MAGNETRIC_FLUX_INDEX] = exp;
                self.unit_map |= MAGNETRIC_FLUX_MAP;
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
            _ => {
                if m != Metric::None {
                    return Err(V3Error::UnsupportedUnit(format!("Unsupported unit: {}", unit)));
                }
                match self._get_metric(&unit.chars().next().unwrap()) {
                    Ok(new_m) => self._get_single_letter(&unit[1..], exp, new_m)?,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

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
                    return Err(V3Error::UnsupportedUnit(format!("Unsupported unit: {}", unit)));
                }
                match self._get_metric(&unit.chars().next().unwrap()) {
                    Ok(new_m) => self._get_double_letter(&unit[1..], exp, new_m)?,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    fn _get_quadrouple_letter(&mut self, unit:&str, exp:i32, m:Metric) -> Result<(), V3Error> {

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
                    return Err(V3Error::UnsupportedUnit(format!("Unsupported unit: {}", unit)));
                }
                match self._get_metric(&unit.chars().next().unwrap()) {
                    Ok(new_m) => self._get_triple_letter(&unit[1..], exp, new_m)?,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    fn _get_pentuple_letter(&mut self, unit:&str, exp:i32, m:Metric) -> Result<(), V3Error> {

        if let Some(da) = unit.strip_prefix("da") {
            return self._get_triple_letter(da, exp, Metric::Deca);
        }

        if m != Metric::None {
            return Err(V3Error::UnsupportedUnit(format!("Unsupported unit: {}", unit)));
        }
        match self._get_metric(&unit.chars().next().unwrap()) {
            Ok(new_m) => self._get_quadrouple_letter(&unit[1..], exp, new_m),
            Err(e) => {
                return Err(e);
            }
        }
    }

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
            'u' | 'μ' => Ok(Metric::Milli),
            'n' => Ok(Metric::Nano),
            'p' => Ok(Metric::Pico),
            'f' => Ok(Metric::Femto),
            'a' => Ok(Metric::Atto),
            'z' => Ok(Metric::Zepto),
            'y' => Ok(Metric::Yocto),
            _ => {
                Err(V3Error::UnsupportedMetric(format!("Unsupported metric: {}", unit)))
            }
        }
    }

    // check if we are equivalent units
    fn __equivalent(&self, other:&Value) -> bool {

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

        for i in 0..30_usize {
            if self.exp[i] != other.exp[i] {
                return false;
            }
        }

        true
    }

    // only if checking if the types are the same
    fn __equal(&self, other:&Value) -> bool {
        if self.unit_map != other.unit_map {
            return false;
        }
        for i in 0..30_usize {
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
                    MAGNETRIC_FLUX_MAP => {
                        if self.v_magnetic_flux.unwrap() != other.v_magnetic_flux.unwrap() {
                            return false;
                        }
                    }
                    MAGNETRIC_FLUX_DENSITY_MAP => {
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
                    _ => {
                        // error
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use crate::units::Metric;
    use crate::units::UnitMass;
    use crate::units::UnitSubstance;
    use crate::units::UnitLength;
    use crate::constants::{MASS_INDEX, SUBSTANCE_INDEX};
    use crate::constants::{MASS_MAP, SUBSTANCE_MAP};
    use crate::values::Value;

    /* This value is used for exponent equivalency
     * When a unit's or values's exponent needs to be compared to another 
     * units exponent, this 'cut off' is where exp are deemed 'equal'. 
     */ 
    const CUTOFF:f64 = 0.0000001;
    
    macro_rules! assert_apx {
        ($x:expr, $y:expr, $d:expr) => {
            if $x.__equivalent(&$y) {
                if f64::max($x.val, $y.val) - f64::min($x.val, $y.val) > %d {panic!();}
            } else {
                panic!();
            }
        };
        ($x:expr, $y:expr) => {
            if $x.__equal(&$y) {
                if f64::max($x.val, $y.val) - f64::min($x.val, $y.val) > CUTOFF {panic!();}
            } else {
                panic!();
            }
        }
    }

    #[test]
    fn value_create_1(){
        let v:Value = Value::new(4.5, "kg/mol").unwrap();
        assert_eq!(v.unit_map, MASS_MAP | SUBSTANCE_MAP);
        assert_eq!(v.val, 4.5);
        assert_eq!(v.v_mass, Some(UnitMass::Gram(Metric::Kilo)));
        assert_eq!(v.v_substance, Some(UnitSubstance::Mole(Metric::None)));
        assert_eq!(v.exp[MASS_INDEX], 1);
        assert_eq!(v.exp[SUBSTANCE_INDEX], -1);
    }

    #[test]
    fn value_1(){
        let result:Value = Value::new(3.4, &"kg").unwrap();
        assert_eq!(result.unit_map, MASS_MAP);
        assert_eq!(result.val, 3.4);
    }

    #[test]
    fn value_2(){
        let mut e1:Value = Value::new(5.0, "bytes").unwrap();
        let e2:Value = Value::new(1.0, "Gb").unwrap();
        let expected:Value = Value::new(0.000000004656613, "Gb").unwrap();
        let _ = e1._convert(&e2).unwrap();

        if f64::max(e1.val, expected.val) - f64::min(e1.val, expected.val) > 0.000001 {
            panic!();
        }
    }

    #[test]
    fn value_3(){
        let v1:Value = Value::new(3.4, "kg").unwrap();
        let v2:Value = Value::new(3.4, "kg").unwrap();
        let result:Value = v1/v2;
        assert_eq!(result.unit_map, 0);
        assert_eq!(result.val, 1.0);
    }

    #[test]
    fn value_4(){
        let v1:Value = Value::new(1.0, "kg^-1").unwrap();
        let v2:Value = Value::new(2.0, "kg").unwrap();
        let result:Value = v1*v2;
        assert_eq!(result.unit_map, 0);
        assert_eq!(result.val, 2.0);
    }

    #[test]
    fn value_5(){
        let mut v1:Value = Value::new(1.0, "kg").unwrap();
        v1+=1.0_f64;
        assert_eq!(v1.val, 2.0);
    }

    #[test]
    fn value_6(){
        let mut v1:Value = Value::new(5.0, "g").unwrap();
        v1-=3.5;
        assert_eq!(v1.val, 1.5);
    }

    #[test]
    fn value_7(){
        let mut v1:Value = Value::new(7.0, "g").unwrap();
        v1*=3.0;
        assert_eq!(v1.val, 21.0);
    }

    #[test]
    fn value_8(){
        let mut v1:Value = Value::new(8.0, "m").unwrap();
        v1/=2.0;
        assert_eq!(v1.val, 4.0);
    }

    #[test]
    fn value_reduction_1(){
        let mut v1:Value = Value::new(247.0, "g").unwrap();
        let v2:Value = Value::const_earth_gravity();
        let ret:Value = Value::new(2.42224255, "N").unwrap();

        v1 *= v2;
        v1.reduce();
        assert_eq!(v1, ret);
        assert_eq!(v1.is_force(), true);
    }

    #[test]
    fn value_reduction_2(){
        let v1:Value = Value::new(0.247, "kg").unwrap();
        let v2:Value = Value::const_earth_gravity();
        let mut v3:Value = Value::new(2.42224255, "N").unwrap();

        v3 /= v2;
        v3.reduce();
        assert_apx!(v3, v1);
        assert_eq!(v3.is_mass(), true);
    }

        #[test]
    fn value_reduction_3(){
        let v1:Value = Value::new(247000.0, "mg").unwrap();
        let v2:Value = Value::const_earth_gravity();
        let mut v3:Value = Value::new(2.42224255, "N").unwrap();

        v3 /= v1;
        println!("{:?}", v3);
        v3.reduce();
        println!("{:?}", v3);
        assert_apx!(v3, v2);
        assert_eq!(v3.is_acceleration(), true);
    }

    #[test]
    fn value_9(){
        let v1:Value = value!(5, "ft/s^2");
        let v2:Value = value!(1.524, "m/s^2");
        let v3:Value = value!(0.00042333333333333334, "m/min^2");
        assert_eq!((v1 >> "m/s^2").unwrap(), v2);
        assert_eq!((v1 >> "m/min^2").unwrap(), v3);
    }
}