use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hardprime::{find_largest_prime, square, square_root};

pub fn bench_square(c: &mut Criterion) {
    let mut group = c.benchmark_group("square");

    group.bench_function("square (5,13)", |b| b.iter(|| square(5, 13)));

    group.finish();
}

pub fn bench_square_root(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_root");

    group.bench_function("square_root (12,13)", |b| b.iter(|| square_root(12, 13)));

    group.finish();
}

pub fn bench_find_largest_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime_generator");

    for bit_length in (32..65).step_by(8) {
        // bench it from 32 bit to 64 bit, stepping by 8
        group.bench_with_input(
            BenchmarkId::from_parameter(bit_length),
            &bit_length,
            |b, &bit_length| {
                b.iter(|| find_largest_prime(bit_length));
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_square,
    bench_square_root,
    bench_find_largest_prime
);
criterion_main!(benches);
