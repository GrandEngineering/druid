use criterion::{Criterion, PlotConfiguration, criterion_group, criterion_main};
use rayon::prelude::*;
use std::time::Duration;
fn gen_id() {
    (0..10).collect::<Vec<u64>>().par_iter().for_each(|_| {
        for i in 0..100 {
            druid::Druid::default();
        }
    });
}
fn gen_idv7() {
    (0..10).collect::<Vec<u64>>().par_iter().for_each(|_| {
        for i in 0..100 {
            druid::DruidV7::default();
        }
    });
}
fn cuid() {
    (0..10).collect::<Vec<u64>>().par_iter().for_each(|_| {
        for i in 0..100 {
            cuid2::cuid();
        }
    });
}
fn gen_uuidv4() {
    (0..10).collect::<Vec<u64>>().par_iter().for_each(|_| {
        for i in 0..100 {
            uuid::Uuid::new_v4();
        }
    });
}
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ID");
    group.sample_size(10_000);
    group.bench_function("Druidx10k", |b| b.iter(gen_id));
    group.bench_function("Druidv7x10k", |b| b.iter(gen_idv7));
    group.bench_function("CUIDx10k", |b| b.iter(cuid));
    group.bench_function("UUIDx10k", |b| b.iter(gen_uuidv4));
    group.bench_function("Druid", |b| b.iter(druid::Druid::default));
    group.bench_function("Druidv7", |b| b.iter(druid::DruidV7::default));
    group.bench_function("CUID", |b| b.iter(cuid2::cuid));
    group.bench_function("UUID", |b| b.iter(uuid::Uuid::new_v4));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
