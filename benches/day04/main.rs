use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day04.rs"]
mod day04;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 4", |b| b.iter(day04::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
