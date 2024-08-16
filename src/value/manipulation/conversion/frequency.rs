use std::ops::{Shr, ShrAssign};

use crate::{
    constants::{FREQUENCY_INDEX, FREQUENCY_MAP, TIME_INDEX, TIME_MAP},
    errors::V3Error,
    units::{Convert, UnitFrequency},
    value::Value,
};

impl Shr<UnitFrequency> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitFrequency) -> Self::Output {
        let mut n: Value = self;
        if n.unit_map & TIME_MAP == TIME_MAP && n.exp[TIME_INDEX] == -1 {
            n.val *= self.v_time.unwrap().convert(&other);
            n.v_time = None;
            n.v_frequency = Some(other);
            n.exp[TIME_INDEX] = 0;
            n.exp[FREQUENCY_INDEX] = 1;
            n.unit_map &= !TIME_MAP;
            n.unit_map |= FREQUENCY_MAP;
            return Ok(n);
        } else if self.unit_map & FREQUENCY_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_frequency
            .unwrap()
            .convert(&other)
            .powi(self.exp[FREQUENCY_INDEX]);
        n.v_frequency = Some(other);
        Ok(n)
    }
}

impl ShrAssign<UnitFrequency> for Value {
    fn shr_assign(&mut self, other: UnitFrequency) {
        if self.unit_map & TIME_MAP == TIME_MAP && self.exp[TIME_INDEX] == -1 {
            self.val *= self.v_time.unwrap().convert(&other);
            self.v_time = None;
            self.v_frequency = Some(other);
            self.exp[TIME_INDEX] = 0;
            self.exp[FREQUENCY_INDEX] = 1;
            self.unit_map &= !TIME_MAP;
            self.unit_map |= FREQUENCY_MAP;
            return;
        } else if self.unit_map & FREQUENCY_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_frequency
            .unwrap()
            .convert(&other)
            .powi(self.exp[FREQUENCY_INDEX]);
        self.v_frequency = Some(other);
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

    use crate::units::{Metric, UnitFrequency, UnitLength, UnitTime};

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
        x >>= UnitFrequency::Hertz(Metric::None);
    }

    #[test]
    fn covert_stat_fail() {
        assert!(((1.0 * UnitLength::Foot) >> UnitFrequency::Hertz(Metric::None)).is_err());
    }

    #[test]
    fn unit_conversions_stat_exp1() {
        let t1 = 4.1 * UnitFrequency::Hertz(Metric::None);
        let t1_2 = 4.1 / UnitTime::Second(Metric::None) * UnitLength::Meter(Metric::None);

        let t2 = (t1_2 >> UnitFrequency::Hertz(Metric::None)).unwrap();
        assert_eq!(t2.to_string(), "4.1 m*Hz");

        for i in TEST_METRIC {
            // Scale the metric value by converting
            let t2 = (t1 >> UnitFrequency::Hertz(i.0)).unwrap();

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1.val / t2.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            // Verify that the string units are correct.
            assert_eq!(
                t2.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}Hz", i.1)
            );

            let t2 = 4.1 / UnitTime::Second(i.0);

            let t3 = (t2 >> UnitFrequency::Hertz(i.0)).unwrap();
            assert_apr!(t3.val, t2.val);
            assert_eq!(
                t3.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}Hz", i.1)
            );
        }
    }

    #[test]
    fn unit_conversions_mut_exp1() {
        let mut t1 = 4.1 * UnitFrequency::Hertz(Metric::None);
        let t1_orig = 4.1 * UnitFrequency::Hertz(Metric::None);

        let mut t1_2 = 4.1 / UnitTime::Second(Metric::None) * UnitLength::Meter(Metric::None);

        t1_2 >>= UnitFrequency::Hertz(Metric::None);
        assert_eq!(t1_2.to_string(), "4.1 m*Hz");

        for i in TEST_METRIC {
            // Scale the metric value by converting
            t1 >>= UnitFrequency::Hertz(i.0);

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1_orig.val / t1.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            // Reset a little bit
            t1 = 4.1 * UnitFrequency::Hertz(Metric::None);
        }
    }
}
