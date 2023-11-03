use std::{ops::Div, process::Output};

use crate::{values::Value, constants::{TIME_MAP, LENGTH_MAP}};

pub struct Length;
pub struct Time;
pub struct Velocity;
pub struct Acceleration;
pub struct Mass;
pub struct Default;

impl<Length> Div<Value<Time>> for Value<Length> {
    type Output = Value<Velocity>;
    fn div(self, rhs: Value<Time>) -> Value<Velocity> {
        Value {
            val: self.val / rhs.val,
            unit_map: TIME_MAP | LENGTH_MAP,
            exp: ,
            measure: Velocity,
            v_ab_dose: todo!(),
            v_angle: todo!(),
            v_capacitance: todo!(),
            v_catalytic: todo!(),
            v_electric_charge: todo!(),
            v_electric_conductance: todo!(),
            v_electric_current: todo!(),
            v_electric_potential: todo!(),
            v_energy: todo!(),
            v_force: todo!(),
            v_frequency: todo!(),
            v_illuminance: todo!(),
            v_inductance: todo!(),
            v_information: todo!(),
            v_length: todo!(),
            v_luminous_flux: todo!(),
            v_luminous_flux_intensity: todo!(),
            v_mass: todo!(),
            v_power: todo!(),
            v_pressure: todo!(),
            v_radioactivity: todo!(),
            v_radioactivity_exposure: todo!(),
            v_resistance: todo!(),
            v_sound: todo!(),
            v_substance: todo!(),
            v_temperature: todo!(),
            v_time: todo!(),
            v_volume: todo!(),
            v_magnetic_flux: todo!(),
            v_magnetic_flux_density: todo!(),
            v_solid_angle: todo!(),
        }
    }
}