use std::ops::{Div, Mul};

use crate::constants::*;
use crate::units::Metric;
use crate::units::UnitAbsorbedDose;
use crate::units::UnitAngle;
use crate::units::UnitCatalyticActivity;
use crate::units::UnitElectricCapacitance;
use crate::units::UnitElectricCharge;
use crate::units::UnitElectricConductance;
use crate::units::UnitElectricCurrent;
use crate::units::UnitElectricInductance;
use crate::units::UnitElectricPotential;
use crate::units::UnitElectricResistance;
use crate::units::UnitEnergy;
use crate::units::UnitForce;
use crate::units::UnitFrequency;
use crate::units::UnitIlluminance;
use crate::units::UnitInformation;
use crate::units::UnitLength;
use crate::units::UnitLuminousFlux;
use crate::units::UnitLuminousIntensity;
use crate::units::UnitMagneticFlux;
use crate::units::UnitMagneticFluxDensity;
use crate::units::UnitMass;
use crate::units::UnitPower;
use crate::units::UnitPressure;
use crate::units::UnitRadioactivity;
use crate::units::UnitRadioactivityExposure;
use crate::units::UnitSolidAngle;
use crate::units::UnitSound;
use crate::units::UnitSubstance;
use crate::units::UnitTemperature;
use crate::units::UnitTime;
use crate::units::UnitVolume;
use crate::values::Value;

impl Mul<UnitLength> for f64 {
    type Output = Value;
    fn mul(self, other: UnitLength) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_length: Some(other),
            unit_map: LENGTH_MAP,
            ..Default::default()
        };
        ret.exp[LENGTH_INDEX] = 1;
        ret
    }
}

impl Div<UnitLength> for f64 {
    type Output = Value;
    fn div(self, other: UnitLength) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_length: Some(other),
            unit_map: LENGTH_MAP,
            ..Default::default()
        };
        ret.exp[LENGTH_INDEX] = -1;
        ret
    }
}

impl Mul<UnitLength> for Value {
    type Output = Value;
    fn mul(self, other: UnitLength) -> Self::Output {
        let mut new: Value = self;
        if self.exp[LENGTH_INDEX] == 0 {
            new.v_length = Some(other);
            new.exp[LENGTH_INDEX] = 1;
            new.unit_map |= LENGTH_MAP;
        } else if self.exp[LENGTH_INDEX] == -1 {
            new.exp[LENGTH_INDEX] = 0;
            new.v_length = None;
            new.unit_map ^= LENGTH_MAP;
        } else {
            if self.v_length != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_length.unwrap()
                );
            }
            new.exp[LENGTH_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitLength> for Value {
    type Output = Value;
    fn div(self, other: UnitLength) -> Value {
        let mut new: Value = self;
        if self.v_length.is_some() && self.v_length != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[LENGTH_INDEX] == 0 {
            new.v_length = Some(other);
            new.unit_map |= LENGTH_MAP;
            new.exp[LENGTH_INDEX] = -1;
        } else if self.exp[LENGTH_INDEX] == 1 {
            new.exp[LENGTH_INDEX] = 0;
            new.v_length = None;
            new.unit_map ^= LENGTH_MAP;
        } else {
            new.exp[LENGTH_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitTime> for f64 {
    type Output = Value;
    fn mul(self, other: UnitTime) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_time: Some(other),
            unit_map: TIME_MAP,
            ..Default::default()
        };
        ret.exp[TIME_INDEX] = 1;
        ret
    }
}

impl Div<UnitTime> for f64 {
    type Output = Value;
    fn div(self, other: UnitTime) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_time: Some(other),
            unit_map: TIME_MAP,
            ..Default::default()
        };
        ret.exp[TIME_INDEX] = -1;
        ret
    }
}

impl Mul<UnitTime> for Value {
    type Output = Value;
    fn mul(self, other: UnitTime) -> Self::Output {
        let mut new: Value = self;
        if self.exp[TIME_INDEX] == 0 {
            new.v_time = Some(other);
            new.exp[TIME_INDEX] = 1;
            new.unit_map |= TIME_MAP;
        } else if self.exp[TIME_INDEX] == -1 {
            new.exp[TIME_INDEX] = 0;
            new.v_time = None;
            new.unit_map ^= TIME_MAP;
        } else {
            if self.v_time != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_time.unwrap()
                );
            }
            new.exp[TIME_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitTime> for Value {
    type Output = Value;
    fn div(self, other: UnitTime) -> Value {
        let mut new: Value = self;
        if self.v_time.is_some() && self.v_time != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[TIME_INDEX] == 0 {
            new.v_time = Some(other);
            new.unit_map |= TIME_MAP;
            new.exp[TIME_INDEX] = -1;
        } else if self.exp[TIME_INDEX] == 1 {
            new.exp[TIME_INDEX] = 0;
            new.v_time = None;
            new.unit_map ^= TIME_MAP;
        } else {
            new.exp[TIME_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitAbsorbedDose> for f64 {
    type Output = Value;
    fn mul(self, other: UnitAbsorbedDose) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_ab_dose: Some(other),
            unit_map: ABSORBED_DOSE_MAP,
            ..Default::default()
        };
        ret.exp[ABSORBED_DOSE_INDEX] = 1;
        ret
    }
}

impl Div<UnitAbsorbedDose> for f64 {
    type Output = Value;
    fn div(self, other: UnitAbsorbedDose) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_ab_dose: Some(other),
            unit_map: ABSORBED_DOSE_MAP,
            ..Default::default()
        };
        ret.exp[ABSORBED_DOSE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitAbsorbedDose> for Value {
    type Output = Value;
    fn mul(self, other: UnitAbsorbedDose) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ABSORBED_DOSE_INDEX] == 0 {
            new.v_ab_dose = Some(other);
            new.exp[ABSORBED_DOSE_INDEX] = 1;
            new.unit_map |= ABSORBED_DOSE_MAP;
        } else if self.exp[ABSORBED_DOSE_INDEX] == -1 {
            new.exp[ABSORBED_DOSE_INDEX] = 0;
            new.v_ab_dose = None;
            new.unit_map ^= ABSORBED_DOSE_MAP;
        } else {
            if self.v_ab_dose != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_ab_dose.unwrap()
                );
            }
            new.exp[ABSORBED_DOSE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitAbsorbedDose> for Value {
    type Output = Value;
    fn div(self, other: UnitAbsorbedDose) -> Value {
        let mut new: Value = self;
        if self.v_ab_dose.is_some() && self.v_ab_dose != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ABSORBED_DOSE_INDEX] == 0 {
            new.v_ab_dose = Some(other);
            new.unit_map |= ABSORBED_DOSE_MAP;
            new.exp[ABSORBED_DOSE_INDEX] = -1;
        } else if self.exp[ABSORBED_DOSE_INDEX] == 1 {
            new.exp[ABSORBED_DOSE_INDEX] = 0;
            new.v_ab_dose = None;
            new.unit_map ^= ABSORBED_DOSE_MAP;
        } else {
            new.exp[ABSORBED_DOSE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitAngle> for f64 {
    type Output = Value;
    fn mul(self, other: UnitAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_angle: Some(other),
            unit_map: ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[ANGLE_INDEX] = 1;
        ret
    }
}

impl Div<UnitAngle> for f64 {
    type Output = Value;
    fn div(self, other: UnitAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_angle: Some(other),
            unit_map: ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[ANGLE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitAngle> for Value {
    type Output = Value;
    fn mul(self, other: UnitAngle) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ANGLE_INDEX] == 0 {
            new.v_angle = Some(other);
            new.exp[ANGLE_INDEX] = 1;
            new.unit_map |= ANGLE_MAP;
        } else if self.exp[ANGLE_INDEX] == -1 {
            new.exp[ANGLE_INDEX] = 0;
            new.v_angle = None;
            new.unit_map ^= ANGLE_MAP;
        } else {
            if self.v_angle != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_angle.unwrap()
                );
            }
            new.exp[ANGLE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitAngle> for Value {
    type Output = Value;
    fn div(self, other: UnitAngle) -> Value {
        let mut new: Value = self;
        if self.v_angle.is_some() && self.v_angle != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ANGLE_INDEX] == 0 {
            new.v_angle = Some(other);
            new.unit_map |= ANGLE_MAP;
            new.exp[ANGLE_INDEX] = -1;
        } else if self.exp[ANGLE_INDEX] == 1 {
            new.exp[ANGLE_INDEX] = 0;
            new.v_angle = None;
            new.unit_map ^= ANGLE_MAP;
        } else {
            new.exp[ANGLE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitElectricCapacitance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricCapacitance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_capacitance: Some(other),
            unit_map: CAPACITANCE_MAP,
            ..Default::default()
        };
        ret.exp[CAPACITANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricCapacitance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricCapacitance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_capacitance: Some(other),
            unit_map: CAPACITANCE_MAP,
            ..Default::default()
        };
        ret.exp[CAPACITANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricCapacitance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricCapacitance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[CAPACITANCE_INDEX] == 0 {
            new.v_capacitance = Some(other);
            new.exp[CAPACITANCE_INDEX] = 1;
            new.unit_map |= CAPACITANCE_MAP;
        } else if self.exp[CAPACITANCE_INDEX] == -1 {
            new.exp[CAPACITANCE_INDEX] = 0;
            new.v_capacitance = None;
            new.unit_map ^= CAPACITANCE_MAP;
        } else {
            if self.v_capacitance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_capacitance.unwrap()
                );
            }
            new.exp[CAPACITANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricCapacitance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricCapacitance) -> Value {
        let mut new: Value = self;
        if self.v_capacitance.is_some() && self.v_capacitance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[CAPACITANCE_INDEX] == 0 {
            new.v_capacitance = Some(other);
            new.unit_map |= CAPACITANCE_MAP;
            new.exp[CAPACITANCE_INDEX] = -1;
        } else if self.exp[CAPACITANCE_INDEX] == 1 {
            new.exp[CAPACITANCE_INDEX] = 0;
            new.v_capacitance = None;
            new.unit_map ^= CAPACITANCE_MAP;
        } else {
            new.exp[CAPACITANCE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitCatalyticActivity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitCatalyticActivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_catalytic: Some(other),
            unit_map: CATALYTIC_ACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[CATALYTIC_ACTIVITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitCatalyticActivity> for f64 {
    type Output = Value;
    fn div(self, other: UnitCatalyticActivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_catalytic: Some(other),
            unit_map: CATALYTIC_ACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[CATALYTIC_ACTIVITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitCatalyticActivity> for Value {
    type Output = Value;
    fn mul(self, other: UnitCatalyticActivity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[CATALYTIC_ACTIVITY_INDEX] == 0 {
            new.v_catalytic = Some(other);
            new.exp[CATALYTIC_ACTIVITY_INDEX] = 1;
            new.unit_map |= CATALYTIC_ACTIVITY_MAP;
        } else if self.exp[CATALYTIC_ACTIVITY_INDEX] == -1 {
            new.exp[CATALYTIC_ACTIVITY_INDEX] = 0;
            new.v_catalytic = None;
            new.unit_map ^= CATALYTIC_ACTIVITY_MAP;
        } else {
            if self.v_catalytic != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_catalytic.unwrap()
                );
            }
            new.exp[CATALYTIC_ACTIVITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitCatalyticActivity> for Value {
    type Output = Value;
    fn div(self, other: UnitCatalyticActivity) -> Value {
        let mut new: Value = self;
        if self.v_catalytic.is_some() && self.v_catalytic != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[CATALYTIC_ACTIVITY_INDEX] == 0 {
            new.v_catalytic = Some(other);
            new.unit_map |= CATALYTIC_ACTIVITY_MAP;
            new.exp[CATALYTIC_ACTIVITY_INDEX] = -1;
        } else if new.exp[CATALYTIC_ACTIVITY_INDEX] == 1 {
            new.exp[CATALYTIC_ACTIVITY_INDEX] = 0;
            new.v_catalytic = None;
            new.unit_map ^= CATALYTIC_ACTIVITY_MAP;
        } else {
            new.exp[CATALYTIC_ACTIVITY_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitElectricCharge> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricCharge) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_charge: Some(other),
            unit_map: ELECTRIC_CHARGE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CHARGE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricCharge> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricCharge) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_charge: Some(other),
            unit_map: ELECTRIC_CHARGE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CHARGE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricCharge> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricCharge) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_CHARGE_INDEX] == 0 {
            new.v_electric_charge = Some(other);
            new.exp[ELECTRIC_CHARGE_INDEX] = 1;
            new.unit_map |= ELECTRIC_CHARGE_MAP;
        } else if self.exp[ELECTRIC_CHARGE_INDEX] == -1 {
            new.exp[ELECTRIC_CHARGE_INDEX] = 0;
            new.v_electric_charge = None;
            new.unit_map ^= ELECTRIC_CHARGE_MAP;
        } else {
            if self.v_electric_charge != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_charge.unwrap()
                );
            }
            new.exp[ELECTRIC_CHARGE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricCharge> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricCharge) -> Value {
        let mut new: Value = self;
        if self.v_electric_charge.is_some() && self.v_electric_charge != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_CHARGE_INDEX] == 0 {
            new.v_electric_charge = Some(other);
            new.unit_map |= ELECTRIC_CHARGE_MAP;
            new.exp[ELECTRIC_CHARGE_INDEX] = -1;
        } else if new.exp[ELECTRIC_CHARGE_INDEX] == 1 {
            new.exp[ELECTRIC_CHARGE_INDEX] = 0;
            new.v_electric_charge = None;
            new.unit_map ^= ELECTRIC_CHARGE_MAP;
        } else {
            new.exp[ELECTRIC_CHARGE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitElectricConductance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricConductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_conductance: Some(other),
            unit_map: ELECTRIC_CONDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CONDUCTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricConductance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricConductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_conductance: Some(other),
            unit_map: ELECTRIC_CONDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CONDUCTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricConductance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricConductance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 0 {
            new.v_electric_conductance = Some(other);
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = 1;
            new.unit_map |= ELECTRIC_CONDUCTANCE_MAP;
        } else if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == -1 {
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
            new.v_electric_conductance = None;
            new.unit_map ^= ELECTRIC_CONDUCTANCE_MAP;
        } else {
            if self.v_electric_conductance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_conductance.unwrap()
                );
            }
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricConductance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricConductance) -> Value {
        let mut new: Value = self;
        if self.v_electric_conductance.is_some() && self.v_electric_conductance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 0 {
            new.v_electric_conductance = Some(other);
            new.unit_map |= ELECTRIC_CONDUCTANCE_MAP;
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = -1;
        } else if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 1 {
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
            new.v_electric_conductance = None;
            new.unit_map ^= ELECTRIC_CONDUCTANCE_MAP;
        } else {
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitElectricCurrent> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricCurrent) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_current: Some(other),
            unit_map: ELECTRIC_CURRENT_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CURRENT_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricCurrent> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricCurrent) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_current: Some(other),
            unit_map: ELECTRIC_CURRENT_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CURRENT_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricCurrent> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricCurrent) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_CURRENT_INDEX] == 0 {
            new.v_electric_current = Some(other);
            new.exp[ELECTRIC_CURRENT_INDEX] = 1;
            new.unit_map |= ELECTRIC_CURRENT_MAP;
        } else if self.exp[ELECTRIC_CURRENT_INDEX] == -1 {
            new.exp[ELECTRIC_CURRENT_INDEX] = 0;
            new.v_electric_current = None;
            new.unit_map ^= ELECTRIC_CURRENT_MAP;
        } else {
            if self.v_electric_current != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_current.unwrap()
                );
            }
            new.exp[ELECTRIC_CURRENT_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricCurrent> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricCurrent) -> Value {
        let mut new: Value = self;
        if self.v_electric_current.is_some() && self.v_electric_current != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_CURRENT_INDEX] == 0 {
            new.v_electric_current = Some(other);
            new.unit_map |= ELECTRIC_CURRENT_MAP;
            new.exp[ELECTRIC_CURRENT_INDEX] = -1;
        } else if new.exp[ELECTRIC_CURRENT_INDEX] == 1 {
            new.exp[ELECTRIC_CURRENT_INDEX] = 0;
            new.v_electric_current = None;
            new.unit_map ^= ELECTRIC_CURRENT_MAP;
        } else {
            new.exp[ELECTRIC_CURRENT_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitElectricPotential> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricPotential) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_potential: Some(other),
            unit_map: ELECTRIC_POTENTIAL_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricPotential> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricPotential) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_potential: Some(other),
            unit_map: ELECTRIC_POTENTIAL_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_POTENTIAL_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricPotential> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricPotential) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_POTENTIAL_INDEX] == 0 {
            new.v_electric_potential = Some(other);
            new.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
            new.unit_map |= ELECTRIC_POTENTIAL_MAP;
        } else if self.exp[ELECTRIC_POTENTIAL_INDEX] == -1 {
            new.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
            new.v_electric_potential = None;
            new.unit_map ^= ELECTRIC_POTENTIAL_MAP;
        } else {
            if self.v_electric_potential != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_potential.unwrap()
                );
            }
            new.exp[ELECTRIC_POTENTIAL_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricPotential> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricPotential) -> Value {
        let mut new: Value = self;
        if self.v_electric_potential.is_some() && self.v_electric_potential != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_POTENTIAL_INDEX] == 0 {
            new.v_electric_potential = Some(other);
            new.unit_map |= ELECTRIC_POTENTIAL_MAP;
            new.exp[ELECTRIC_POTENTIAL_INDEX] = -1;
        } else if new.exp[ELECTRIC_POTENTIAL_INDEX] == 1 {
            new.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
            new.v_electric_potential = None;
            new.unit_map ^= ELECTRIC_POTENTIAL_MAP;
        } else {
            new.exp[ELECTRIC_POTENTIAL_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitEnergy> for f64 {
    type Output = Value;
    fn mul(self, other: UnitEnergy) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_energy: Some(other),
            unit_map: ENERGY_MAP,
            ..Default::default()
        };
        ret.exp[ENERGY_INDEX] = 1;
        ret
    }
}

impl Div<UnitEnergy> for f64 {
    type Output = Value;
    fn div(self, other: UnitEnergy) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_energy: Some(other),
            unit_map: ENERGY_MAP,
            ..Default::default()
        };
        ret.exp[ENERGY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitEnergy> for Value {
    type Output = Value;
    fn mul(self, other: UnitEnergy) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ENERGY_INDEX] == 0 {
            new.v_energy = Some(other);
            new.exp[ENERGY_INDEX] = 1;
            new.unit_map |= ENERGY_MAP;
        } else if self.exp[ENERGY_INDEX] == -1 {
            new.exp[ENERGY_INDEX] = 0;
            new.v_energy = None;
            new.unit_map ^= ENERGY_MAP;
        } else {
            if self.v_energy != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_energy.unwrap()
                );
            }
            new.exp[ENERGY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitEnergy> for Value {
    type Output = Value;
    fn div(self, other: UnitEnergy) -> Value {
        let mut new: Value = self;
        if self.v_energy.is_some() && self.v_energy != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ENERGY_INDEX] == 0 {
            new.v_energy = Some(other);
            new.unit_map |= ENERGY_MAP;
            new.exp[ENERGY_INDEX] = -1;
        } else if new.exp[ENERGY_INDEX] == 1 {
            new.exp[ENERGY_INDEX] = 0;
            new.v_energy = None;
            new.unit_map ^= ENERGY_MAP;
        } else {
            new.exp[ENERGY_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitForce> for f64 {
    type Output = Value;
    fn mul(self, other: UnitForce) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_force: Some(other),
            unit_map: FORCE_MAP,
            ..Default::default()
        };
        ret.exp[FORCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitForce> for f64 {
    type Output = Value;
    fn div(self, other: UnitForce) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_force: Some(other),
            unit_map: FORCE_MAP,
            ..Default::default()
        };
        ret.exp[FORCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitForce> for Value {
    type Output = Value;
    fn mul(self, other: UnitForce) -> Self::Output {
        let mut new: Value = self;
        if self.exp[FORCE_INDEX] == 0 {
            new.v_force = Some(other);
            new.exp[FORCE_INDEX] = 1;
            new.unit_map |= FORCE_MAP;
        } else if self.exp[FORCE_INDEX] == -1 {
            new.exp[FORCE_INDEX] = 0;
            new.v_force = None;
            new.unit_map ^= FORCE_MAP;
        } else {
            if self.v_force != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_force.unwrap()
                );
            }
            new.exp[FORCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitForce> for Value {
    type Output = Value;
    fn div(self, other: UnitForce) -> Value {
        let mut new: Value = self;
        if self.v_force.is_some() && self.v_force != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[FORCE_INDEX] == 0 {
            new.v_force = Some(other);
            new.unit_map |= FORCE_MAP;
            new.exp[FORCE_INDEX] = -1;
        } else if new.exp[FORCE_INDEX] == 1 {
            new.exp[FORCE_INDEX] = 0;
            new.v_force = None;
            new.unit_map ^= FORCE_MAP;
        } else {
            new.exp[FORCE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitFrequency> for f64 {
    type Output = Value;
    fn mul(self, other: UnitFrequency) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_frequency: Some(other),
            unit_map: FREQUENCY_MAP,
            ..Default::default()
        };
        ret.exp[FREQUENCY_INDEX] = 1;
        ret
    }
}

impl Div<UnitFrequency> for f64 {
    type Output = Value;
    fn div(self, other: UnitFrequency) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_frequency: Some(other),
            unit_map: FREQUENCY_MAP,
            ..Default::default()
        };
        ret.exp[FREQUENCY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitFrequency> for Value {
    type Output = Value;
    fn mul(self, other: UnitFrequency) -> Self::Output {
        let mut new: Value = self;
        if self.exp[FREQUENCY_INDEX] == 0 {
            new.v_frequency = Some(other);
            new.exp[FREQUENCY_INDEX] = 1;
            new.unit_map |= FREQUENCY_MAP;
        } else if self.exp[FREQUENCY_INDEX] == -1 {
            new.exp[FREQUENCY_INDEX] = 0;
            new.v_frequency = None;
            new.unit_map ^= FREQUENCY_MAP;
        } else {
            if self.v_frequency != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_frequency.unwrap()
                );
            }
            new.exp[FREQUENCY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitFrequency> for Value {
    type Output = Value;
    fn div(self, other: UnitFrequency) -> Value {
        let mut new: Value = self;
        if self.v_frequency.is_some() && self.v_frequency != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[FREQUENCY_INDEX] == 0 {
            new.v_frequency = Some(other);
            new.unit_map |= FREQUENCY_MAP;
            new.exp[FREQUENCY_INDEX] = -1;
        } else if new.exp[FREQUENCY_INDEX] == 1 {
            new.exp[FREQUENCY_INDEX] = 0;
            new.v_frequency = None;
            new.unit_map ^= FREQUENCY_MAP;
        } else {
            new.exp[FREQUENCY_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitIlluminance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitIlluminance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_illuminance: Some(other),
            unit_map: ILLUMINANCE_MAP,
            ..Default::default()
        };
        ret.exp[ILLUMINANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitIlluminance> for f64 {
    type Output = Value;
    fn div(self, other: UnitIlluminance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_illuminance: Some(other),
            unit_map: ILLUMINANCE_MAP,
            ..Default::default()
        };
        ret.exp[ILLUMINANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitIlluminance> for Value {
    type Output = Value;
    fn mul(self, other: UnitIlluminance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ILLUMINANCE_INDEX] == 0 {
            new.v_illuminance = Some(other);
            new.exp[ILLUMINANCE_INDEX] = 1;
            new.unit_map |= ILLUMINANCE_MAP;
        } else if self.exp[ILLUMINANCE_INDEX] == -1 {
            new.exp[ILLUMINANCE_INDEX] = 0;
            new.v_illuminance = None;
            new.unit_map ^= ILLUMINANCE_MAP;
        } else {
            if self.v_illuminance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_illuminance.unwrap()
                );
            }
            new.exp[ILLUMINANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitIlluminance> for Value {
    type Output = Value;
    fn div(self, other: UnitIlluminance) -> Value {
        let mut new: Value = self;
        if self.v_illuminance.is_some() && self.v_illuminance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ILLUMINANCE_INDEX] == 0 {
            new.v_illuminance = Some(other);
            new.unit_map |= ILLUMINANCE_MAP;
            new.exp[ILLUMINANCE_INDEX] = -1;
        } else if new.exp[ILLUMINANCE_INDEX] == 1 {
            new.exp[ILLUMINANCE_INDEX] = 0;
            new.v_illuminance = None;
            new.unit_map ^= ILLUMINANCE_MAP;
        } else {
            new.exp[ILLUMINANCE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitVolume> for f64 {
    type Output = Value;
    fn mul(self, other: UnitVolume) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_volume: Some(other),
            unit_map: VOLUME_MAP,
            ..Default::default()
        };
        ret.exp[VOLUME_INDEX] = 1;
        ret
    }
}

impl Div<UnitVolume> for f64 {
    type Output = Value;
    fn div(self, other: UnitVolume) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_volume: Some(other),
            unit_map: VOLUME_MAP,
            ..Default::default()
        };
        ret.exp[VOLUME_INDEX] = -1;
        ret
    }
}

impl Mul<UnitVolume> for Value {
    type Output = Value;
    fn mul(self, other: UnitVolume) -> Self::Output {
        let mut new: Value = self;
        if self.exp[VOLUME_INDEX] == 0 {
            new.v_volume = Some(other);
            new.exp[VOLUME_INDEX] = 1;
            new.unit_map |= VOLUME_MAP;
        } else if self.exp[VOLUME_INDEX] == -1 {
            new.exp[VOLUME_INDEX] = 0;
            new.v_volume = None;
            new.unit_map ^= VOLUME_MAP;
        } else {
            if self.v_volume != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_volume.unwrap()
                );
            }
            new.exp[VOLUME_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitVolume> for Value {
    type Output = Value;
    fn div(self, other: UnitVolume) -> Value {
        let mut new: Value = self;
        if self.v_volume.is_some() && self.v_volume != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[VOLUME_INDEX] == 0 {
            new.v_volume = Some(other);
            new.unit_map |= VOLUME_MAP;
            new.exp[VOLUME_INDEX] = -1;
        } else if new.exp[VOLUME_INDEX] == 1 {
            new.exp[VOLUME_INDEX] = 0;
            new.v_volume = None;
            new.unit_map ^= VOLUME_MAP;
        } else {
            new.exp[VOLUME_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitTemperature> for f64 {
    type Output = Value;
    fn mul(self, other: UnitTemperature) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_temperature: Some(other),
            unit_map: TEMPERATURE_MAP,
            ..Default::default()
        };
        ret.exp[TEMPERATURE_INDEX] = 1;
        ret
    }
}

impl Div<UnitTemperature> for f64 {
    type Output = Value;
    fn div(self, other: UnitTemperature) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_temperature: Some(other),
            unit_map: TEMPERATURE_MAP,
            ..Default::default()
        };
        ret.exp[TEMPERATURE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitTemperature> for Value {
    type Output = Value;
    fn mul(self, other: UnitTemperature) -> Self::Output {
        let mut new: Value = self;
        if self.exp[TEMPERATURE_INDEX] == 0 {
            new.v_temperature = Some(other);
            new.exp[TEMPERATURE_INDEX] = 1;
            new.unit_map |= TEMPERATURE_MAP;
        } else if self.exp[TEMPERATURE_INDEX] == -1 {
            new.exp[TEMPERATURE_INDEX] = 0;
            new.v_temperature = None;
            new.unit_map ^= TEMPERATURE_MAP;
        } else {
            if self.v_temperature != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_temperature.unwrap()
                );
            }
            new.exp[TEMPERATURE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitTemperature> for Value {
    type Output = Value;
    fn div(self, other: UnitTemperature) -> Value {
        let mut new: Value = self;
        if self.v_temperature.is_some() && self.v_temperature != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[TEMPERATURE_INDEX] == 0 {
            new.v_temperature = Some(other);
            new.unit_map |= TEMPERATURE_MAP;
            new.exp[TEMPERATURE_INDEX] = -1;
        } else if new.exp[TEMPERATURE_INDEX] == 1 {
            new.exp[TEMPERATURE_INDEX] = 0;
            new.v_temperature = None;
            new.unit_map ^= TEMPERATURE_MAP;
        } else {
            new.exp[TEMPERATURE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitSubstance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitSubstance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_substance: Some(other),
            unit_map: SUBSTANCE_MAP,
            ..Default::default()
        };
        ret.exp[SUBSTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitSubstance> for f64 {
    type Output = Value;
    fn div(self, other: UnitSubstance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_substance: Some(other),
            unit_map: SUBSTANCE_MAP,
            ..Default::default()
        };
        ret.exp[SUBSTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitSubstance> for Value {
    type Output = Value;
    fn mul(self, other: UnitSubstance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[SUBSTANCE_INDEX] == 0 {
            new.v_substance = Some(other);
            new.exp[SUBSTANCE_INDEX] = 1;
            new.unit_map |= SUBSTANCE_MAP;
        } else if self.exp[SUBSTANCE_INDEX] == -1 {
            new.exp[SUBSTANCE_INDEX] = 0;
            new.v_substance = None;
            new.unit_map ^= SUBSTANCE_MAP;
        } else {
            if self.v_substance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_substance.unwrap()
                );
            }
            new.exp[SUBSTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitSubstance> for Value {
    type Output = Value;
    fn div(self, other: UnitSubstance) -> Value {
        let mut new: Value = self;
        if self.v_substance.is_some() && self.v_substance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[SUBSTANCE_INDEX] == 0 {
            new.v_substance = Some(other);
            new.unit_map |= SUBSTANCE_MAP;
            new.exp[SUBSTANCE_INDEX] = -1;
        } else if new.exp[SUBSTANCE_INDEX] == 1 {
            new.exp[SUBSTANCE_INDEX] = 0;
            new.v_substance = None;
            new.unit_map ^= SUBSTANCE_MAP;
        } else {
            new.exp[SUBSTANCE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitSound> for f64 {
    type Output = Value;
    fn mul(self, other: UnitSound) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_sound: Some(other),
            unit_map: SOUND_MAP,
            ..Default::default()
        };
        ret.exp[SOUND_INDEX] = 1;
        ret
    }
}

impl Div<UnitSound> for f64 {
    type Output = Value;
    fn div(self, other: UnitSound) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_sound: Some(other),
            unit_map: SOUND_MAP,
            ..Default::default()
        };
        ret.exp[SOUND_INDEX] = -1;
        ret
    }
}

impl Mul<UnitSound> for Value {
    type Output = Value;
    fn mul(self, other: UnitSound) -> Self::Output {
        let mut new: Value = self;
        if self.exp[SOUND_INDEX] == 0 {
            new.v_sound = Some(other);
            new.exp[SOUND_INDEX] = 1;
            new.unit_map |= SOUND_MAP;
        } else if self.exp[SOUND_INDEX] == -1 {
            new.exp[SOUND_INDEX] = 0;
            new.v_sound = None;
            new.unit_map ^= SOUND_MAP;
        } else {
            if self.v_sound != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_sound.unwrap()
                );
            }
            new.exp[SOUND_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitSound> for Value {
    type Output = Value;
    fn div(self, other: UnitSound) -> Value {
        let mut new: Value = self;
        if self.v_sound.is_some() && self.v_sound != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[SOUND_INDEX] == 0 {
            new.v_sound = Some(other);
            new.unit_map |= SOUND_MAP;
            new.exp[SOUND_INDEX] = -1;
        } else if new.exp[SOUND_INDEX] == 1 {
            new.exp[SOUND_INDEX] = 0;
            new.v_sound = None;
            new.unit_map ^= SOUND_MAP;
        } else {
            new.exp[SOUND_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitSolidAngle> for f64 {
    type Output = Value;
    fn mul(self, other: UnitSolidAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_solid_angle: Some(other),
            unit_map: SOLID_ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[SOLID_ANGLE_INDEX] = 1;
        ret
    }
}

impl Div<UnitSolidAngle> for f64 {
    type Output = Value;
    fn div(self, other: UnitSolidAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_solid_angle: Some(other),
            unit_map: SOLID_ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[SOLID_ANGLE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitSolidAngle> for Value {
    type Output = Value;
    fn mul(self, other: UnitSolidAngle) -> Self::Output {
        let mut new: Value = self;
        if self.exp[SOLID_ANGLE_INDEX] == 0 {
            new.v_solid_angle = Some(other);
            new.exp[SOLID_ANGLE_INDEX] = 1;
            new.unit_map |= SOLID_ANGLE_MAP;
        } else if self.exp[SOLID_ANGLE_INDEX] == -1 {
            new.exp[SOLID_ANGLE_INDEX] = 0;
            new.v_solid_angle = None;
            new.unit_map ^= SOLID_ANGLE_MAP;
        } else {
            if self.v_solid_angle != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_solid_angle.unwrap()
                );
            }
            new.exp[SOLID_ANGLE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitSolidAngle> for Value {
    type Output = Value;
    fn div(self, other: UnitSolidAngle) -> Value {
        let mut new: Value = self;
        if self.v_solid_angle.is_some() && self.v_solid_angle != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[SOLID_ANGLE_INDEX] == 0 {
            new.v_solid_angle = Some(other);
            new.unit_map |= SOLID_ANGLE_MAP;
            new.exp[SOLID_ANGLE_INDEX] = -1;
        } else if new.exp[SOLID_ANGLE_INDEX] == 1 {
            new.exp[SOLID_ANGLE_INDEX] = 0;
            new.v_solid_angle = None;
            new.unit_map ^= SOLID_ANGLE_MAP;
        } else {
            new.exp[SOLID_ANGLE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitElectricResistance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricResistance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_resistance: Some(other),
            unit_map: RESISTANCE_MAP,
            ..Default::default()
        };
        ret.exp[RESISTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricResistance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricResistance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_resistance: Some(other),
            unit_map: RESISTANCE_MAP,
            ..Default::default()
        };
        ret.exp[RESISTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricResistance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricResistance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[RESISTANCE_INDEX] == 0 {
            new.v_resistance = Some(other);
            new.exp[RESISTANCE_INDEX] = 1;
            new.unit_map |= RESISTANCE_MAP;
        } else if self.exp[RESISTANCE_INDEX] == -1 {
            new.exp[RESISTANCE_INDEX] = 0;
            new.v_resistance = None;
            new.unit_map ^= RESISTANCE_MAP;
        } else {
            if self.v_resistance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_resistance.unwrap()
                );
            }
            new.exp[RESISTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricResistance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricResistance) -> Value {
        let mut new: Value = self;
        if self.v_resistance.is_some() && self.v_resistance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[RESISTANCE_INDEX] == 0 {
            new.v_resistance = Some(other);
            new.unit_map |= RESISTANCE_MAP;
            new.exp[RESISTANCE_INDEX] = -1;
        } else if new.exp[RESISTANCE_INDEX] == 1 {
            new.exp[RESISTANCE_INDEX] = 0;
            new.v_resistance = None;
            new.unit_map ^= RESISTANCE_MAP;
        } else {
            new.exp[RESISTANCE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitRadioactivityExposure> for f64 {
    type Output = Value;
    fn mul(self, other: UnitRadioactivityExposure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity_exposure: Some(other),
            unit_map: RADIOACTIVITY_EXPOSURE_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 1;
        ret
    }
}

impl Div<UnitRadioactivityExposure> for f64 {
    type Output = Value;
    fn div(self, other: UnitRadioactivityExposure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity_exposure: Some(other),
            unit_map: RADIOACTIVITY_EXPOSURE_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_EXPOSURE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitRadioactivityExposure> for Value {
    type Output = Value;
    fn mul(self, other: UnitRadioactivityExposure) -> Self::Output {
        let mut new: Value = self;
        if self.exp[RADIOACTIVITY_EXPOSURE_INDEX] == 0 {
            new.v_radioactivity_exposure = Some(other);
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 1;
            new.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
        } else if self.exp[RADIOACTIVITY_EXPOSURE_INDEX] == -1 {
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 0;
            new.v_radioactivity_exposure = None;
            new.unit_map ^= RADIOACTIVITY_EXPOSURE_MAP;
        } else {
            if self.v_radioactivity_exposure != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_radioactivity_exposure.unwrap()
                );
            }
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitRadioactivityExposure> for Value {
    type Output = Value;
    fn div(self, other: UnitRadioactivityExposure) -> Value {
        let mut new: Value = self;
        if self.v_radioactivity_exposure.is_some() && self.v_radioactivity_exposure != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[RADIOACTIVITY_EXPOSURE_INDEX] == 0 {
            new.v_radioactivity_exposure = Some(other);
            new.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = -1;
        } else if new.exp[RADIOACTIVITY_EXPOSURE_INDEX] == 1 {
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 0;
            new.v_radioactivity_exposure = None;
            new.unit_map ^= RADIOACTIVITY_EXPOSURE_MAP;
        } else {
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitRadioactivity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitRadioactivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity: Some(other),
            unit_map: RADIOACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitRadioactivity> for f64 {
    type Output = Value;
    fn div(self, other: UnitRadioactivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity: Some(other),
            unit_map: RADIOACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitRadioactivity> for Value {
    type Output = Value;
    fn mul(self, other: UnitRadioactivity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[RADIOACTIVITY_INDEX] == 0 {
            new.v_radioactivity = Some(other);
            new.exp[RADIOACTIVITY_INDEX] = 1;
            new.unit_map |= RADIOACTIVITY_MAP;
        } else if self.exp[RADIOACTIVITY_INDEX] == -1 {
            new.exp[RADIOACTIVITY_INDEX] = 0;
            new.v_radioactivity = None;
            new.unit_map ^= RADIOACTIVITY_MAP;
        } else {
            if self.v_radioactivity != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_radioactivity.unwrap()
                );
            }
            new.exp[RADIOACTIVITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitRadioactivity> for Value {
    type Output = Value;
    fn div(self, other: UnitRadioactivity) -> Value {
        let mut new: Value = self;
        if self.v_radioactivity.is_some() && self.v_radioactivity != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[RADIOACTIVITY_INDEX] == 0 {
            new.v_radioactivity = Some(other);
            new.unit_map |= RADIOACTIVITY_MAP;
            new.exp[RADIOACTIVITY_INDEX] = -1;
        } else if new.exp[RADIOACTIVITY_INDEX] == 1 {
            new.exp[RADIOACTIVITY_INDEX] = 0;
            new.v_radioactivity = None;
            new.unit_map ^= RADIOACTIVITY_MAP;
        } else {
            new.exp[RADIOACTIVITY_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitPressure> for f64 {
    type Output = Value;
    fn mul(self, other: UnitPressure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_pressure: Some(other),
            unit_map: PRESSURE_MAP,
            ..Default::default()
        };
        ret.exp[PRESSURE_INDEX] = 1;
        ret
    }
}

impl Div<UnitPressure> for f64 {
    type Output = Value;
    fn div(self, other: UnitPressure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_pressure: Some(other),
            unit_map: PRESSURE_MAP,
            ..Default::default()
        };
        ret.exp[PRESSURE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitPressure> for Value {
    type Output = Value;
    fn mul(self, other: UnitPressure) -> Self::Output {
        let mut new: Value = self;
        if self.exp[PRESSURE_INDEX] == 0 {
            new.v_pressure = Some(other);
            new.exp[PRESSURE_INDEX] = 1;
            new.unit_map |= PRESSURE_MAP;
        } else if self.exp[PRESSURE_INDEX] == -1 {
            new.exp[PRESSURE_INDEX] = 0;
            new.v_pressure = None;
            new.unit_map ^= PRESSURE_MAP;
        } else {
            if self.v_pressure != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_pressure.unwrap()
                );
            }
            new.exp[PRESSURE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitPressure> for Value {
    type Output = Value;
    fn div(self, other: UnitPressure) -> Value {
        let mut new: Value = self;
        if self.v_pressure.is_some() && self.v_pressure != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[PRESSURE_INDEX] == 0 {
            new.v_pressure = Some(other);
            new.unit_map |= PRESSURE_MAP;
            new.exp[PRESSURE_INDEX] = -1;
        } else if new.exp[PRESSURE_INDEX] == 1 {
            new.exp[PRESSURE_INDEX] = 0;
            new.v_pressure = None;
            new.unit_map ^= PRESSURE_MAP;
        } else {
            new.exp[PRESSURE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitPower> for f64 {
    type Output = Value;
    fn mul(self, other: UnitPower) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_power: Some(other),
            unit_map: POWER_MAP,
            ..Default::default()
        };
        ret.exp[POWER_INDEX] = 1;
        ret
    }
}

impl Div<UnitPower> for f64 {
    type Output = Value;
    fn div(self, other: UnitPower) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_power: Some(other),
            unit_map: POWER_MAP,
            ..Default::default()
        };
        ret.exp[POWER_INDEX] = -1;
        ret
    }
}

impl Mul<UnitPower> for Value {
    type Output = Value;
    fn mul(self, other: UnitPower) -> Self::Output {
        let mut new: Value = self;
        if self.exp[POWER_INDEX] == 0 {
            new.v_power = Some(other);
            new.exp[POWER_INDEX] = 1;
            new.unit_map |= POWER_MAP;
        } else if self.exp[POWER_INDEX] == -1 {
            new.exp[POWER_INDEX] = 0;
            new.v_power = None;
            new.unit_map ^= POWER_MAP;
        } else {
            if self.v_power != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_power.unwrap()
                );
            }
            new.exp[POWER_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitPower> for Value {
    type Output = Value;
    fn div(self, other: UnitPower) -> Value {
        let mut new: Value = self;
        if self.v_power.is_some() && self.v_power != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[POWER_INDEX] == 0 {
            new.v_power = Some(other);
            new.unit_map |= POWER_MAP;
            new.exp[POWER_INDEX] = -1;
        } else if new.exp[POWER_INDEX] == 1 {
            new.exp[POWER_INDEX] = 0;
            new.v_power = None;
            new.unit_map ^= POWER_MAP;
        } else {
            new.exp[POWER_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitElectricInductance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricInductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_inductance: Some(other),
            unit_map: INDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[INDUCTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricInductance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricInductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_inductance: Some(other),
            unit_map: INDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[INDUCTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricInductance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricInductance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[INDUCTANCE_INDEX] == 0 {
            new.v_inductance = Some(other);
            new.exp[INDUCTANCE_INDEX] = 1;
            new.unit_map |= INDUCTANCE_MAP;
        } else if self.exp[INDUCTANCE_INDEX] == -1 {
            new.exp[INDUCTANCE_INDEX] = 0;
            new.v_inductance = None;
            new.unit_map ^= INDUCTANCE_MAP;
        } else {
            if self.v_inductance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_inductance.unwrap()
                );
            }
            new.exp[INDUCTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricInductance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricInductance) -> Value {
        let mut new: Value = self;
        if self.v_inductance.is_some() && self.v_inductance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[INDUCTANCE_INDEX] == 0 {
            new.v_inductance = Some(other);
            new.unit_map |= INDUCTANCE_MAP;
            new.exp[INDUCTANCE_INDEX] = -1;
        } else if new.exp[INDUCTANCE_INDEX] == 1 {
            new.exp[INDUCTANCE_INDEX] = 0;
            new.v_inductance = None;
            new.unit_map ^= INDUCTANCE_MAP;
        } else {
            new.exp[INDUCTANCE_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitInformation> for f64 {
    type Output = Value;
    fn mul(self, other: UnitInformation) -> Self::Output {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut ret = Value {
            val: self,
            v_information: Some(other),
            unit_map: INFORMATION_MAP,
            ..Default::default()
        };
        ret.exp[INFORMATION_INDEX] = 1;
        ret
    }
}

impl Div<UnitInformation> for f64 {
    type Output = Value;
    fn div(self, other: UnitInformation) -> Self::Output {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut ret = Value {
            val: self,
            v_information: Some(other),
            unit_map: INFORMATION_MAP,
            ..Default::default()
        };
        ret.exp[INFORMATION_INDEX] = -1;
        ret
    }
}

impl Mul<UnitInformation> for Value {
    type Output = Value;
    fn mul(self, other: UnitInformation) -> Self::Output {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut new: Value = self;
        if self.exp[INFORMATION_INDEX] == 0 {
            new.v_information = Some(other);
            new.exp[INFORMATION_INDEX] = 1;
            new.unit_map |= INFORMATION_MAP;
        } else if self.exp[INFORMATION_INDEX] == -1 {
            new.exp[INFORMATION_INDEX] = 0;
            new.v_information = None;
            new.unit_map ^= INFORMATION_MAP;
        } else {
            if self.v_information != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_information.unwrap()
                );
            }
            new.exp[INFORMATION_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitInformation> for Value {
    type Output = Value;
    fn div(self, other: UnitInformation) -> Value {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut new: Value = self;
        if self.v_information.is_some() && self.v_information != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[INFORMATION_INDEX] == 0 {
            new.v_information = Some(other);
            new.unit_map |= INFORMATION_MAP;
            new.exp[INFORMATION_INDEX] = -1;
        } else if new.exp[INFORMATION_INDEX] == 1 {
            new.exp[INFORMATION_INDEX] = 0;
            new.v_information = None;
            new.unit_map ^= INFORMATION_MAP;
        } else {
            new.exp[INFORMATION_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitLuminousFlux> for f64 {
    type Output = Value;
    fn mul(self, other: UnitLuminousFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux: Some(other),
            unit_map: LUMINOUS_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_FLUX_INDEX] = 1;
        ret
    }
}

impl Div<UnitLuminousFlux> for f64 {
    type Output = Value;
    fn div(self, other: UnitLuminousFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux: Some(other),
            unit_map: LUMINOUS_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_FLUX_INDEX] = -1;
        ret
    }
}

impl Mul<UnitLuminousFlux> for Value {
    type Output = Value;
    fn mul(self, other: UnitLuminousFlux) -> Self::Output {
        let mut new: Value = self;
        if self.exp[LUMINOUS_FLUX_INDEX] == 0 {
            new.v_luminous_flux = Some(other);
            new.exp[LUMINOUS_FLUX_INDEX] = 1;
            new.unit_map |= LUMINOUS_FLUX_MAP;
        } else if self.exp[LUMINOUS_FLUX_INDEX] == -1 {
            new.exp[LUMINOUS_FLUX_INDEX] = 0;
            new.v_luminous_flux = None;
            new.unit_map ^= LUMINOUS_FLUX_MAP;
        } else {
            if self.v_luminous_flux != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_luminous_flux.unwrap()
                );
            }
            new.exp[LUMINOUS_FLUX_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitLuminousFlux> for Value {
    type Output = Value;
    fn div(self, other: UnitLuminousFlux) -> Value {
        let mut new: Value = self;
        if self.v_luminous_flux.is_some() && self.v_luminous_flux != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[LUMINOUS_FLUX_INDEX] == 0 {
            new.v_luminous_flux = Some(other);
            new.unit_map |= LUMINOUS_FLUX_MAP;
            new.exp[LUMINOUS_FLUX_INDEX] = -1;
        } else if new.exp[LUMINOUS_FLUX_INDEX] == 1 {
            new.exp[LUMINOUS_FLUX_INDEX] = 0;
            new.v_luminous_flux = None;
            new.unit_map ^= LUMINOUS_FLUX_MAP;
        } else {
            new.exp[LUMINOUS_FLUX_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitLuminousIntensity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitLuminousIntensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux_intensity: Some(other),
            unit_map: LUMINOUS_INTENSITY_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_INTENSITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitLuminousIntensity> for f64 {
    type Output = Value;
    fn div(self, other: UnitLuminousIntensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux_intensity: Some(other),
            unit_map: LUMINOUS_INTENSITY_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_INTENSITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitLuminousIntensity> for Value {
    type Output = Value;
    fn mul(self, other: UnitLuminousIntensity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[LUMINOUS_INTENSITY_INDEX] == 0 {
            new.v_luminous_flux_intensity = Some(other);
            new.exp[LUMINOUS_INTENSITY_INDEX] = 1;
            new.unit_map |= LUMINOUS_INTENSITY_MAP;
        } else if self.exp[LUMINOUS_INTENSITY_INDEX] == -1 {
            new.exp[LUMINOUS_INTENSITY_INDEX] = 0;
            new.v_luminous_flux_intensity = None;
            new.unit_map ^= LUMINOUS_INTENSITY_MAP;
        } else {
            if self.v_luminous_flux_intensity != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_luminous_flux_intensity.unwrap()
                );
            }
            new.exp[LUMINOUS_INTENSITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitLuminousIntensity> for Value {
    type Output = Value;
    fn div(self, other: UnitLuminousIntensity) -> Value {
        let mut new: Value = self;
        if self.v_luminous_flux_intensity.is_some() && self.v_luminous_flux_intensity != Some(other)
        {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[LUMINOUS_INTENSITY_INDEX] == 0 {
            new.v_luminous_flux_intensity = Some(other);
            new.unit_map |= LUMINOUS_INTENSITY_MAP;
            new.exp[LUMINOUS_INTENSITY_INDEX] = -1;
        } else if new.exp[LUMINOUS_INTENSITY_INDEX] == 1 {
            new.exp[LUMINOUS_INTENSITY_INDEX] = 0;
            new.v_luminous_flux_intensity = None;
            new.unit_map ^= LUMINOUS_INTENSITY_MAP;
        } else {
            new.exp[LUMINOUS_INTENSITY_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitMagneticFlux> for f64 {
    type Output = Value;
    fn mul(self, other: UnitMagneticFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux: Some(other),
            unit_map: MAGNETIC_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_INDEX] = 1;
        ret
    }
}

impl Div<UnitMagneticFlux> for f64 {
    type Output = Value;
    fn div(self, other: UnitMagneticFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux: Some(other),
            unit_map: MAGNETIC_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_INDEX] = -1;
        ret
    }
}

impl Mul<UnitMagneticFlux> for Value {
    type Output = Value;
    fn mul(self, other: UnitMagneticFlux) -> Self::Output {
        let mut new: Value = self;
        if self.exp[MAGNETIC_FLUX_INDEX] == 0 {
            new.v_magnetic_flux = Some(other);
            new.exp[MAGNETIC_FLUX_INDEX] = 1;
            new.unit_map |= MAGNETIC_FLUX_MAP;
        } else if self.exp[MAGNETIC_FLUX_INDEX] == -1 {
            new.exp[MAGNETIC_FLUX_INDEX] = 0;
            new.v_magnetic_flux = None;
            new.unit_map ^= MAGNETIC_FLUX_MAP;
        } else {
            if self.v_magnetic_flux != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_magnetic_flux.unwrap()
                );
            }
            new.exp[MAGNETIC_FLUX_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitMagneticFlux> for Value {
    type Output = Value;
    fn div(self, other: UnitMagneticFlux) -> Value {
        let mut new: Value = self;
        if self.v_magnetic_flux.is_some() && self.v_magnetic_flux != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[MAGNETIC_FLUX_INDEX] == 0 {
            new.v_magnetic_flux = Some(other);
            new.unit_map |= MAGNETIC_FLUX_MAP;
            new.exp[MAGNETIC_FLUX_INDEX] = -1;
        } else if new.exp[MAGNETIC_FLUX_INDEX] == 1 {
            new.exp[MAGNETIC_FLUX_INDEX] = 0;
            new.v_magnetic_flux = None;
            new.unit_map ^= MAGNETIC_FLUX_MAP;
        } else {
            new.exp[MAGNETIC_FLUX_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitMagneticFluxDensity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitMagneticFluxDensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux_density: Some(other),
            unit_map: MAGNETIC_FLUX_DENSITY_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitMagneticFluxDensity> for f64 {
    type Output = Value;
    fn div(self, other: UnitMagneticFluxDensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux_density: Some(other),
            unit_map: MAGNETIC_FLUX_DENSITY_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_DENSITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitMagneticFluxDensity> for Value {
    type Output = Value;
    fn mul(self, other: UnitMagneticFluxDensity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 0 {
            new.v_magnetic_flux_density = Some(other);
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 1;
            new.unit_map |= MAGNETIC_FLUX_DENSITY_MAP;
        } else if self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == -1 {
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
            new.v_magnetic_flux_density = None;
            new.unit_map ^= MAGNETIC_FLUX_DENSITY_MAP;
        } else {
            if self.v_magnetic_flux_density != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_magnetic_flux_density.unwrap()
                );
            }
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitMagneticFluxDensity> for Value {
    type Output = Value;
    fn div(self, other: UnitMagneticFluxDensity) -> Value {
        let mut new: Value = self;
        if self.v_magnetic_flux_density.is_some() && self.v_magnetic_flux_density != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 0 {
            new.v_magnetic_flux_density = Some(other);
            new.unit_map |= MAGNETIC_FLUX_DENSITY_MAP;
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = -1;
        } else if new.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 1 {
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
            new.v_magnetic_flux_density = None;
            new.unit_map ^= MAGNETIC_FLUX_DENSITY_MAP;
        } else {
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] -= 1;
        }
        new
    }
}

impl Mul<UnitMass> for f64 {
    type Output = Value;
    fn mul(self, other: UnitMass) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_mass: Some(other),
            unit_map: MASS_MAP,
            ..Default::default()
        };
        ret.exp[MASS_INDEX] = 1;
        ret
    }
}

impl Div<UnitMass> for f64 {
    type Output = Value;
    fn div(self, other: UnitMass) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_mass: Some(other),
            unit_map: MASS_MAP,
            ..Default::default()
        };
        ret.exp[MASS_INDEX] = -1;
        ret
    }
}

impl Mul<UnitMass> for Value {
    type Output = Value;
    fn mul(self, other: UnitMass) -> Self::Output {
        let mut new: Value = self;
        if self.exp[MASS_INDEX] == 0 {
            new.v_mass = Some(other);
            new.exp[MASS_INDEX] = 1;
            new.unit_map |= MASS_MAP;
        } else if self.exp[MASS_INDEX] == -1 {
            new.exp[MASS_INDEX] = 0;
            new.v_mass = None;
            new.unit_map ^= MASS_MAP;
        } else {
            if self.v_mass != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_mass.unwrap()
                );
            }
            new.exp[MASS_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitMass> for Value {
    type Output = Value;
    fn div(self, other: UnitMass) -> Value {
        let mut new: Value = self;
        if self.v_mass.is_some() && new.v_mass != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[MASS_INDEX] == 0 {
            new.v_mass = Some(other);
            new.unit_map |= MASS_MAP;
            new.exp[MASS_INDEX] = -1;
        } else if new.exp[MASS_INDEX] == 1 {
            new.exp[MASS_INDEX] = 0;
            new.v_mass = None;
            new.unit_map ^= MASS_MAP;
        } else {
            new.exp[MASS_INDEX] -= 1;
        }
        new
    }
}
