use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day03.rs"]
mod day03;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 3", |b| b.iter(day03::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
