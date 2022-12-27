use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use tween::{FixedTweener, Tweener};

fn func() {
    let mut rng = rand::prelude::StdRng::seed_from_u64(0);

    let mut my_boys: Vec<FixedTweener<i32, i32, _>> = (0..100_000)
        .map(|_| Tweener::linear(rng.gen(), rng.gen(), 5).into_fixed(1))
        .collect();

    for _ in 0..5 {
        my_boys.iter_mut().for_each(|v| {
            v.move_next();
        })
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("make 100k", |b| b.iter(func));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
