use std::str::FromStr;

use crate::{errors::V3Error, values::Value};

impl FromStr for Value {
    type Err = V3Error;
    fn from_str(s: &str) -> Result<Value, V3Error> {
        if !s.contains(char::is_whitespace) {
            let val: Value = match s.parse::<f64>() {
                Ok(t) => Value::new(t, "").unwrap(),
                Err(_) => {
                    return Err(V3Error::ParsingError("[from_str] float conversion".into()));
                }
            };
            return Ok(val);
        }
        let temp: Vec<&str> = s.split(' ').collect();
        let v: f64 = match temp[0].parse::<f64>() {
            Ok(t) => t,
            Err(_) => {
                return Err(V3Error::ParsingError("[from_str] float conversion".into()));
            }
        };
        Value::new(v, temp[1])
    }
}

#[cfg(test)]
mod from_str_testing {
    use std::str::FromStr;

    use crate::{
        constants::{LENGTH_INDEX, LENGTH_MAP, TIME_INDEX, TIME_MAP},
        units::{Metric, UnitLength, UnitTime},
        values::Value,
    };

    #[test]
    fn from_str_test_01() {
        let v1: Value = Value::from_str("3.4 in/s").unwrap();
        assert_eq!(v1.val, 3.4);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Inch));
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -1);
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
    }

    #[test]
    fn parse_test() {
        let v1 = "3.4 in/s".parse::<Value>().unwrap();
        assert_eq!(v1.val, 3.4);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Inch));
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -1);
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
    }

    #[test]
    fn from_str_test_02() {
        let v1: Value = Value::from_str("3.4").unwrap();
        assert_eq!(v1.val, 3.4);
        assert_eq!(v1.unit_map, 0);
        assert_eq!(v1.v_length, None);
        assert_eq!(v1.v_time, None);
        assert!(v1.is_empty());
    }

    #[test]
    #[should_panic]
    fn from_str_test_03() {
        let _: Value = Value::from_str("3.4.5").unwrap();
    }

    #[test]
    #[should_panic]
    fn from_str_test_04() {
        let _: Value = Value::from_str("3.4.5 in/s").unwrap();
    }
}
