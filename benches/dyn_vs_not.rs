use criterion::{Criterion, black_box, criterion_group, criterion_main};
use tween::{Linear, Tween, Tweener};

fn static_access(tweeners: &mut [Tweener<i32, i32, Linear>]) {
    let mut output = 0;
    for tweener in tweeners.iter_mut() {
        for _ in 0..10 {
            output = tweener.move_to(5);
        }
    }

    black_box(output);
}

fn dynamic_access(tweeners: &mut [Tweener<i32, i32, Box<dyn Tween<i32>>>]) {
    let mut output = 0;
    for tweener in tweeners.iter_mut() {
        for _ in 0..10 {
            output = tweener.move_to(5);
        }
    }

    black_box(output);
}

fn bench_static_vs_dyn(c: &mut Criterion) {
    let mut static_tweener: Vec<Tweener<i32, i32, Linear>> =
        (0..100_000).map(|_| Tweener::new(0, 10, 10, Linear)).collect();

    let mut dyn_tweeners: Vec<Tweener<i32, i32, Box<dyn Tween<i32>>>> = (0..100_000)
        .map(|_| Tweener::new(0, 10, 10, Box::new(Linear) as Box<dyn Tween<i32>>))
        .collect();

    let mut group = c.benchmark_group("Linear Tween");
    group.bench_function("Static", |b| b.iter(|| static_access(&mut static_tweener)));
    group.bench_function("Dynamic", |b| b.iter(|| dynamic_access(&mut dyn_tweeners)));
    group.finish();
}

criterion_group!(static_vs_dyn, bench_static_vs_dyn);
criterion_main!(static_vs_dyn);
