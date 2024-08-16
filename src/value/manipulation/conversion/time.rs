use std::ops::{Shr, ShrAssign};

use crate::{
    constants::{FREQUENCY_INDEX, FREQUENCY_MAP, TIME_INDEX, TIME_MAP},
    errors::V3Error,
    units::{Convert, UnitTime},
    value::Value,
};

impl Shr<UnitTime> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitTime) -> Self::Output {
        let mut n: Value = self;
        if n.unit_map & FREQUENCY_MAP == FREQUENCY_MAP && n.exp[FREQUENCY_INDEX] == 1 {
            n.val *= n.v_frequency.unwrap().convert(&other);
            n.v_frequency = None;
            n.v_time = Some(other);
            n.unit_map &= !FREQUENCY_MAP;
            n.unit_map |= TIME_MAP;
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

impl ShrAssign<UnitTime> for Value {
    fn shr_assign(&mut self, other: UnitTime) {
        if self.unit_map & FREQUENCY_MAP == FREQUENCY_MAP && self.exp[FREQUENCY_INDEX] == 1 {
            self.val *= self.v_frequency.unwrap().convert(&other);
            self.v_frequency = None;
            self.v_time = Some(other);
            self.unit_map &= !FREQUENCY_MAP;
            self.unit_map |= TIME_MAP;
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
        x >>= UnitTime::Second(Metric::None);
    }

    #[test]
    fn covert_stat_fail() {
        assert!(((1.0 * UnitLength::Foot) >> UnitTime::Second(Metric::None)).is_err());
    }

    #[test]
    fn unit_conversions_stat_exp1() {
        let t1 = 4.1 * UnitTime::Second(Metric::None);
        let t1_2 = 4.1 * UnitFrequency::Hertz(Metric::None) * UnitLength::Meter(Metric::None);

        let t2 = (t1_2 >> UnitTime::Second(Metric::None)).unwrap();
        assert_eq!(t2.to_string(), "4.1 m/s");

        for i in TEST_METRIC {
            // Scale the metric value by converting
            let t2 = (t1 >> UnitTime::Second(i.0)).unwrap();

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1.val / t2.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            // Verify that the string units are correct.
            assert_eq!(
                t2.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}s", i.1)
            );

            let t2 = 4.1 * UnitFrequency::Hertz(i.0);
            let t3 = (t2 >> UnitTime::Second(i.0)).unwrap();
            assert_apr!(t3.val, t2.val);
            assert_eq!(
                t3.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("1/{}s", i.1)
            );
        }

        let t2 = 90.0 * UnitTime::Second(Metric::None);
        let t3 = (t2 >> UnitTime::Minute).unwrap();
        assert_eq!(t3.to_string(), "1.5 min");
        let t2 = 720.0 * UnitTime::Second(Metric::None);
        let t3 = (t2 >> UnitTime::Minute).unwrap();
        assert_eq!(t3.to_string(), "12 min");
        let t3 = (t2 >> UnitTime::Hour).unwrap();
        assert_eq!(t3.to_string(), "0.2 hr");
        let t3 = (t2 >> UnitTime::Day).unwrap();
        assert_apr!(t3.val, 0.0083333, 0.00001);
    }

    #[test]
    fn unit_conversions_mut_exp1() {
        let mut t1 = 4.1 * UnitTime::Second(Metric::None);
        let t1_orig = 4.1 * UnitTime::Second(Metric::None);

        let mut t1_2 = 4.1 * UnitFrequency::Hertz(Metric::None) * UnitLength::Meter(Metric::None);

        t1_2 >>= UnitTime::Second(Metric::None);
        assert_eq!(t1_2.to_string(), "4.1 m/s");

        for i in TEST_METRIC {
            // Scale the metric value by converting
            t1 >>= UnitTime::Second(i.0);

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1_orig.val / t1.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            // Reset a little bit
            t1 = 4.1 * UnitTime::Second(Metric::None);
        }
    }
}
