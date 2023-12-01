use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day01.rs"]
mod day01;
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(day01::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
