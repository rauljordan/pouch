use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pouch::Pouch;

pub fn pouch_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pouch");

    group.bench_function("insert", |b| {
        let mut pouch = Pouch::new();
        b.iter(|| {
            pouch.insert(black_box(42));
        });
    });

    group.bench_function("contains", |b| {
        let mut pouch = Pouch::new();
        pouch.insert(42);
        b.iter(|| {
            pouch.contains(black_box(42));
        });
    });

    group.finish();
}

criterion_group!(benches, pouch_benchmark);
criterion_main!(benches);
