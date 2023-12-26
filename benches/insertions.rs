use criterion::{black_box, criterion_group, criterion_main, Criterion};
use purse::Purse;

pub fn purse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Purse");

    group.bench_function("insert", |b| {
        let mut purse = Purse::new();
        b.iter(|| {
            purse.insert(black_box(42));
        });
    });

    group.bench_function("contains", |b| {
        let mut purse = Purse::new();
        purse.insert(42);
        b.iter(|| {
            purse.contains(black_box(42));
        });
    });

    group.finish();
}

criterion_group!(benches, purse_benchmark);
criterion_main!(benches);
