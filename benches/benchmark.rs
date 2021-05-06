use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hardprime::{find_largest_prime, square, square_root};

// 16-bit prime -> 65521
// 32-bit prime -> 4294967197
// 48-bit prime -> 281474976710597

// let our small number be 51, so that 51^2 will never be more than the primes
// and we will compare p-51 with 51 as inputs for square function to see if there is any difference in performance

pub fn bench_square_fit(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_fit");

    for prime in [65521, 4294967197, 281474976710597].iter() {
        // bench it from 32 bit to 64 bit, stepping by 8
        group.bench_with_input(BenchmarkId::from_parameter(prime), &prime, |b, &prime| {
            b.iter(|| square(51, *prime));
        });
    }

    group.finish();
}

pub fn bench_square_overflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_overflow");

    for prime in [65521, 4294967197, 281474976710597].iter() {
        // bench it from 32 bit to 64 bit, stepping by 8
        group.bench_with_input(BenchmarkId::from_parameter(prime), &prime, |b, &prime| {
            b.iter(|| square(*prime - 51, *prime));
        });
    }

    group.finish();
}

pub fn bench_square_root(c: &mut Criterion) {
    let mut group = c.benchmark_group("square_root");

    for prime in [65521, 4294967197, 281474976710597].iter() {
        // bench it from 32 bit to 64 bit, stepping by 8
        group.bench_with_input(BenchmarkId::from_parameter(prime), &prime, |b, &prime| {
            b.iter(|| square_root(2601, *prime)); // 2601 is the square of 51 and p-51
        });
    }

    group.finish();
}

pub fn bench_find_largest_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime_generator");

    for bit_length in (32..64).step_by(8) {
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
    bench_square_fit,
    bench_square_overflow,
    bench_square_root,
    bench_find_largest_prime
);
criterion_main!(benches);
