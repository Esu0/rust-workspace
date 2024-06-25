use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::{Distribution, Standard}, rngs::StdRng, Rng, SeedableRng};
use opt_div::FromStrFast;

fn random_integer_string<I: ToString, R: Rng>(n: usize, rng: &mut R) -> Vec<String>
where
    Standard: Distribution<I>,
{
    (0..n).map(|_| rng.gen::<I>().to_string()).collect()
}

fn bench_fast_parse(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(100);
    let data = random_integer_string::<u32, _>(100000, &mut rng);
    c.benchmark_group("string to integer").bench_function("FromStr::parse", |b| {
        b.iter(|| {
            for s in &data {
                let s = black_box(s.as_str());
                black_box(s.parse::<u32>().unwrap());
            }
        })
    }).bench_function("FromStrFast::parse_fast", |b| {
        b.iter(|| {
            for s in &data {
                let s = black_box(s.as_str());
                black_box(u32::parse_fast(s).unwrap());
            }
        })
    }).bench_function("FromStr::parse unchecked", |b| {
        b.iter(|| {
            for s in &data {
                let s = black_box(s.as_str());
                black_box(unsafe { s.parse::<u32>().unwrap_unchecked() });
            }
        })
    }).bench_function("FromStrFast::parse_fast unchecked", |b| {
        b.iter(|| {
            for s in &data {
                let s = black_box(s.as_str());
                black_box(unsafe { u32::parse_fast_unchecked(s) });
            }
        })
    });
}

criterion_group!(bench, bench_fast_parse);
criterion_main!(bench);
