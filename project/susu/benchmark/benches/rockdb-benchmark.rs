#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use std::time::{Duration, SystemTime};

use banchmark::rocksdb_write;
use banchmark::rocksdb_read;

//Defined Benchmark setting
fn short_benchmark() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(3000))
        .measurement_time(Duration::from_millis(5000))
        .nresamples(1000)
        .with_plots()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    //Write
    short_benchmark()
         .sample_size(10)
        .bench_function("rocksdb_write_benchmark", |b| b.iter(|| rocksdb_write()));

    //Read
    short_benchmark()
        .sample_size(10)
        .bench_function("rocksdb_read_benchmark", |b| b.iter(|| rocksdb_read()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);