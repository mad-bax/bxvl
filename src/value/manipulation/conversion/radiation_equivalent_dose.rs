use std::ops::{Shr, ShrAssign};

use crate::{
    constants::{RADIOACTIVITY_EXPOSURE_INDEX, RADIOACTIVITY_EXPOSURE_MAP},
    errors::V3Error,
    units::{Convert, UnitRadioactivityExposure},
    value::Value,
};

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

#[cfg(test)]
#[macro_use]
mod conversion_testing {

    // Macro defined here so as to not be part of the release software
    macro_rules! assert_apr {
        ($x:expr, $y:expr, $d:expr) => {
            if f64::max($x, $y) - f64::min($x, $y) > $d {
                panic!("{:?} !~= {:?}", $x, $y);
            }
        };
        ($x:expr, $y:expr) => {
            if f64::max($x, $y) - f64::min($x, $y) > 0.000001 {
                panic!("{:?} !~= {:?}", $x, $y);
            }
        };
    }

    use crate::units::{Metric, UnitLength, UnitRadioactivityExposure};

    const TEST_METRIC: [(Metric, &str); 25] = [
        (Metric::Quetta, "Q"),
        (Metric::Ronna, "R"),
        (Metric::Yotta, "Y"),
        (Metric::Zetta, "Z"),
        (Metric::Exa, "E"),
        (Metric::Peta, "P"),
        (Metric::Tera, "T"),
        (Metric::Giga, "G"),
        (Metric::Mega, "M"),
        (Metric::Kilo, "k"),
        (Metric::Hecto, "h"),
        (Metric::Deca, "da"),
        (Metric::None, ""),
        (Metric::Deci, "d"),
        (Metric::Centi, "c"),
        (Metric::Milli, "m"),
        (Metric::Micro, "μ"),
        (Metric::Nano, "n"),
        (Metric::Pico, "p"),
        (Metric::Femto, "f"),
        (Metric::Atto, "a"),
        (Metric::Zepto, "z"),
        (Metric::Yocto, "y"),
        (Metric::Ronto, "r"),
        (Metric::Quecto, "q"),
    ];

    #[test]
    #[should_panic]
    fn covert_mut_fail() {
        let mut x = 1.0 * UnitLength::Foot;
        x >>= UnitRadioactivityExposure::Sievert(Metric::None);
    }

    #[test]
    fn covert_stat_fail() {
        assert!(
            ((1.0 * UnitLength::Foot) >> UnitRadioactivityExposure::Sievert(Metric::None)).is_err()
        );
    }

    #[test]
    fn unit_conversions_stat_exp1() {
        let t1 = 4.1 * UnitRadioactivityExposure::Sievert(Metric::None);

        for i in TEST_METRIC {
            // Scale the metric value by converting
            let t2 = (t1 >> UnitRadioactivityExposure::Sievert(i.0)).unwrap();

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1.val / t2.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            // Verify that the string units are correct.
            assert_eq!(
                t2.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}Sv", i.1)
            );
        }
    }

    #[test]
    fn unit_conversions_mut_exp1() {
        let mut t1 = 4.1 * UnitRadioactivityExposure::Sievert(Metric::None);
        let t1_orig = 4.1 * UnitRadioactivityExposure::Sievert(Metric::None);

        for i in TEST_METRIC {
            // Scale the metric value by converting
            t1 >>= UnitRadioactivityExposure::Sievert(i.0);

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1_orig.val / t1.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            // Reset a little bit
            t1 = 4.1 * UnitRadioactivityExposure::Sievert(Metric::None);
        }
    }
}
