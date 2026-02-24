use criterion::{Criterion, criterion_group, criterion_main};
use tween::{Linear, Tweener};

#[inline(never)]
#[unsafe(no_mangle)]
fn bencher_function(i: &mut [Tweener<f32, f32, Linear>]) {
    let mut output = 0.0;
    for tweener in i.iter_mut() {
        output += tweener.move_to(5.5);
    }

    std::hint::black_box(output);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut input: Vec<_> = (0..100_048_576).map(|_| Tweener::linear(0.0, 10.0, 10.0)).collect();

    c.bench_function("simd come thru", |b| {
        b.iter(|| {
            bencher_function(&mut input);
        });

        std::hint::black_box(&input);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
