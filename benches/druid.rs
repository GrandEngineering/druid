use criterion::{Criterion, PlotConfiguration, criterion_group, criterion_main};
fn gen_id() {
    for n in 0..10000 {
        druid::Druid::default();
    }
}
fn gen_idv7() {
    for n in 0..10000 {
        druid::DruidV7::default();
    }
}
fn cuid() {
    for n in 0..10000 {
        cuid::cuid2();
    }
}
fn gen_uuidv4() {
    for n in 0..10000 {
        uuid::Uuid::new_v4();
    }
}
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ID");
    group.nresamples(10000);
    group.bench_function("Druidx10k", |b| b.iter(gen_id));
    group.bench_function("Druidv7x10k", |b| b.iter(gen_idv7));
    group.bench_function("CUIDx10k", |b| b.iter(cuid));
    group.bench_function("UUIDx10k", |b| b.iter(gen_uuidv4));
    group.bench_function("Druid", |b| b.iter(druid::Druid::default));
    group.bench_function("Druidv7", |b| b.iter(druid::DruidV7::default));
    group.bench_function("CUID", |b| b.iter(cuid::cuid2));
    group.bench_function("UUID", |b| b.iter(uuid::Uuid::new_v4));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
