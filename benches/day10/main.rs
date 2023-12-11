use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day10.rs"]
mod day10;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 10", |b| b.iter(day10::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

