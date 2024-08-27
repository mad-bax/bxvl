use std::ops::{Shr, ShrAssign};

use crate::{
    consts::{LENGTH_INDEX, LENGTH_MAP, VOLUME_INDEX, VOLUME_MAP},
    errors::V3Error,
    units::{Convert, UnitLength},
    Value,
};

impl Shr<UnitLength> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitLength) -> Self::Output {
        let mut n: Value = self;
        if n.unit_map & VOLUME_MAP == VOLUME_MAP && n.exp[VOLUME_INDEX] == 1 {
            n.val *= n.v_volume.unwrap().convert(&other);
            n.v_volume = None;
            n.v_length = Some(other);
            n.unit_map = LENGTH_MAP;
            n.exp[VOLUME_INDEX] = 0;
            n.exp[LENGTH_INDEX] = 3;
            return Ok(n);
        } else if self.unit_map & LENGTH_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_length
            .unwrap()
            .convert(&other)
            .powi(self.exp[LENGTH_INDEX]);
        n.v_length = Some(other);
        Ok(n)
    }
}

impl ShrAssign<UnitLength> for Value {
    fn shr_assign(&mut self, other: UnitLength) {
        if self.unit_map & VOLUME_MAP == VOLUME_MAP && self.exp[VOLUME_INDEX] == 1 {
            self.val *= self.v_volume.unwrap().convert(&other);
            self.v_volume = None;
            self.v_length = Some(other);
            self.unit_map = LENGTH_MAP;
            self.exp[VOLUME_INDEX] = 0;
            self.exp[LENGTH_INDEX] = 3;
            return;
        } else if self.unit_map & LENGTH_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_length
            .unwrap()
            .convert(&other)
            .powi(self.exp[LENGTH_INDEX]);
        self.v_length = Some(other);
    }
}

#[cfg(test)]
#[macro_use]
mod length_conversion_testing {

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

    use crate::{
        consts::{LENGTH_LYR_TO_METER, LENGTH_PC_TO_METER},
        units::{Metric, UnitAngle, UnitLength, UnitVolume},
        Value,
    };

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
        let mut x = 1.0 * UnitAngle::Degree;
        x >>= UnitLength::Foot;
    }

    #[test]
    fn covert_stat_fail() {
        assert!(((1.0 * UnitAngle::Degree) >> UnitLength::Foot).is_err());
    }

    #[test]
    fn unit_volume_conversion1() {
        let t1 = Value::new(4.1, "m^3").unwrap();
        let mut t2 = Value::new(4.1, "m^3").unwrap();

        let t3 = (t1 >> UnitVolume::Liter(Metric::Deci)).unwrap();
        t2 >>= UnitVolume::Liter(Metric::Deci);

        assert_eq!(t3, t2);
        assert_eq!(t2.to_string(), "41000 dl");
    }

    #[test]
    fn unit_volume_conversion2() {
        let t1 = Value::new(4.0, "l").unwrap();
        let mut t2 = Value::new(4.0, "l").unwrap();

        let t3 = (t1 >> UnitLength::Meter(Metric::None)).unwrap();
        t2 >>= UnitLength::Meter(Metric::None);

        assert_eq!(t3, t2);
        assert_eq!(t2.to_string(), "0.004 m^3");
    }

    #[test]
    fn unit_conversions_stat_exp1() {
        let t1 = 4.1 * UnitLength::Meter(Metric::None);

        for i in TEST_METRIC {
            // Scale the metric value by converting
            let t2 = (t1 >> UnitLength::Meter(i.0)).unwrap();

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1.val / t2.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            // Verify that the string units are correct.
            assert_eq!(
                t2.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}m", i.1)
            );

            let t3 = (t1 >> UnitLength::LightYear(i.0)).unwrap();
            let temp = (t1.val / t3.val) / i.0.scale();
            assert_apr!(temp, LENGTH_LYR_TO_METER * 1.0, 2.1);
            assert_eq!(
                t3.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}lyr", i.1)
            );

            let t4 = (t1 >> UnitLength::Parsec(i.0)).unwrap();
            let temp = (t1.val / t4.val) / i.0.scale();
            assert_apr!(temp, LENGTH_PC_TO_METER * 1.0, 4.0);
            assert_eq!(
                t4.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}pc", i.1)
            );
        }

        let t2 = (t1 >> UnitLength::Inch).unwrap();
        assert_apr!(t2.val, 161.4173, 0.0001);
        let t2 = (t1 >> UnitLength::Foot).unwrap();
        assert_apr!(t2.val, 13.45144, 0.00001);
        let t2 = (t1 >> UnitLength::Yard).unwrap();
        assert_apr!(t2.val, 4.483815, 0.000001);
        let t2 = (t1 >> UnitLength::Mile).unwrap();
        assert_apr!(t2.val, 0.002547622);
        let t2 = ((t1 * 1.0e15_f64) >> UnitLength::AstronomicalUnit).unwrap();
        assert_apr!(t2.val, 27406.81, 0.01);
        let t2 = (t1 >> UnitLength::Angstrom).unwrap();
        assert_apr!(t2.val, 4.1e+10);
    }

    #[test]
    fn unit_conversions_mut_exp1() {
        let mut t1 = 4.1 * UnitLength::Meter(Metric::None);
        let t1_orig = 4.1 * UnitLength::Meter(Metric::None);

        for i in TEST_METRIC {
            // Scale the metric value by converting
            t1 >>= UnitLength::Meter(i.0);

            // Then divide the values and divide by the scaling. This should
            // bring the value back to the original assignment. This value
            // is checked within a tolerance of 7 decimal places.
            let temp = (t1_orig.val / t1.val) / i.0.scale();
            assert_apr!(temp, 1.0);

            t1 >>= UnitLength::LightYear(i.0);
            let temp = (t1_orig.val / t1.val) / i.0.scale();
            assert_apr!(temp, LENGTH_LYR_TO_METER * 1.0, 2.1);

            t1 >>= UnitLength::Parsec(i.0);
            let temp = (t1_orig.val / t1.val) / i.0.scale();
            assert_apr!(temp, LENGTH_PC_TO_METER * 1.0, 12.0);

            // Reset a little bit
            t1 = 4.1 * UnitLength::Meter(Metric::None);
        }

        // Reset a little bit
        t1 = 4.1 * UnitLength::Meter(Metric::None);

        t1 >>= UnitLength::Inch;
        assert_apr!(t1.val, 161.4173, 0.0001);
        t1 >>= UnitLength::Foot;
        assert_apr!(t1.val, 13.45144, 0.00001);
        t1 >>= UnitLength::Yard;
        assert_apr!(t1.val, 4.483815, 0.000001);
        t1 >>= UnitLength::Mile;
        assert_apr!(t1.val, 0.002547622);
        t1 >>= UnitLength::Angstrom;
        assert_apr!(t1.val, 4.1e+10);
    }
}
