use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day07.rs"]
mod day07;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 07", |b| b.iter(day07::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

