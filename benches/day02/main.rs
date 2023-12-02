use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day02.rs"]
mod day02;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 2", |b| b.iter(day02::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
