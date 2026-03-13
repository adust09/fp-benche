use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fp_benchmarks::concurrency::{channel_throughput, parallel_map};

fn bench_parallel_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel-map");
    for size in [100_000, 1_000_000] {
        let data: Vec<f64> = (1..=size).map(|i| i as f64).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| parallel_map::parallel_map_sum(data));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("sequential-map");
    for size in [100_000, 1_000_000] {
        let data: Vec<f64> = (1..=size).map(|i| i as f64).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| parallel_map::sequential_map_sum(data));
        });
    }
    group.finish();
}

fn bench_channel(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel-throughput");
    for n in [10_000u64, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| channel_throughput::channel_roundtrip(n));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parallel_map, bench_channel);
criterion_main!(benches);
