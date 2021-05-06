use criterion::{criterion_group, criterion_main, Criterion};
use hardprime::{find_largest_prime, square, square_root};

pub fn bench_square(c: &mut Criterion) {
    let mut group = c.benchmark_group("square function");

    group.bench_function("square (5,13)", |b| b.iter(|| square(5, 13)));

    group.finish();
}

pub fn bench_square_root(c: &mut Criterion) {
    let mut group = c.benchmark_group("square root function");

    group.bench_function("square_root (12,13)", |b| b.iter(|| square_root(12, 13)));

    group.finish();
}

pub fn bench_find_largest_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime generator function");

    group.bench_function("find_largest_prime (8)", |b| {
        b.iter(|| find_largest_prime(8))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_square,
    bench_square_root,
    bench_find_largest_prime
);
criterion_main!(benches);
