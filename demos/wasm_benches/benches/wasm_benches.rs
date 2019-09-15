#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

pub static BENCH_SIZE: isize = 40;

use wasm_benches::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib_benchmark", |b| b.iter(|| fibonacci(black_box(25))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
