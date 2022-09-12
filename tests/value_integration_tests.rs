extern crate v3;

#[cfg(test)]
mod value_usage_tests {
    use v3::values::Value;

    macro_rules! assert_apr {
        ($x:expr, $y:expr, $d:expr) => {
            if f64::max($x, $y) - f64::min($x, $y) > $d {panic!("{:?} {:?}", $x, $y);}
        };
        ($x:expr, $y:expr) => {
            if f64::max($x, $y) - f64::min($x, $y) > 0.000001 {panic!("{:?} {:?}", $x, $y);}
        }
    }

    #[test]
    fn value_create() {
        let n:Value = Value::new(4.5, "m").unwrap();
        let s:String = n.to_string();

        println!("{}", s);

        assert_eq!(s.len()>0, true);
        assert_eq!(s, String::from("4.5 m"));
    }

    #[test]
    fn value_funcs() {
        let n:Value = Value::new(0.5, "rad").unwrap();
        assert_apr!(n.acos().val, 1.0471975511);
        assert_apr!(n.asin().val, 0.5235987755);
        assert_apr!(n.atan().val, 0.4636476090);
        assert_apr!(n.atan2(&n).val, 0.78539816339);
        assert_apr!(n.sin().val, 0.47942553860);
        assert_apr!(n.cos().val, 0.87758256189);
        assert_apr!(n.tan().val, 0.54630248984);
    }

    #[test]
    fn value_math_mul_add() {
        // multiplication
        let v1:Value = Value::new(4.5, "m").unwrap();
        let mut v2:Value = Value::new(4.5, "m").unwrap();
        let multi1:Value = Value::new(50.0, "cm").unwrap();
        let multi2:Value = Value::new(50.0, "cm^2").unwrap();
        let ret3:Value = Value::new(9.0, "m").unwrap();
        let ret:Value = v1 * multi1;
        v2 *= multi1;
        assert_eq!(ret, v2);
        v2 = Value::new(4.5, "m").unwrap();
        let ret:Value = v1 * multi2;
        v2 *= multi2;
        assert_eq!(ret, v2);
        v2 = Value::new(4.5, "m").unwrap();

        let ret2:Value = v1 + v2;
        assert_eq!(ret2, ret3);
    }

    #[test]
    fn value_math_div_sub() {
        let v1:Value = Value::new(15.0, "kg*m/s^2").unwrap();
        let mut v2:Value = Value::new(15.0, "kg*m/s^2").unwrap();
        let v3:Value = Value::new(3.0, "m/s").unwrap();
        let ret:Value = Value::new(5.0, "kg/s").unwrap();
        v2 /= v3;

        assert_eq!(v2, ret);
        assert_eq!(v1/v3, ret);

        let v4:Value = Value::new(6.3, "m").unwrap();
        let mut v5:Value = Value::new(6.3, "m").unwrap();
        let v6:Value = Value::new(24.6, "cm").unwrap();
        let ret:Value = Value::new(6.053999999999999, "m").unwrap();

        v5 -= v6;
        assert_eq!(v4-v6, ret);
        assert_eq!(v5, ret);
    }
}

#[cfg(test)]
mod unit_conversion_tests {
    use v3::{values::Value, units::{UnitLength, Metric, UnitTime}};

    macro_rules! assert_apr {
        ($x:expr, $y:expr, $d:expr) => {
            if f64::max($x, $y) - f64::min($x, $y) > $d {panic!("{:?} {:?}", $x, $y);}
        };
        ($x:expr, $y:expr) => {
            if f64::max($x, $y) - f64::min($x, $y) > 0.000001 {panic!("{:?} {:?}", $x, $y);}
        }
    }

    // We will start with the length tests
    #[test]
    fn convert_length1(){
        let base_val:f64 = 5.0;
        let mut v1:Value = Value::new(base_val, "m").unwrap();
        let mut v2:Value = Value::new(500000000000.0, "m").unwrap();
        let mut v3:Value = Value::new(base_val, "m").unwrap();
        let mut v4:Value = Value::new(base_val, "m").unwrap();
        let mut v5:Value = Value::new(base_val, "m").unwrap();
        let mut v6:Value = Value::new(base_val, "m").unwrap();
        let mut v7:Value = Value::new(50000000000000000.0, "m").unwrap();
        let mut v8:Value = Value::new(base_val, "m").unwrap();
        let mut v9:Value = Value::new(50000000000000000.0, "m").unwrap();
        v1.convert("in").unwrap();
        assert_apr!(v1.val, 196.85039370079);
        v2.convert("AU").unwrap();
        assert_apr!(v2.val,3.342294356);
        v3.convert("Å").unwrap();
        assert_apr!(v3.val, 50000000000.0);
        v4.convert("ft").unwrap();
        assert_apr!(v4.val, 16.4042);
        v5.convert("yd").unwrap();
        assert_apr!(v5.val, 5.4680664917);
        v6.convert("mile").unwrap();
        assert_apr!(v6.val, 0.003106856);
        v7.convert("lyr").unwrap();
        assert_apr!(v7.val, 5.285004170123077);
        v8.convert("m").unwrap();
        assert_apr!(v8.val, 5.0);
        v9.convert("pc").unwrap();
        assert_apr!(v9.val, 1.6203896447221826);
    }

    #[test]
    fn convert_length2(){
        let base_val:f64 = 5.0;
        let mut v1:Value = Value::new(base_val, "yd").unwrap();
        let mut v2:Value = Value::new(500000000000.0, "yd").unwrap();
        let mut v3:Value = Value::new(base_val, "yd").unwrap();
        let mut v4:Value = Value::new(base_val, "yd").unwrap();
        let mut v5:Value = Value::new(base_val, "yd").unwrap();
        let mut v6:Value = Value::new(base_val, "yd").unwrap();
        let mut v7:Value = Value::new(50000000000000000.0, "yd").unwrap();
        let mut v8:Value = Value::new(base_val, "yd").unwrap();
        let mut v9:Value = Value::new(50000000000000000.0, "yd").unwrap();
        v1.convert("in").unwrap();
        assert_apr!(v1.val, 180.0);
        v2.convert("AU").unwrap();
        assert_apr!(v2.val, 3.0561932323);
        v3.convert("Å").unwrap();
        assert_apr!(v3.val, 45720000000.0);
        v4.convert("ft").unwrap();
        assert_apr!(v4.val, 15.0);
        v5.convert("yd").unwrap();
        assert_apr!(v5.val, 5.0);
        v6.convert("mile").unwrap();
        assert_apr!(v6.val, 0.002840909);
        v7.convert("lyr").unwrap();
        assert_apr!(v7.val, 4.8326078131605427);
        v8.convert("m").unwrap();
        assert_apr!(v8.val, 4.572);
        v9.convert("pc").unwrap();
        assert_apr!(v9.val, 1.4816842911339638);
    }

    #[test]
    fn convert_length3(){
        let base_val:f64 = 5.0;
        let mut v1:Value = Value::new(base_val, "in").unwrap();
        let mut v2:Value = Value::new(500000000000.0, "in").unwrap();
        let mut v3:Value = Value::new(base_val, "in").unwrap();
        let mut v4:Value = Value::new(base_val, "in").unwrap();
        let mut v5:Value = Value::new(base_val, "in").unwrap();
        let mut v6:Value = Value::new(base_val, "in").unwrap();
        let mut v7:Value = Value::new(50000000000000000.0, "in").unwrap();
        let mut v8:Value = Value::new(base_val, "in").unwrap();
        let mut v9:Value = Value::new(50000000000000000.0, "in").unwrap();
        v1.convert("in").unwrap();
        assert_apr!(v1.val, 5.0);
        v2.convert("AU").unwrap();
        assert_apr!(v2.val, 3.0561932323/36.0);
        v3.convert("Å").unwrap();
        assert_apr!(v3.val, 45720000000.0/36.0);
        v4.convert("ft").unwrap();
        assert_apr!(v4.val, 15.0/36.0);
        v5.convert("yd").unwrap();
        assert_apr!(v5.val, 5.0/36.0);
        v6.convert("mile").unwrap();
        assert_apr!(v6.val, 0.002840909/36.0);
        v7.convert("lyr").unwrap();
        assert_apr!(v7.val, 4.8326078131605427/36.0);
        v8.convert("m").unwrap();
        assert_apr!(v8.val, 4.572/36.0);
        v9.convert("pc").unwrap();
        assert_apr!(v9.val, 1.4816842911339638/36.0);
    }

    #[test]
    fn convert_length4(){
        let base_val:f64 = 5.0;
        let mut v1:Value = Value::new(base_val, "ft").unwrap();
        let mut v2:Value = Value::new(500000000000.0, "ft").unwrap();
        let mut v3:Value = Value::new(base_val, "ft").unwrap();
        let mut v4:Value = Value::new(base_val, "ft").unwrap();
        let mut v5:Value = Value::new(base_val, "ft").unwrap();
        let mut v6:Value = Value::new(base_val, "ft").unwrap();
        let mut v7:Value = Value::new(50000000000000000.0, "ft").unwrap();
        let mut v8:Value = Value::new(base_val, "ft").unwrap();
        let mut v9:Value = Value::new(50000000000000000.0, "ft").unwrap();
        v1.convert("in").unwrap();
        assert_apr!(v1.val, 5.0*12.0);
        v2.convert("AU").unwrap();
        assert_apr!(v2.val, 3.0561932323/3.0);
        v3.convert("Å").unwrap();
        assert_apr!(v3.val, 45720000000.0/3.0);
        v4.convert("ft").unwrap();
        assert_apr!(v4.val, 15.0/3.0);
        v5.convert("yd").unwrap();
        assert_apr!(v5.val, 5.0/3.0);
        v6.convert("mile").unwrap();
        assert_apr!(v6.val, 0.002840909/3.0);
        v7.convert("lyr").unwrap();
        assert_apr!(v7.val, 4.8326078131605427/3.0);
        v8.convert("m").unwrap();
        assert_apr!(v8.val, 4.572/3.0);
        v9.convert("pc").unwrap();
        assert_apr!(v9.val, 1.4816842911339638/3.0);
    }

    #[test]
    fn convert_length5(){
        let base_val:f64 = 5.0;
        let mut v1:Value = Value::new(base_val, "mile").unwrap();
        let mut v2:Value = Value::new(500000000000.0, "mile").unwrap();
        let mut v3:Value = Value::new(base_val, "mile").unwrap();
        let mut v4:Value = Value::new(base_val, "mile").unwrap();
        let mut v5:Value = Value::new(base_val, "mile").unwrap();
        let mut v6:Value = Value::new(base_val, "mile").unwrap();
        let mut v7:Value = Value::new(50000000000000000.0, "mile").unwrap();
        let mut v8:Value = Value::new(base_val, "mile").unwrap();
        let mut v9:Value = Value::new(50000000000000000.0, "mile").unwrap();
        v1.convert("in").unwrap();
        assert_apr!(v1.val, 316800.0);
        v2.convert("AU").unwrap();
        assert_apr!(v2.val, 5378.90008885);
        v3.convert("Å").unwrap();
        assert_apr!(v3.val, 80467200000000.0);
        v4.convert("ft").unwrap();
        assert_apr!(v4.val, 26400.0);
        v5.convert("yd").unwrap();
        assert_apr!(v5.val, 8800.0);
        v6.convert("mile").unwrap();
        assert_apr!(v6.val, 5.0);
        v7.convert("lyr").unwrap();
        assert_apr!(v7.val, 8505.389751162555);
        v8.convert("m").unwrap();
        assert_apr!(v8.val, 8046.72);
        v9.convert("pc").unwrap();
        assert_apr!(v9.val, 2607.7643523957763);
    }

    #[test]
    fn convert_length6(){
        let base_val:f64 = 50000000000000000.0;
        let mut v1:Value = Value::new(base_val, "Å").unwrap();
        let mut v2:Value = Value::new(base_val, "Å").unwrap();
        let mut v3:Value = Value::new(base_val, "Å").unwrap();
        let mut v4:Value = Value::new(base_val, "Å").unwrap();
        let mut v5:Value = Value::new(base_val, "Å").unwrap();
        let mut v6:Value = Value::new(base_val, "Å").unwrap();
        let mut v7:Value = Value::new(base_val, "Å").unwrap();
        let mut v8:Value = Value::new(base_val, "Å").unwrap();
        let mut v9:Value = Value::new(base_val, "Å").unwrap();
        v1.convert("in").unwrap();
        assert_apr!(v1.val, 196850393.700787);
        v2.convert("AU").unwrap();
        assert_apr!(v2.val, 0.0000334229);
        v3.convert("Å").unwrap();
        assert_apr!(v3.val, base_val);
        v4.convert("ft").unwrap();
        assert_apr!(v4.val, 16404199.4750656);
        v5.convert("yd").unwrap();
        assert_apr!(v5.val, 5468066.4916885);
        v6.convert("mile").unwrap();
        assert_apr!(v6.val, 3106.85596118667);
        v7.convert("lyr").unwrap();
        assert_apr!(v7.val, 0.0000000005285004170123077, 0.000000000000000001);
        v8.convert("m").unwrap();
        assert_apr!(v8.val, 5000000.0);
        v9.convert("pc").unwrap();
        assert_apr!(v9.val, 0.00000000016203896447221824, 0.000000000000000001);
    }

    #[test]
    fn convert_length7(){
        let base_val:f64 = 1.0;
        let mut v1:Value = Value::new(base_val, "AU").unwrap();
        let mut v2:Value = Value::new(base_val, "AU").unwrap();
        let mut v3:Value = Value::new(base_val, "AU").unwrap();
        let mut v4:Value = Value::new(base_val, "AU").unwrap();
        let mut v5:Value = Value::new(base_val, "AU").unwrap();
        let mut v6:Value = Value::new(base_val, "AU").unwrap();
        let mut v7:Value = Value::new(base_val, "AU").unwrap();
        let mut v8:Value = Value::new(base_val, "AU").unwrap();
        let mut v9:Value = Value::new(base_val, "AU").unwrap();
        v1.convert("in").unwrap();
        assert_apr!(v1.val, 5889679948818.9, 1.0);
        v2.convert("AU").unwrap();
        assert_apr!(v2.val, 1.0);
        v3.convert("Å").unwrap();
        assert_apr!(v3.val, 1.49597870700*f64::powf(10.0, 21.0));
        v4.convert("ft").unwrap();
        assert_apr!(v4.val, 490806662401.57477);
        v5.convert("yd").unwrap();
        assert_apr!(v5.val, 163602220800.52493);
        v6.convert("mile").unwrap();
        assert_apr!(v6.val, 92955807.273026);
        v7.convert("lyr").unwrap();
        assert_apr!(v7.val, 0.00001581251);
        v8.convert("m").unwrap();
        assert_apr!(v8.val, 149597870700.0);
        v9.convert("pc").unwrap();
        assert_apr!(v9.val, 0.0000048481376052493);
    }

    #[test]
    fn convert_mass1(){
        let mut v1:Value = Value::new(1.4, "kg").unwrap();
        let mut v2:Value = Value::new(1.4, "kg").unwrap();
        let mut v3:Value = Value::new(1.4, "kg").unwrap();
        v1.convert("lb").unwrap();
        assert_apr!(v1.val, 3.0864712623);
        v2.convert("oz").unwrap();
        assert_apr!(v2.val, 49.383546729413);
        v3.convert("gr").unwrap();
        assert_apr!(v3.val, 21605.301694118);
    }

    #[test]
    fn convert_mass2(){
        let mut v1:Value = Value::new(1.4, "lb").unwrap();
        let mut v2:Value = Value::new(1.4, "lb").unwrap();
        let mut v3:Value = Value::new(1.4, "lb").unwrap();
        v1.convert("g").unwrap();
        assert_apr!(v1.val, 635.029318);
        v2.convert("oz").unwrap();
        assert_apr!(v2.val, 22.4);
        v3.convert("gr").unwrap();
        assert_apr!(v3.val, 9800.0);
    }

    #[test]
    fn convert_mass3(){
        let mut v1:Value = Value::new(56.4, "oz").unwrap();
        let mut v2:Value = Value::new(56.4, "oz").unwrap();
        let mut v3:Value = Value::new(56.4, "oz").unwrap();
        v1.convert("lb").unwrap();
        assert_apr!(v1.val, 3.524999533);
        v2.convert("g").unwrap();
        assert_apr!(v2.val, 1598.91310425);
        v3.convert("gr").unwrap();
        assert_apr!(v3.val, 24675.0);
    }

    #[test]
    fn convert_mass4(){
        let mut v1:Value = Value::new(300.0, "gr").unwrap();
        let mut v2:Value = Value::new(300.0, "gr").unwrap();
        let mut v3:Value = Value::new(300.0, "gr").unwrap();
        v1.convert("lb").unwrap();
        assert_apr!(v1.val, 0.042857137);
        v2.convert("oz").unwrap();
        assert_apr!(v2.val, 0.6857142857);
        v3.convert("g").unwrap();
        assert_apr!(v3.val, 19.439673);
    }

    #[test]
    fn convert_volume(){
        let mut v1:Value = Value::new(2.4, "l").unwrap();
        let mut v2:Value = Value::new(6038.7, "mm^3").unwrap();
        let mut v3:Value = Value::new(6038.7, "in^3").unwrap();
        v1.convert("mm^3").unwrap();
        assert_apr!(v1.val, 2.4);
        v2.convert("l").unwrap();
        assert_apr!(v2.val, 0.0060387);
        v3.convert("l").unwrap();
        assert_apr!(v3.val, 98.956563377);
    }

    #[test]
    fn convert_angle1(){
        let mut v1:Value = Value::new(57.0, "degrees").unwrap();
        let mut v2:Value = Value::new(57.0, "degrees").unwrap();
        let mut v3:Value = Value::new(57.0, "degrees").unwrap();
        v1.convert("rad").unwrap();
        assert_apr!(v1.val, 0.9948377636);
        v2.convert("mil").unwrap();
        assert_apr!(v2.val, 994.8376736);
        v3.convert("moa").unwrap();
        assert_apr!(v3.val, 3420.0);
    }

    #[test]
    fn convert_angle2(){
        let mut v1:Value = Value::new(1.3, "rad").unwrap();
        let mut v2:Value = Value::new(1.3, "rad").unwrap();
        let mut v3:Value = Value::new(1.3, "rad").unwrap();
        v1.convert("degrees").unwrap();
        assert_apr!(v1.val, 74.48451336);
        v2.convert("mil").unwrap();
        assert_apr!(v2.val, 1300.0);
        v3.convert("moa").unwrap();
        assert_apr!(v3.val, 4469.0708020);
    }

    #[test]
    fn convert_angle3(){
        let mut v1:Value = Value::new(6.0, "mil").unwrap();
        let mut v2:Value = Value::new(6.0, "mil").unwrap();
        let mut v3:Value = Value::new(6.0, "mil").unwrap();
        v1.convert("degrees").unwrap();
        assert_apr!(v1.val, 0.3437746770784);
        v2.convert("rad").unwrap();
        assert_apr!(v2.val, 0.006);
        v3.convert("moa").unwrap();
        assert_apr!(v3.val, 20.6264, 0.001);
    }

    #[test]
    fn convert_angle4(){
        let mut v1:Value = Value::new(4.0, "moa").unwrap();
        let mut v2:Value = Value::new(4.0, "moa").unwrap();
        let mut v3:Value = Value::new(4.0, "moa").unwrap();
        v1.convert("degrees").unwrap();
        assert_apr!(v1.val, 0.0666666162);
        v2.convert("rad").unwrap();
        assert_apr!(v2.val, 0.001163552);
        v3.convert("mil").unwrap(); 
        assert_apr!(v3.val, 1.163552);
    }

    #[test]
    fn convert_absorbed_dose1(){
        let mut v1:Value = Value::new(120.0, "Gy").unwrap();
        let mut v2:Value = Value::new(120.0, "Gy").unwrap();
        v1 >>= "rads";
        assert_apr!(v1.val, 12000.0);
        v2 >>= "R";
        assert_apr!(v2.val, 13683.0);
    }

    #[test]
    fn convert_absorbed_dose2(){
        let mut v1:Value = Value::new(120.0, "rads").unwrap();
        let mut v2:Value = Value::new(120.0, "rads").unwrap();
        v1 >>= "Gy";
        assert_apr!(v1.val, 1.2);
        v2 >>= "R";
        assert_apr!(v2.val, 136.829999);
    }

    #[test]
    fn convert_absorbed_dose3(){
        let mut v1:Value = Value::new(120.0, "R").unwrap();
        let mut v2:Value = Value::new(120.0, "R").unwrap();
        v1 >>= "Gy";
        assert_apr!(v1.val, 1.0524);
        v2 >>= "rads";
        assert_apr!(v2.val, 105.2400789);
    }

    #[test]
    fn convert_energy1(){
        let mut v1:Value = Value::new(400.0, "cal").unwrap();
        let mut v2:Value = Value::new(400.0, "cal").unwrap();
        let mut v3:Value = Value::new(400.0, "cal").unwrap();
        let mut v4:Value = Value::new(400.0, "cal").unwrap();
        v1 >>= "J";
        assert_apr!(v1.val, 1673.6);
        v2 >>= "kcal";
        assert_apr!(v2.val, 0.4);
        v3 >>= "eV";
        assert_apr!(v3.val, 1.045*f64::powf(10.0, 22.0), f64::powf(10.0, 20.0));
        v4 >>= "ftlb";
        assert_apr!(v4.val, 1234.383965);
    }

    #[test]
    fn convert_energy2(){
        let mut v1:Value = Value::new(400.0, "kcal").unwrap();
        let mut v2:Value = Value::new(400.0, "kcal").unwrap();
        let mut v3:Value = Value::new(400.0, "kcal").unwrap();
        let mut v4:Value = Value::new(400.0, "kcal").unwrap();
        v1 >>= "J";
        assert_apr!(v1.val, 1673600.0);
        v2 >>= "cal";
        assert_apr!(v2.val, 400000.0);
        v3 >>= "eV";
        assert_apr!(v3.val, 1.045*f64::powf(10.0, 25.0), f64::powf(10.0, 23.0));
        v4 >>= "ftlb";
        assert_apr!(v4.val, 1234383.965989);
    }

    #[test]
    fn convert_energy3(){
        let mut v1:Value = Value::new(400.0, "J").unwrap();
        let mut v2:Value = Value::new(400.0, "J").unwrap();
        let mut v3:Value = Value::new(400.0, "J").unwrap();
        let mut v4:Value = Value::new(400.0, "J").unwrap();
        v1 >>= "cal";
        assert_apr!(v1.val, 95.60229445);
        v2 >>= "kcal";
        assert_apr!(v2.val, 0.095602294);
        v3 >>= "eV";
        assert_apr!(v3.val, 2.497*f64::powf(10.0, 21.0), f64::powf(10.0, 19.0));
        v4 >>= "ftlb";
        assert_apr!(v4.val, 295.024848);
    }

    #[test]
    fn convert_energy4(){
        let mut v1:Value = Value::new(400.0, "ftlb").unwrap();
        let mut v2:Value = Value::new(400.0, "ftlb").unwrap();
        let mut v3:Value = Value::new(400.0, "ftlb").unwrap();
        let mut v4:Value = Value::new(400.0, "ftlb").unwrap();
        v1 >>= "J";
        assert_apr!(v1.val, 542.3272);
        v2 >>= "kcal";
        assert_apr!(v2.val, 0.129619311);
        v3 >>= "eV";
        assert_apr!(v3.val, 3.385*f64::powf(10.0, 21.0), f64::powf(10.0, 19.0));
        v4 >>= "cal";
        assert_apr!(v4.val, 129.619311);
    }

    #[test]
    fn convert_energy5(){
        let mut v1:Value = Value::new(400000000.0, "eV").unwrap();
        let mut v2:Value = Value::new(400000000.0, "eV").unwrap();
        let mut v3:Value = Value::new(400000000.0, "eV").unwrap();
        let mut v4:Value = Value::new(400000000.0, "eV").unwrap();
        v1 >>= "J";
        assert_apr!(v1.val, 6.40870626*f64::powf(10.0, -11.0), f64::powf(10.0, -15.0));
        v2 >>= "kcal";
        assert_apr!(v2.val, 1.5317175574*f64::powf(10.0, -14.0), f64::powf(10.0, -18.0));
        v3 >>= "cal";
        assert_apr!(v3.val, 1.5317175574*f64::powf(10.0, -11.0), f64::powf(10.0, -15.0));
        v4 >>= "ftlb";
        assert_apr!(v4.val, 4.7268191632*f64::powf(10.0, -11.0), f64::powf(10.0, -15.0));
    }

    #[test]
    fn convert_force1(){
        let mut v1:Value = Value::new(145.0, "N").unwrap();
        let mut v2:Value = Value::new(145.0, "lbfr").unwrap();
        v1 >>= "lbfr";
        assert_apr!(v1.val, 32.597296);
        v2 >>= "N";
        assert_apr!(v2.val, 644.992134);
    }

    #[test]
    fn convert_pressure1(){
        let mut v1:Value = Value::new(1.04, "ATM").unwrap();
        let mut v2:Value = Value::new(1.04, "ATM").unwrap();
        let mut v3:Value = Value::new(1.04, "ATM").unwrap();
        let mut v4:Value = Value::new(1.04, "ATM").unwrap();
        let mut v5:Value = Value::new(1.04, "ATM").unwrap();
        let mut v6:Value = Value::new(1.04, "ATM").unwrap();
        v1 >>= "bar";
        assert_apr!(v1.val, 1.05378);
        v2 >>= "inHg";
        assert_apr!(v2.val, 31.118105);
        v3 >>= "mmHg";
        assert_apr!(v3.val, 790.399887);
        v4 >>= "Pa";
        assert_apr!(v4.val, 105378.0);
        v5 >>= "psi";
        assert_apr!(v5.val, 15.283787);
        v6 >>= "torr";
        assert_apr!(v6.val, 790.4);
    }

    #[test]
    fn convert_pressure2(){
        let mut v1:Value = Value::new(1.04, "bar").unwrap();
        let mut v2:Value = Value::new(1.04, "bar").unwrap();
        let mut v3:Value = Value::new(1.04, "bar").unwrap();
        let mut v4:Value = Value::new(1.04, "bar").unwrap();
        let mut v5:Value = Value::new(1.04, "bar").unwrap();
        let mut v6:Value = Value::new(1.04, "bar").unwrap();
        v1 >>= "atm";
        assert_apr!(v1.val, 1.0264001);
        v2 >>= "inHg";
        assert_apr!(v2.val, 30.71118239);
        v3 >>= "mmHg";
        assert_apr!(v3.val, 780.064038);
        v4 >>= "Pa";
        assert_apr!(v4.val, 104000.0);
        v5 >>= "psi";
        assert_apr!(v5.val, 15.0839247);
        v6 >>= "torr";
        assert_apr!(v6.val, 780.06415001);
    }

    #[test]
    fn convert_pressure3(){
        let mut v1:Value = Value::new(28.7, "inHg").unwrap();
        let mut v2:Value = Value::new(28.7, "inHg").unwrap();
        let mut v3:Value = Value::new(28.7, "inHg").unwrap();
        let mut v4:Value = Value::new(28.7, "inHg").unwrap();
        let mut v5:Value = Value::new(28.7, "inHg").unwrap();
        let mut v6:Value = Value::new(28.7, "inHg").unwrap();
        v1 >>= "atm";
        assert_apr!(v1.val, 0.95918444);
        v2 >>= "bar";
        assert_apr!(v2.val, 0.97189354);
        v3 >>= "mmHg";
        assert_apr!(v3.val, 728.980005);
        v4 >>= "Pa";
        assert_apr!(v4.val, 97189.35473142);
        v5 >>= "psi";
        assert_apr!(v5.val, 14.096124);
        v6 >>= "torr";
        assert_apr!(v6.val, 728.980109522);
    }

    #[test]
    fn convert_pressure4(){
        let mut v1:Value = Value::new(728.7, "mmHg").unwrap();
        let mut v2:Value = Value::new(728.7, "mmHg").unwrap();
        let mut v3:Value = Value::new(728.7, "mmHg").unwrap();
        let mut v4:Value = Value::new(728.7, "mmHg").unwrap();
        let mut v5:Value = Value::new(728.7, "mmHg").unwrap();
        let mut v6:Value = Value::new(728.7, "mmHg").unwrap();
        v1 >>= "atm";
        assert_apr!(v1.val, 0.9588159);
        v2 >>= "bar";
        assert_apr!(v2.val, 0.9715202);
        v3 >>= "inHg";
        assert_apr!(v3.val, 28.688977);
        v4 >>= "Pa";
        assert_apr!(v4.val, 97152.02370931);
        v5 >>= "psi";
        assert_apr!(v5.val, 14.09071003);
        v6 >>= "torr";
        assert_apr!(v6.val, 728.7001038);
    }

    #[test]
    fn convert_pressure5(){
        let mut v1:Value = Value::new(80.0, "kPa").unwrap();
        let mut v2:Value = Value::new(80.0, "kPa").unwrap();
        let mut v3:Value = Value::new(80.0, "kPa").unwrap();
        let mut v4:Value = Value::new(80.0, "kPa").unwrap();
        let mut v5:Value = Value::new(80.0, "kPa").unwrap();
        let mut v6:Value = Value::new(80.0, "kPa").unwrap();
        v1 >>= "atm";
        assert_apr!(v1.val, 0.789538);
        v2 >>= "bar";
        assert_apr!(v2.val, 0.8);
        v3 >>= "inHg";
        assert_apr!(v3.val, 23.623986);
        v4 >>= "mmHg";
        assert_apr!(v4.val, 600.049260);
        v5 >>= "psi";
        assert_apr!(v5.val, 11.603019);
        v6 >>= "torr";
        assert_apr!(v6.val, 600.049346);
    }

    #[test]
    fn convert_pressure6(){
        let mut v1:Value = Value::new(80.0, "psi").unwrap();
        let mut v2:Value = Value::new(80.0, "psi").unwrap();
        let mut v3:Value = Value::new(80.0, "psi").unwrap();
        let mut v4:Value = Value::new(80.0, "psi").unwrap();
        let mut v5:Value = Value::new(80.0, "psi").unwrap();
        let mut v6:Value = Value::new(80.0, "psi").unwrap();
        v1 >>= "atm";
        assert_apr!(v1.val, 5.4436771);
        v2 >>= "bar";
        assert_apr!(v2.val, 5.5158058);
        v3 >>= "inHg";
        assert_apr!(v3.val, 162.881645);
        v4 >>= "mmHg";
        assert_apr!(v4.val, 4137.193840);
        v5 >>= "Pa";
        assert_apr!(v5.val, 551580.559999);
        v6 >>= "torr";
        assert_apr!(v6.val, 4137.194429);
    }

    #[test]
    fn convert_pressure7(){
        let mut v1:Value = Value::new(800.0, "torr").unwrap();
        let mut v2:Value = Value::new(800.0, "torr").unwrap();
        let mut v3:Value = Value::new(800.0, "torr").unwrap();
        let mut v4:Value = Value::new(800.0, "torr").unwrap();
        let mut v5:Value = Value::new(800.0, "torr").unwrap();
        let mut v6:Value = Value::new(800.0, "torr").unwrap();
        v1 >>= "atm";
        assert_apr!(v1.val, 1.05263157);
        v2 >>= "bar";
        assert_apr!(v2.val, 1.06657894);
        v3 >>= "inHg";
        assert_apr!(v3.val, 31.4960582);
        v4 >>= "mmHg";
        assert_apr!(v4.val, 799.999886);
        v5 >>= "Pa";
        assert_apr!(v5.val, 106657.894736);
        v6 >>= "psi";
        assert_apr!(v6.val, 15.4694197);
    }

    #[test]
    fn convert_radioactivity1(){
        let mut v1:Value = Value::new(50000000.0, "kBq").unwrap();
        let mut v2:Value = Value::new(50.0, "Ci").unwrap();
        v1 >>= "Ci";
        assert_apr!(v1.val, 1.35135135);
        v2 >>= "kBq";
        assert_apr!(v2.val, 1850000000.0);
    }

    #[test]
    fn convert_radex1(){
        let mut v1:Value = Value::new(175.0, "Sv").unwrap();
        let mut v2:Value = Value::new(175.0, "rem").unwrap();
        v1 >>= "rem";
        assert_apr!(v1.val, 17500.0);
        v2 >>= "Sv";
        assert_apr!(v2.val, 1.75);
    }

    #[test]
    fn convert_exp1(){
        let v1:Value = Value::new(175.0, "m").unwrap();
        let v2:Value = Value::new(175.0, "m").unwrap();
        let v3:Value = v1/v2;
        assert_apr!(v3.val, 1.0);
    }

    #[test]
    fn convert_temp1(){
        let mut t1:Value = Value::new(300.0, "c").unwrap();
        let mut t2:Value = Value::new(300.0, "c").unwrap();

        t1 >>= "°F";
        assert_apr!(t1.val, 572.0);
        t2 >>= "K";
        assert_apr!(t2.val, 573.15);
    }

    #[test]
    fn convert_temp2(){
        let mut t1:Value = Value::new(300.0, "f").unwrap();
        let mut t2:Value = Value::new(300.0, "f").unwrap();

        t1 >>= "c";
        assert_apr!(t1.val, 148.8888888);
        t2 >>= "K";
        assert_apr!(t2.val, 422.038888);
    }

    #[test]
    fn convert_temp3(){
        let mut t1:Value = Value::new(300.0, "K").unwrap();
        let mut t2:Value = Value::new(300.0, "K").unwrap();

        t1 >>= "c";
        assert_apr!(t1.val, 26.85);
        t2 >>= "f";
        assert_apr!(t2.val, 80.33);
    }

    #[test]
    fn convert_information1(){
        let mut t1:Value = Value::new(1024.0, "bytes").unwrap();
        
        t1 >>= "bits";
        assert_apr!(t1.val, 1024.0*8.0);
    }

    #[test]
    fn convert_information2(){
        let mut t1:Value = Value::new(1024.0, "bits").unwrap();
        
        t1 >>= "bytes";
        assert_apr!(t1.val, 1024.0/8.0);
    }

    #[test]
    fn convert_mph(){
        let mut v1:Value = Value::new(20.0, "mph").unwrap();
        let v2:Value = 32.18688 ^ UnitTime::Hour | UnitLength::Meter(Metric::Kilo);
        v1.convert("km/hr").unwrap();
        assert_eq!(v1, v2);
    }
}

#[cfg(test)]
mod value_complex_tests {
    use v3::values::Value;

    #[test]
    fn t1() {
        let m:Value = Value::new(2.5, "kg").unwrap();
        let acc:Value = Value::new(9.81, "m/s^2").unwrap();
        let f:Value = Value::new(24.525000000000002, "N").unwrap();

        let t = (m*acc).complex().unwrap();

        assert_eq!(t, f);
    }

    #[test]
    fn t2() {
        let m:Value = Value::new(2500.0, "g").unwrap();
        let acc:Value = Value::new(9.81, "m/s^2").unwrap();
        let f:Value = Value::new(24.525000000000002, "N").unwrap();

        let t = (m*acc).complex().unwrap();

        assert_eq!(t, f);
    }
}

#[cfg(test)]
mod value_display_tests {
    use v3::{values::Value, units::{UnitLength, Metric, UnitAbsorbedDose, UnitAngle, UnitCapacitance, UnitCatalyticActivity, UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricPotential, UnitEnergy, UnitForce, UnitFrequency, UnitIlluminance, UnitInductance, UnitInformation, UnitLuminousFlux, UnitLuminousIntensity, UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure, UnitRadioactivity, UnitRadioactivityExposure, UnitResistance, UnitSolidAngle, UnitSound, UnitSubstance, UnitTime, UnitVolume}};

    #[test]
    fn t1() {

        let t = UnitLength::Meter;
        let s:&str = "m";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t2() {

        let t = UnitAbsorbedDose::Gray;
        let s:&str = "Gy";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t3() {

        let t = UnitAngle::Radian;
        let s:&str = "rad";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 mil"), x.to_string()); // This one is unique because of milliradians
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t4() {

        let t = UnitCapacitance::Farad;
        let s:&str = "F";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t5() {

        let t = UnitCatalyticActivity::Katal;
        let s:&str = "kat";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t6() {

        let t = UnitElectricCharge::Coulomb;
        let s:&str = "C";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t7() {

        let t = UnitElectricConductance::Siemens;
        let s:&str = "S";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t8() {

        let t = UnitElectricCurrent::Ampere;
        let s:&str = "A";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t9() {

        let t = UnitElectricPotential::Volt;
        let s:&str = "V";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t10() {

        let t = UnitEnergy::Joule;
        let s:&str = "J";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t11() {

        let t = UnitEnergy::GramCalorie;
        let s:&str = "cal";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 Cal"), x.to_string()); // Unique for Cal/cal difference
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t12() {

        let t = UnitForce::Newton;
        let s:&str = "N";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t13() {

        let t = UnitFrequency::Hertz;
        let s:&str = "Hz";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t14() {

        let t = UnitIlluminance::Lux;
        let s:&str = "lx";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t15() {

        let t = UnitInductance::Henry;
        let s:&str = "H";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t16() {

        let t = UnitInformation::Byte;
        let s:&str = "b";

        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t17() {

        let t = UnitInformation::Bit;
        let s:&str = "bits";

        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t18() {

        let t = UnitLuminousFlux::Lumen;
        let s:&str = "lm";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t19() {

        let t = UnitLuminousIntensity::Candela;
        let s:&str = "cd";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t20() {

        let t = UnitMagneticFlux::Weber;
        let s:&str = "Wb";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t21() {

        let t = UnitMagneticFluxDensity::Tesla;
        let s:&str = "T";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t22() {

        let t = UnitMass::Gram;
        let s:&str = "g";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t23() {

        let t = UnitPower::Watt;
        let s:&str = "W";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t24() {

        let t = UnitPressure::Pascal;
        let s:&str = "Pa";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

        #[test]
    fn t25() {

        let t = UnitPressure::Bar;
        let s:&str = "bar";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t26() {

        let t = UnitRadioactivity::Becquerel;
        let s:&str = "Bq";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t27() {

        let t = UnitRadioactivityExposure::Sievert;
        let s:&str = "Sv";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t28() {

        let t = UnitResistance::Ohm;
        let s:&str = "Ω";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t29() {

        let t = UnitSolidAngle::Steradian;
        let s:&str = "sr";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t30() {

        let t = UnitSound::Bel;
        let s:&str = "B";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t31() {

        let t = UnitSubstance::Mole;
        let s:&str = "mol";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t32() {

        let t = UnitTime::Second;
        let s:&str = "s";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }

    #[test]
    fn t33() {

        let t = UnitVolume::Liter;
        let s:&str = "l";

        let x:Value = 3.4 | t(Metric::Yotta);
        assert_eq!(format!("3.4 Y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Atto);
        assert_eq!(format!("3.4 a{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Centi);
        assert_eq!(format!("3.4 c{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deca);
        assert_eq!(format!("3.4 da{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Deci);
        assert_eq!(format!("3.4 d{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Exa);
        assert_eq!(format!("3.4 E{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Femto);
        assert_eq!(format!("3.4 f{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Giga);
        assert_eq!(format!("3.4 G{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Hecto);
        assert_eq!(format!("3.4 h{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Kilo);
        assert_eq!(format!("3.4 k{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Mega);
        assert_eq!(format!("3.4 M{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Micro);
        assert_eq!(format!("3.4 μ{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Milli);
        assert_eq!(format!("3.4 m{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Nano);
        assert_eq!(format!("3.4 n{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::None);
        assert_eq!(format!("3.4 {}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Peta);
        assert_eq!(format!("3.4 P{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Pico);
        assert_eq!(format!("3.4 p{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Tera);
        assert_eq!(format!("3.4 T{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Yocto);
        assert_eq!(format!("3.4 y{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zepto);
        assert_eq!(format!("3.4 z{}", s), x.to_string());
        let x:Value = 3.4 | t(Metric::Zetta);
        assert_eq!(format!("3.4 Z{}", s), x.to_string());
    }
}

#[cfg(test)]
mod value_reduce_tests {
    use v3::values::Value;

    #[test]
    fn t1() {
        let mut f:Value = Value::new(24.525, "N").unwrap();
        let ret:Value = Value::new(24.525, "kg*m/s^2").unwrap();
        f.reduce("kg*m/s^2").unwrap();

        assert_eq!(f, ret);
    }
}