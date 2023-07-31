use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use crate::values::Value;
use crate::constants::*;

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

impl Add<f32> for Value {
    type Output = Value;
    fn add(self, rhs:f32) -> Value {
        let mut n:Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<f32> for Value {
    fn add_assign(&mut self, rhs:f32) {
        self.val += rhs as f64;
    }
}

impl Sub<f32> for Value {
    type Output = Value;
    fn sub(self, rhs:f32) -> Value {
        let mut n:Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<f32> for Value {
    fn sub_assign(&mut self, rhs:f32) {
        self.val -= rhs as f64;
    }
}

impl Mul<f32> for Value {
    type Output = Value;
    fn mul(self, rhs:f32) -> Value {
        let mut n:Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<f32> for Value {
    fn mul_assign(&mut self, rhs:f32) {
        self.val *= rhs as f64;
    }
}

impl Div<f32> for Value {
    type Output = Value;
    fn div(self, rhs: f32) -> Value { 
        let mut n:Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<f32> for Value {
    fn div_assign(&mut self, rhs:f32) {
        self.val /= rhs as f64;
    }
}

impl Add<usize> for Value {
    type Output = Value;
    fn add(self, rhs:usize) -> Value {
        let mut n:Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<usize> for Value {
    fn add_assign(&mut self, rhs:usize) {
        self.val += rhs as f64;
    }
}

impl Sub<usize> for Value {
    type Output = Value;
    fn sub(self, rhs:usize) -> Value {
        let mut n:Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<usize> for Value {
    fn sub_assign(&mut self, rhs:usize) {
        self.val -= rhs as f64;
    }
}

impl Mul<usize> for Value {
    type Output = Value;
    fn mul(self, rhs:usize) -> Value {
        let mut n:Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<usize> for Value {
    fn mul_assign(&mut self, rhs:usize) {
        self.val *= rhs as f64;
    }
}

impl Div<usize> for Value {
    type Output = Value;
    fn div(self, rhs: usize) -> Value { 
        let mut n:Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<usize> for Value {
    fn div_assign(&mut self, rhs:usize) {
        self.val /= rhs as f64;
    }
}

impl Add<isize> for Value {
    type Output = Value;
    fn add(self, rhs:isize) -> Value {
        let mut n:Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<isize> for Value {
    fn add_assign(&mut self, rhs:isize) {
        self.val += rhs as f64;
    }
}

impl Sub<isize> for Value {
    type Output = Value;
    fn sub(self, rhs:isize) -> Value {
        let mut n:Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<isize> for Value {
    fn sub_assign(&mut self, rhs:isize) {
        self.val -= rhs as f64;
    }
}

impl Mul<isize> for Value {
    type Output = Value;
    fn mul(self, rhs:isize) -> Value {
        let mut n:Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<isize> for Value {
    fn mul_assign(&mut self, rhs:isize) {
        self.val *= rhs as f64;
    }
}

impl Div<isize> for Value {
    type Output = Value;
    fn div(self, rhs: isize) -> Value { 
        let mut n:Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<isize> for Value {
    fn div_assign(&mut self, rhs:isize) {
        self.val /= rhs as f64;
    }
}

impl Add<i32> for Value {
    type Output = Value;
    fn add(self, rhs:i32) -> Value {
        let mut n:Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<i32> for Value {
    fn add_assign(&mut self, rhs:i32) {
        self.val += rhs as f64;
    }
}

impl Sub<i32> for Value {
    type Output = Value;
    fn sub(self, rhs:i32) -> Value {
        let mut n:Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<i32> for Value {
    fn sub_assign(&mut self, rhs:i32) {
        self.val -= rhs as f64;
    }
}

impl Mul<i32> for Value {
    type Output = Value;
    fn mul(self, rhs:i32) -> Value {
        let mut n:Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<i32> for Value {
    fn mul_assign(&mut self, rhs:i32) {
        self.val *= rhs as f64;
    }
}

impl Div<i32> for Value {
    type Output = Value;
    fn div(self, rhs: i32) -> Value { 
        let mut n:Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<i32> for Value {
    fn div_assign(&mut self, rhs:i32) {
        self.val /= rhs as f64;
    }
}

impl Add<u32> for Value {
    type Output = Value;
    fn add(self, rhs:u32) -> Value {
        let mut n:Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<u32> for Value {
    fn add_assign(&mut self, rhs:u32) {
        self.val += rhs as f64;
    }
}

impl Sub<u32> for Value {
    type Output = Value;
    fn sub(self, rhs:u32) -> Value {
        let mut n:Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<u32> for Value {
    fn sub_assign(&mut self, rhs:u32) {
        self.val -= rhs as f64;
    }
}

impl Mul<u32> for Value {
    type Output = Value;
    fn mul(self, rhs:u32) -> Value {
        let mut n:Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<u32> for Value {
    fn mul_assign(&mut self, rhs:u32) {
        self.val *= rhs as f64;
    }
}

impl Div<u32> for Value {
    type Output = Value;
    fn div(self, rhs: u32) -> Value { 
        let mut n:Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<u32> for Value {
    fn div_assign(&mut self, rhs:u32) {
        self.val /= rhs as f64;
    }
}

impl Add<i64> for Value {
    type Output = Value;
    fn add(self, rhs:i64) -> Value {
        let mut n:Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<i64> for Value {
    fn add_assign(&mut self, rhs:i64) {
        self.val += rhs as f64;
    }
}

impl Sub<i64> for Value {
    type Output = Value;
    fn sub(self, rhs:i64) -> Value {
        let mut n:Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<i64> for Value {
    fn sub_assign(&mut self, rhs:i64) {
        self.val -= rhs as f64;
    }
}

impl Mul<i64> for Value {
    type Output = Value;
    fn mul(self, rhs:i64) -> Value {
        let mut n:Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<i64> for Value {
    fn mul_assign(&mut self, rhs:i64) {
        self.val *= rhs as f64;
    }
}

impl Div<i64> for Value {
    type Output = Value;
    fn div(self, rhs: i64) -> Value { 
        let mut n:Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<i64> for Value {
    fn div_assign(&mut self, rhs:i64) {
        self.val /= rhs as f64;
    }
}

impl Add<u64> for Value {
    type Output = Value;
    fn add(self, rhs:u64) -> Value {
        let mut n:Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<u64> for Value {
    fn add_assign(&mut self, rhs:u64) {
        self.val += rhs as f64;
    }
}

impl Sub<u64> for Value {
    type Output = Value;
    fn sub(self, rhs:u64) -> Value {
        let mut n:Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<u64> for Value {
    fn sub_assign(&mut self, rhs:u64) {
        self.val -= rhs as f64;
    }
}

impl Mul<u64> for Value {
    type Output = Value;
    fn mul(self, rhs:u64) -> Value {
        let mut n:Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<u64> for Value {
    fn mul_assign(&mut self, rhs:u64) {
        self.val *= rhs as f64;
    }
}

impl Div<u64> for Value {
    type Output = Value;
    fn div(self, rhs: u64) -> Value { 
        let mut n:Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<u64> for Value {
    fn div_assign(&mut self, rhs:u64) {
        self.val /= rhs as f64;
    }
}

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, other:Value) -> Value {
        if self.unit_map != other.unit_map {
            // Error
            panic!("Cannot Add values {} and {}", self, other);
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map != TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot Add values {} and {}", self, other);
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                // Error
                panic!("Cannot Add values {} and {}", self, other);
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
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }                    
                    _ => {
                        // error
                        panic!("Cannot Add values {} and {}", self, other);
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
            panic!("Cannot AddAssign values {} and {}", self, other);
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot AddAssign values {} and {}", self, other);
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                // Error
                panic!("Cannot AddAssign values {} and {}", self, other);
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
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot AddAssign values {} and {}", self, other);
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
            panic!("Cannot Sub values {} and {}", self, other);
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot Sub values {} and {}", self, other);
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                // Error
                panic!("Cannot Sub values {} and {}", self, other);
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
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot Sub values {} and {}", self, other);
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
            panic!("Cannot SubAssign values {} and {}", self, other);
        }

        // special case to check if temperature is already the correct unit
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot SubAssign values {} and {}", self, other);
        }

        let mut cmp_val:f64 = other.val;

        for i in 0..31_usize {
            if self.exp[i] != other.exp[i] {
                // Error
                panic!("Cannot SubAssign values {} and {}", self, other);
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
                    MAGNETIC_FLUX_MAP => {
                        if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot SubAssign values {} and {}", self, other);
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
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot Mul values {} and {}", self, other);
        }

        if other.is_radians() && !self.is_angle() {
            n.val *= other.val;
            n.unit_map = self.unit_map;
            return n;
        } else if self.is_radians() && !other.is_angle() {
            n.val *= other.val;
            n.unit_map = other.unit_map;
            return n;
        }

        let mut cmp_val:f64 = other.val;
        for i in 0..31_usize {
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
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            n.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            n.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot Mul values {} and {}", self, other);
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
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot MulAssign values {} and {}", self, other);
        }
        
        if other.is_radians() && !self.is_angle() {
            self.val *= other.val;
            return;
        } else if self.is_radians() && !other.is_angle() {
            let t:f64 = self.val;
            *self = other;
            self.val *= t;
            return;
        }

        let mut cmp_val:f64 = other.val;
        for i in 0..31_usize {
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
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            self.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            self.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot MulAssign values {} and {}", self, other);
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
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot Div values {} and {}", self, other);
        }

        if other.is_radians() && !self.is_angle() {
            n.val /= other.val;
            n.unit_map = self.unit_map;
            return n;
        } else if self.is_radians() && !other.is_angle() {
            n.val /= other.val;
            n.unit_map = other.unit_map;
            return n;
        }

        let mut cmp_val:f64 = other.val;
        for i in 0..31_usize {
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
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            n.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            n.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot Div values {} and {}", self, other);
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
        if self.unit_map & TEMPERATURE_MAP != 0 && self.unit_map > TEMPERATURE_MAP && self.v_temperature != other.v_temperature {
            // Error cannot convert as part of larger unit
            panic!("Cannot DivAssign values {} and {}", self, other);
        }

        if other.is_radians() && !self.is_angle() {
            self.val /= other.val;
            return;
        } else if self.is_radians() && !other.is_angle() {
            let t:f64 = self.val;
            *self = other;
            self.val *= 1.0/t; // TODO : How to divide radian by value?
            return;
        }

        self.unit_map = 0;

        let mut cmp_val:f64 = other.val;
        for i in 0..31_usize {
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
                    MAGNETIC_FLUX_MAP => {
                        if must_assign {
                            self.v_magnetic_flux = other.v_magnetic_flux;
                        } else if self.v_magnetic_flux != other.v_magnetic_flux {
                            cmp_val *= other.v_magnetic_flux.unwrap().convert(&self.v_magnetic_flux.unwrap());
                        }
                    }
                    MAGNETIC_FLUX_DENSITY_MAP => {
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
                    SOLID_ANGLE_MAP => {
                        if must_assign {
                            self.v_solid_angle = other.v_solid_angle;
                        } else if self.v_solid_angle != other.v_solid_angle {
                            cmp_val *= other.v_solid_angle.unwrap().convert(&self.v_solid_angle.unwrap());
                        }
                    }
                    _ => {
                        // error
                        panic!("Cannot DivAssign values {} and {}", self, other);
                    }
                }
            }
        }
        self.val /= cmp_val;
    }
}