#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

pub static BENCH_SIZE: isize = 10;
pub static SAMPLE_SIZE: isize = 10;

//use rockdb_research::fibonacci;
use rockdb_research::rocksdb_write;
use rockdb_research::rocksdb_read;

pub fn criterion_benchmark(c: &mut Criterion) {
    //Todo: first, we need to set the sample_size. 
    //c.sample_size(10);

    //RocksDB
    //c.bench_function("rocksdb_write_benchmark", |b| b.iter(|| rocksdb_write()));
    c.bench_function("rocksdb_read_benchmark", |b| b.iter(|| rocksdb_read()));

    //SusuDB
    //Todo
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);