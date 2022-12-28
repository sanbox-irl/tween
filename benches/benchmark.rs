use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tween::{BounceIn, Tweener};

fn func(i: &mut [Tweener<f32, f32, BounceIn>]) {
    for tweener in i.iter_mut() {
        tweener.move_to(5.5);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut input: Vec<_> = (0..1_048_576).map(|_| Tweener::bounce_in(0.0, 10.0, 10.0)).collect();

    c.bench_function("simd come thru", |b| {
        b.iter(|| {
            func(&mut input);
        });

        black_box(&input);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
