#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use std::time::{Duration, SystemTime};

extern crate susudb;
use crate::susudb::traits::{SusuDataTrait, SusuDatabaseTrait};

//Defined Benchmark setting
fn short_benchmark() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(3000))
        .measurement_time(Duration::from_millis(5000))
        .nresamples(1000)
        .with_plots()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut database = susudb::SusuDatabase::new();
    database.config("susu_db");

    //Write
    short_benchmark()
        .sample_size(10)
        .bench_function("susudb_write_benchmark", |b| b.iter(|| 
            for key_temp in 0..1200 {
                let new_data =
                    susudb::SusuData::new_data(&format!("key#{}", key_temp), &format!("rust_{}", key_temp));
                database.add(new_data);
            }
        ));

    //Read
    short_benchmark()
        .sample_size(10)
        .bench_function("susudb_read_benchmark", |b| b.iter(|| 
            for key_temp in 0..1200 {
                let value = database.get(&format!("key#{}", key_temp));
                // match value {
                //     Some(val) => println!("Found value: {:?}\n", val),
                //     None => println!("Not found any data\n"),
                // }
            }
        ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
