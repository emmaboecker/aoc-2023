use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../../src/bin/day08.rs"]
mod day08;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 08", |b| b.iter(day08::main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

