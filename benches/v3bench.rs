use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate v3;

use v3::{
    units::{length::UnitLength, substance::UnitSubstance, Convert, Metric, UnitAngle},
    value,
    values::Value,
};

fn criterion_benchmark(c: &mut Criterion) {
    let va1: Value = black_box(Value::new(1.2, "s").unwrap());
    let va2: Value = black_box(Value::new(10.9, "m/s").unwrap());
    let mut va3_1: Value = black_box(Value::new(10.9, "m/s").unwrap());
    let mut va3_2: Value = black_box(Value::new(10.9, "m/s").unwrap());
    let va3_3: Value = black_box(Value::new(10.9, "ft/s").unwrap());
    let va4: Value = black_box(Value::new(3.4, "m/s").unwrap());
    let va5: Value = black_box(Value::new(3.4, "m").unwrap());
    let u1 = black_box(UnitLength::Meter(Metric::Kilo));
    let u2 = black_box(UnitLength::Mile);
    let u3 = black_box(UnitLength::Meter(Metric::Milli));
    let u4 = black_box(UnitLength::Angstrom);
    let u5 = black_box(UnitLength::Parsec(Metric::None));
    c.bench_function("new (best case)", |b| {
        b.iter(|| Value::new(black_box(20.0), "m"))
    });
    c.bench_function("new (worst case)", |b| {
        b.iter(|| Value::new(black_box(20.0), "damol/yrad"))
    });
    c.bench_function("value! (best case)", |b| {
        b.iter(|| value!(black_box(20.0), "m"))
    });
    c.bench_function("value! (worst case)", |b| {
        b.iter(|| value!(black_box(20.0), "damol/yrad"))
    });
    c.bench_function("new (static best)", |b| {
        b.iter(|| 20.0 * UnitLength::Meter(Metric::Kilo))
    });
    c.bench_function("new (static worst)", |b| {
        b.iter(|| 20.0 * UnitSubstance::Mole(Metric::Deca) * UnitAngle::Radian(Metric::Yocto))
    });
    c.bench_function("eq (true)", |b| b.iter(|| va3_1 == va3_2));
    c.bench_function("eq (false)", |b| b.iter(|| va3_1 == va1));
    c.bench_function("from_str", |b| b.iter(|| "10.0 ft/s".parse::<Value>()));
    c.bench_function("convert from value", |b| b.iter(|| va3_1 >> va3_3));
    c.bench_function("mul", |b| b.iter(|| black_box(va1 * va2)));
    c.bench_function("mul as pow", |b| b.iter(|| va1 * va1));
    c.bench_function("powv as pow", |b| b.iter(|| va1.powv(2)));
    c.bench_function("div", |b| b.iter(|| black_box(va5 / va1)));
    c.bench_function("add", |b| b.iter(|| black_box(va4 + va2)));
    c.bench_function("sub", |b| b.iter(|| black_box(va2 - va4)));
    c.bench_function("convert_str", |b| b.iter(|| black_box(va3_1 >>= "ft/s")));
    c.bench_function("convert_value", |b| b.iter(|| black_box(va3_2 >>= va3_3)));
    c.bench_function("convert_direct", |b| {
        b.iter(|| black_box(va3_2 >>= UnitLength::Foot))
    });
    c.bench_function("unit_conversion km   - mile", |b| {
        b.iter(|| black_box(u1.convert(&u2)))
    });
    c.bench_function("unit_conversion mile - km", |b| {
        b.iter(|| black_box(u2.convert(&u1)))
    });
    c.bench_function("unit_conversion km - mm", |b| {
        b.iter(|| black_box(u1.convert(&u3)))
    });
    c.bench_function("unit_conversion mm - km", |b| {
        b.iter(|| black_box(u3.convert(&u1)))
    });
    c.bench_function("unit_conversion a  - pc", |b| {
        b.iter(|| black_box(u4.convert(&u5)))
    });
    c.bench_function("unit_conversion pc - a", |b| {
        b.iter(|| black_box(u5.convert(&u4)))
    });
    c.bench_function("Unit str Kilometer", |b| b.iter(|| format!("{}", u1)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
