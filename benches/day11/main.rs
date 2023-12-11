use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day11.rs"]
mod day11;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 11", |b| b.iter(day11::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

