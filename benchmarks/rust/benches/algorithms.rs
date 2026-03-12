use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fp_benchmarks::algorithms::{fibonacci, merge_sort, sieve};

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci-naive");
    for n in [30, 35, 40] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| fibonacci::fib_naive(n));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("fibonacci-iter");
    for n in [10_000u64, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| fibonacci::fib_iter(n));
        });
    }
    group.finish();
}

fn bench_merge_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge-sort");
    for size in [1_000, 10_000, 100_000] {
        let data: Vec<i64> = (0..size as i64).rev().collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| merge_sort::merge_sort(data));
        });
    }
    group.finish();
}

fn bench_sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("sieve");
    for n in [100_000, 1_000_000, 10_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| sieve::sieve_count(n));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci, bench_merge_sort, bench_sieve);
criterion_main!(benches);
