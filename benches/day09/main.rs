use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day09.rs"]
mod day09;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 09", |b| b.iter(day09::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

