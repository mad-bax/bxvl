use std::ops::{Shr, ShrAssign};

use crate::constants::*;
use crate::errors::V3Error;
use crate::units::Convert;
use crate::units::UnitAbsorbedDose;
use crate::units::UnitIlluminance;
use crate::units::UnitInformation;
use crate::units::UnitLuminousFlux;
use crate::units::UnitLuminousIntensity;
use crate::units::UnitMagneticFlux;
use crate::units::UnitMagneticFluxDensity;
use crate::units::UnitMass;
use crate::units::UnitPower;
use crate::units::UnitPressure;
use crate::units::UnitRadioactivity;
use crate::units::UnitRadioactivityExposure;
use crate::units::UnitSound;
use crate::units::UnitSubstance;
use crate::units::UnitTemperature;
use crate::units::UnitTime;
use crate::units::UnitVolume;
use crate::values::Value;

impl Shr<Value> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: Value) -> Self::Output {
        if self.__equivalent(&other) {
            let mut ret: Value = self;
            ret._convert(&other)?;
            return Ok(ret);
        }
        Err(V3Error::ValueConversionError("[shr] Incompatible types"))
    }
}

impl Shr<&str> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: &str) -> Self::Output {
        let n: Value = Value::new(1.0, other)?;
        self >> n
    }
}

impl Shr<String> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: String) -> Self::Output {
        let n: Value = Value::new(1.0, other.as_str())?;
        self >> n
    }
}

impl Shr<UnitAbsorbedDose> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitAbsorbedDose) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & ABSORBED_DOSE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_ab_dose
            .unwrap()
            .convert(&other)
            .powi(self.exp[ABSORBED_DOSE_INDEX]);
        n.v_ab_dose = Some(other);
        Ok(n)
    }
}

impl Shr<UnitIlluminance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitIlluminance) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & ILLUMINANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_illuminance
            .unwrap()
            .convert(&other)
            .powi(self.exp[ILLUMINANCE_INDEX]);
        n.v_illuminance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitInformation> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitInformation) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & INFORMATION_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_information
            .unwrap()
            .convert(&other)
            .powi(self.exp[INFORMATION_INDEX]);
        n.v_information = Some(other);
        Ok(n)
    }
}

impl Shr<UnitLuminousFlux> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitLuminousFlux) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & LUMINOUS_FLUX_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_luminous_flux
            .unwrap()
            .convert(&other)
            .powi(self.exp[LUMINOUS_FLUX_INDEX]);
        n.v_luminous_flux = Some(other);
        Ok(n)
    }
}

impl Shr<UnitLuminousIntensity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitLuminousIntensity) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & LUMINOUS_INTENSITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_luminous_flux_intensity
            .unwrap()
            .convert(&other)
            .powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
        n.v_luminous_flux_intensity = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMagneticFlux> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitMagneticFlux) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & MAGNETIC_FLUX_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_magnetic_flux
            .unwrap()
            .convert(&other)
            .powi(self.exp[MAGNETIC_FLUX_INDEX]);
        n.v_magnetic_flux = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMagneticFluxDensity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitMagneticFluxDensity) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & MAGNETIC_FLUX_DENSITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_magnetic_flux_density
            .unwrap()
            .convert(&other)
            .powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
        n.v_magnetic_flux_density = Some(other);
        Ok(n)
    }
}

impl Shr<UnitMass> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitMass) -> Self::Output {
        let mut n: Value = self;
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
    fn shr(self, other: UnitPower) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & POWER_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_power
            .unwrap()
            .convert(&other)
            .powi(self.exp[POWER_INDEX]);
        n.v_power = Some(other);
        Ok(n)
    }
}

impl Shr<UnitPressure> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitPressure) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & PRESSURE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_pressure
            .unwrap()
            .convert(&other)
            .powi(self.exp[PRESSURE_INDEX]);
        n.v_pressure = Some(other);
        Ok(n)
    }
}

impl Shr<UnitRadioactivity> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitRadioactivity) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & RADIOACTIVITY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_radioactivity
            .unwrap()
            .convert(&other)
            .powi(self.exp[RADIOACTIVITY_INDEX]);
        n.v_radioactivity = Some(other);
        Ok(n)
    }
}

impl Shr<UnitRadioactivityExposure> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitRadioactivityExposure) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & RADIOACTIVITY_EXPOSURE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_radioactivity_exposure
            .unwrap()
            .convert(&other)
            .powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
        n.v_radioactivity_exposure = Some(other);
        Ok(n)
    }
}

impl Shr<UnitSound> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitSound) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & SOUND_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_sound
            .unwrap()
            .convert(&other)
            .powi(self.exp[SOUND_INDEX]);
        n.v_sound = Some(other);
        Ok(n)
    }
}

impl Shr<UnitSubstance> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitSubstance) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & SUBSTANCE_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_substance
            .unwrap()
            .convert(&other)
            .powi(self.exp[SUBSTANCE_INDEX]);
        n.v_substance = Some(other);
        Ok(n)
    }
}

impl Shr<UnitTemperature> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitTemperature) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & TEMPERATURE_MAP != TEMPERATURE_MAP {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        if self.exp[TEMPERATURE_INDEX] != 1 && self.exp[TEMPERATURE_INDEX] != -1 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val = n
            .v_temperature
            .unwrap()
            .convert(&other, n.val)
            .powi(self.exp[TEMPERATURE_INDEX]);
        n.v_temperature = Some(other);
        Ok(n)
    }
}

impl Shr<UnitTime> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitTime) -> Self::Output {
        let mut n: Value = self;
        if n.unit_map & FREQUENCY_MAP == FREQUENCY_MAP && n.exp[FREQUENCY_INDEX] == 1 {
            n.val *= n.v_frequency.unwrap().convert(&other);
            n.v_frequency = None;
            n.v_time = Some(other);
            n.unit_map = TIME_MAP;
            n.exp[TIME_INDEX] = -1;
            n.exp[FREQUENCY_INDEX] = 0;
            return Ok(n);
        } else if self.unit_map & TIME_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n.v_time.unwrap().convert(&other).powi(self.exp[TIME_INDEX]);
        n.v_time = Some(other);
        Ok(n)
    }
}

impl Shr<UnitVolume> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitVolume) -> Self::Output {
        let mut n: Value = self;
        if self.unit_map & LENGTH_MAP == LENGTH_MAP && self.exp[LENGTH_INDEX] == 3 {
            n.val = self.val * self.v_length.unwrap().convert(&other);
            n.v_volume = Some(other);
            n.v_length = None;
            n.unit_map = VOLUME_MAP;
            n.exp[VOLUME_INDEX] = 1;
            n.exp[LENGTH_INDEX] = 0;
            return Ok(n);
        } else if self.unit_map & VOLUME_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_volume
            .unwrap()
            .convert(&other)
            .powi(self.exp[VOLUME_INDEX]);
        n.v_volume = Some(other);
        Ok(n)
    }
}

impl ShrAssign<Value> for Value {
    fn shr_assign(&mut self, other: Value) {
        if self.__equivalent(&other) {
            match self._convert(&other) {
                Ok(_) => {}
                Err(_) => panic!("[shr_assign] Incompatible value types: {}, {}", self, other),
            }
        } else {
            panic!("[shr_assign] Incompatible value types: {}, {}", self, other);
        }
    }
}

impl ShrAssign<&str> for Value {
    fn shr_assign(&mut self, other: &str) {
        let n: Value = match Value::new(1.0, other) {
            Ok(t) => t,
            Err(_) => panic!("[shr_assign] Incompatible value types"),
        };
        *self >>= n;
    }
}

impl ShrAssign<String> for Value {
    fn shr_assign(&mut self, other: String) {
        let n: Value = match Value::new(1.0, other.as_str()) {
            Ok(t) => t,
            Err(_) => panic!("[shr_assign] Incompatible value types"),
        };
        *self >>= n;
    }
}

impl ShrAssign<UnitAbsorbedDose> for Value {
    fn shr_assign(&mut self, other: UnitAbsorbedDose) {
        if self.unit_map & ABSORBED_DOSE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_ab_dose
            .unwrap()
            .convert(&other)
            .powi(self.exp[ABSORBED_DOSE_INDEX]);
        self.v_ab_dose = Some(other);
    }
}

impl ShrAssign<UnitIlluminance> for Value {
    fn shr_assign(&mut self, other: UnitIlluminance) {
        if self.unit_map & ILLUMINANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_illuminance
            .unwrap()
            .convert(&other)
            .powi(self.exp[ILLUMINANCE_INDEX]);
        self.v_illuminance = Some(other);
    }
}

impl ShrAssign<UnitInformation> for Value {
    fn shr_assign(&mut self, other: UnitInformation) {
        if self.unit_map & INFORMATION_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_information
            .unwrap()
            .convert(&other)
            .powi(self.exp[INFORMATION_INDEX]);
        self.v_information = Some(other);
    }
}

impl ShrAssign<UnitLuminousFlux> for Value {
    fn shr_assign(&mut self, other: UnitLuminousFlux) {
        if self.unit_map & LUMINOUS_FLUX_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_luminous_flux
            .unwrap()
            .convert(&other)
            .powi(self.exp[LUMINOUS_FLUX_INDEX]);
        self.v_luminous_flux = Some(other);
    }
}

impl ShrAssign<UnitLuminousIntensity> for Value {
    fn shr_assign(&mut self, other: UnitLuminousIntensity) {
        if self.unit_map & LUMINOUS_INTENSITY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_luminous_flux_intensity
            .unwrap()
            .convert(&other)
            .powi(self.exp[LUMINOUS_INTENSITY_INDEX]);
        self.v_luminous_flux_intensity = Some(other);
    }
}

impl ShrAssign<UnitMagneticFlux> for Value {
    fn shr_assign(&mut self, other: UnitMagneticFlux) {
        if self.unit_map & MAGNETIC_FLUX_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_magnetic_flux
            .unwrap()
            .convert(&other)
            .powi(self.exp[MAGNETIC_FLUX_INDEX]);
        self.v_magnetic_flux = Some(other);
    }
}

impl ShrAssign<UnitMagneticFluxDensity> for Value {
    fn shr_assign(&mut self, other: UnitMagneticFluxDensity) {
        if self.unit_map & MAGNETIC_FLUX_DENSITY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_magnetic_flux_density
            .unwrap()
            .convert(&other)
            .powi(self.exp[MAGNETIC_FLUX_DENSITY_INDEX]);
        self.v_magnetic_flux_density = Some(other);
    }
}

impl ShrAssign<UnitMass> for Value {
    fn shr_assign(&mut self, other: UnitMass) {
        if self.unit_map & MASS_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_mass
            .unwrap()
            .convert(&other)
            .powi(self.exp[MASS_INDEX]);
        self.v_mass = Some(other);
    }
}

impl ShrAssign<UnitPower> for Value {
    fn shr_assign(&mut self, other: UnitPower) {
        if self.unit_map & POWER_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_power
            .unwrap()
            .convert(&other)
            .powi(self.exp[POWER_INDEX]);
        self.v_power = Some(other);
    }
}

impl ShrAssign<UnitPressure> for Value {
    fn shr_assign(&mut self, other: UnitPressure) {
        if self.unit_map & PRESSURE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_pressure
            .unwrap()
            .convert(&other)
            .powi(self.exp[PRESSURE_INDEX]);
        self.v_pressure = Some(other);
    }
}

impl ShrAssign<UnitRadioactivity> for Value {
    fn shr_assign(&mut self, other: UnitRadioactivity) {
        if self.unit_map & RADIOACTIVITY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_radioactivity
            .unwrap()
            .convert(&other)
            .powi(self.exp[RADIOACTIVITY_INDEX]);
        self.v_radioactivity = Some(other);
    }
}

impl ShrAssign<UnitRadioactivityExposure> for Value {
    fn shr_assign(&mut self, other: UnitRadioactivityExposure) {
        if self.unit_map & RADIOACTIVITY_EXPOSURE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_radioactivity_exposure
            .unwrap()
            .convert(&other)
            .powi(self.exp[RADIOACTIVITY_EXPOSURE_INDEX]);
        self.v_radioactivity_exposure = Some(other);
    }
}

impl ShrAssign<UnitSound> for Value {
    fn shr_assign(&mut self, other: UnitSound) {
        if self.unit_map & SOUND_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_sound
            .unwrap()
            .convert(&other)
            .powi(self.exp[SOUND_INDEX]);
        self.v_sound = Some(other);
    }
}

impl ShrAssign<UnitSubstance> for Value {
    fn shr_assign(&mut self, other: UnitSubstance) {
        if self.unit_map & SUBSTANCE_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_substance
            .unwrap()
            .convert(&other)
            .powi(self.exp[SUBSTANCE_INDEX]);
        self.v_substance = Some(other);
    }
}

impl ShrAssign<UnitTemperature> for Value {
    fn shr_assign(&mut self, other: UnitTemperature) {
        if self.unit_map & TEMPERATURE_MAP != TEMPERATURE_MAP {
            panic!("[shr_assign] Incompatible value types");
        }
        if self.exp[TEMPERATURE_INDEX] != 1 && self.exp[TEMPERATURE_INDEX] != -1 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val = self
            .v_temperature
            .unwrap()
            .convert(&other, self.val)
            .powi(self.exp[TEMPERATURE_INDEX]);
        self.v_temperature = Some(other);
    }
}

impl ShrAssign<UnitTime> for Value {
    fn shr_assign(&mut self, other: UnitTime) {
        if self.unit_map & FREQUENCY_MAP == FREQUENCY_MAP && self.exp[FREQUENCY_INDEX] == 1 {
            self.val *= self.v_frequency.unwrap().convert(&other);
            self.v_frequency = None;
            self.v_time = Some(other);
            self.unit_map = TIME_MAP;
            self.exp[TIME_INDEX] = -1;
            self.exp[FREQUENCY_INDEX] = 0;
            return;
        } else if self.unit_map & TIME_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_time
            .unwrap()
            .convert(&other)
            .powi(self.exp[TIME_INDEX]);
        self.v_time = Some(other);
    }
}

impl ShrAssign<UnitVolume> for Value {
    fn shr_assign(&mut self, other: UnitVolume) {
        if self.unit_map & LENGTH_MAP == LENGTH_MAP && self.exp[LENGTH_INDEX] == 3 {
            self.val *= self.v_length.unwrap().convert(&other);
            self.v_volume = Some(other);
            self.v_length = None;
            self.unit_map = VOLUME_MAP;
            self.exp[VOLUME_INDEX] = 1;
            self.exp[LENGTH_INDEX] = 0;
            return;
        } else if self.unit_map & VOLUME_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }

        self.val *= self
            .v_volume
            .unwrap()
            .convert(&other)
            .powi(self.exp[VOLUME_INDEX]);
        self.v_volume = Some(other);
    }
}
