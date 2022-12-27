use criterion::{criterion_group, criterion_main, Criterion};
use tween::Tweener;

fn func() {
    let my_boys: [_; 10] = std::array::from_fn(|_| Tweener::bounce_in(0.0f32, 10.0, 3.0f32).into_fixed(1.0));

    for mut my_boy in my_boys {
        for _ in 0..3 {
            my_boy.move_next();
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("handle 10", |b| b.iter(func));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
