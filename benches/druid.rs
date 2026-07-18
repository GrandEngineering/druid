use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rayon::prelude::*;
use std::hint::black_box;

const BATCH_SIZE: u64 = 1_000;

fn parallel_batch<T: Send>(generate: impl Fn() -> T + Sync) {
    (0..BATCH_SIZE).into_par_iter().for_each(|_| {
        black_box(generate());
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut single = c.benchmark_group("single ID");
    single.bench_function("Druid", |b| b.iter(|| black_box(druid::Druid::new())));
    single.bench_function("Druid UUIDv7", |b| {
        b.iter(|| black_box(druid::DruidV7::new()))
    });
    single.bench_function("CUID2", |b| b.iter(|| black_box(cuid2::cuid())));
    single.bench_function("UUIDv4", |b| b.iter(|| black_box(uuid::Uuid::new_v4())));
    single.finish();

    let mut batch = c.benchmark_group("parallel batch");
    batch.sample_size(100);
    batch.throughput(Throughput::Elements(BATCH_SIZE));
    batch.bench_function(BenchmarkId::new("Druid", BATCH_SIZE), |b| {
        b.iter(|| parallel_batch(druid::Druid::new));
    });
    batch.bench_function(BenchmarkId::new("Druid UUIDv7", BATCH_SIZE), |b| {
        b.iter(|| parallel_batch(druid::DruidV7::new));
    });
    batch.bench_function(BenchmarkId::new("CUID2", BATCH_SIZE), |b| {
        b.iter(|| parallel_batch(cuid2::cuid));
    });
    batch.bench_function(BenchmarkId::new("UUIDv4", BATCH_SIZE), |b| {
        b.iter(|| parallel_batch(uuid::Uuid::new_v4));
    });
    batch.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
