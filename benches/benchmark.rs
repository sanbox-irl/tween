use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tween::{BounceIn, FixedTweener, Tweener};

fn func(i: &mut [FixedTweener<f32, f32, BounceIn>]) {
    for tweener in i.iter_mut() {
        tweener.move_next();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut input: Vec<_> = (0..1048576)
        .map(|_| Tweener::bounce_in(0.0, 10.0, 10.0).into_fixed(1.0))
        .collect();

    c.bench_function("simd come thru", |b| {
        for v in input.iter_mut() {
            v.move_to(0.0);
        }

        b.iter(|| {
            func(&mut input);
        });

        black_box(&input);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
