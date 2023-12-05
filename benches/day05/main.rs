use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day05.rs"]
mod day05;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 5", |b| b.iter(day05::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
