use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hyperstar::{biguint, biguint_arr, Number};


criterion_group!(benches, base_conversion);

criterion_main!(benches);
fn base_conversion(c: &mut Criterion) {

    let number = Number::from(
        biguint_arr!(3).to_vec(),
        biguint_arr!(1, 4, 1, 5, 1, 3, 7, 1, 7, 1, 5, 4, 3).to_vec(),
        biguint!(8),
    );

    c.bench_function("fake pi", |b| {
        b.iter(|| black_box(number.to_base(biguint!(10))))
    });

    let number = Number::new("0.5");

    c.bench_function("repeating 1000 digits", |b| {
        b.iter(|| black_box(number.to_base(biguint!(3))))
    });


    let number = Number::new(include_str!("digits_of_pi.txt"));

    c.bench_function("digits of pi", |b| {
        b.iter(|| number.to_base(biguint!(26)))
    });
}
