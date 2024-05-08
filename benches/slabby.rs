use criterion::{criterion_group, criterion_main, Criterion};
use slabby::Slab32;

pub fn bench(c: &mut Criterion) {
    c.bench_function("allocate 10k", |b| {
        b.iter(|| {
            let mut s = Slab32::new();
            for i in 0..10000 {
                unsafe { s.insert(i + 1) };
            }
            s
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
