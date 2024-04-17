
pub mod metric;
pub mod angle;
pub mod catalytic_activity;
pub mod electrical_capacitance;
pub mod electrical_charge;
pub mod electrical_conductance;
pub mod electrical_current;
pub mod electrical_inductance;
pub mod electrical_potential;
pub mod electrical_resistance;
pub mod energy;
pub mod force;
pub mod frequency;
pub mod illuminance;
pub mod information;
pub mod length;
pub mod luminous_flux;
pub mod luminous_intensity;
pub mod magnetic_flux;
pub mod magnetic_flux_density;
pub mod mass;
pub mod power;
pub mod pressure;
pub mod radiation_absorbed_dose;
pub mod radiation_equivalent_dose;
pub mod radioactivity;
pub mod angle_solid;
pub mod sound;
pub mod substance;
pub mod temperature;
pub mod time;
pub mod unitless;
pub mod volume;

use self::metric::Metric;

/// Trait that can be used and called by all of the unit types
trait Convert<T> {
    /// The function template for a unit type
    fn convert(&self, other: &T) -> f64;
}

trait BaseUnit {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64;

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64;

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric;
}

#[cfg(test)]
mod units_unit_test {
    use crate::units::Metric;
    use crate::units::pressure::UnitPressure;

    #[test]
    fn unit_get_metric() {
        let t = UnitPressure::Bar(Metric::Exa);
        assert_eq!(t.get_metric(), Metric::Exa);
    }
}
