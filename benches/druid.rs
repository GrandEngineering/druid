use criterion::{Criterion, PlotConfiguration, criterion_group, criterion_main};
fn gen_id() {
    druid::Druid::default();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ID");
    group.bench_function("Druid", |b| b.iter(gen_id));
    group.bench_function("CUID", |b| b.iter(cuid::cuid2));
    group.bench_function("UUID", |b| b.iter(uuid::Uuid::new_v4));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
