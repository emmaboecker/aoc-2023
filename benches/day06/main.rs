use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day06.rs"]
mod day06;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 06", |b| b.iter(day06::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

