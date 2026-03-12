use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fp_benchmarks::algorithms::{merge_sort, merge_sort_fp, sieve, sieve_fp};
use fp_benchmarks::concurrency::{channel_throughput, channel_throughput_fp};
use fp_benchmarks::data_processing::{group_by, group_by_fp, word_count_fp};

/// Merge step comparison: index-based (while + i,j) vs slice-decomposition based iteration.
/// The recursive sort structure is identical — only the merge helper differs.
fn bench_merge_step_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge-step-styles");
    for size in [1_000, 10_000, 100_000] {
        let data: Vec<i64> = (0..size as i64).rev().collect();
        group.bench_with_input(
            BenchmarkId::new("index-based", size),
            &data,
            |b, data| {
                b.iter(|| merge_sort::merge_sort(black_box(data)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("iterator-based", size),
            &data,
            |b, data| {
                b.iter(|| merge_sort_fp::merge_sort_fp(black_box(data)));
            },
        );
    }
    group.finish();
}

/// Prime generation comparison: imperative sieve vs declarative trial division.
/// This now contrasts algorithmic style as well as control-flow style.
fn bench_sieve_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("sieve-styles");
    group.sample_size(20);
    for n in [100_000, 1_000_000, 10_000_000] {
        group.bench_with_input(BenchmarkId::new("while-loop", n), &n, |b, &n| {
            b.iter(|| sieve::sieve_count(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("fold-for_each", n), &n, |b, &n| {
            b.iter(|| sieve_fp::sieve_count_fp(black_box(n)));
        });
    }
    group.finish();
}

/// Word count comparison: nested for-loops vs flat_map + fold chain.
/// Both are in-memory (no I/O), matching the logic of word_count.rs.
fn bench_word_count_styles(c: &mut Criterion) {
    let lines: Vec<String> = (0..10_000)
        .map(|i| format!("the quick brown fox jumps over lazy dog number {i}"))
        .collect();

    let mut group = c.benchmark_group("word-count-styles");
    for size in [1_000, 5_000, 10_000] {
        let input = &lines[..size];
        group.bench_with_input(
            BenchmarkId::new("nested-loops", size),
            &input,
            |b, input| {
                b.iter(|| word_count_fp::count_words_mem_imperative(black_box(input)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("flat_map-fold", size),
            &input,
            |b, input| {
                b.iter(|| word_count_fp::count_words_mem_fp(black_box(input)));
            },
        );
    }
    group.finish();
}

/// Group-by comparison: for-loop accumulator vs fold accumulator.
fn bench_group_by_styles(c: &mut Criterion) {
    let categories = ["food", "drink", "electronics", "clothing", "books"];
    let mut group = c.benchmark_group("group-by-styles");
    for count in [10_000, 100_000, 1_000_000] {
        let records: Vec<(String, f64)> = (0..count)
            .map(|i| {
                (
                    categories[i % categories.len()].to_string(),
                    (i as f64) * 1.5,
                )
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("for-loop", count),
            &records,
            |b, records| {
                b.iter(|| group_by::group_by_category_mem(black_box(records)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("fold", count),
            &records,
            |b, records| {
                b.iter(|| group_by_fp::group_by_category_mem_fp(black_box(records)));
            },
        );
    }
    group.finish();
}

/// Channel comparison (null hypothesis): for-loop vs for_each/fold.
/// Channel synchronization dominates — style choice should have zero impact.
fn bench_channel_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel-styles");
    for n in [10_000u64, 100_000] {
        group.bench_with_input(BenchmarkId::new("for-loop", n), &n, |b, &n| {
            b.iter(|| channel_throughput::channel_roundtrip(black_box(n)));
        });
        group.bench_with_input(
            BenchmarkId::new("for_each-fold", n),
            &n,
            |b, &n| {
                b.iter(|| channel_throughput_fp::channel_roundtrip_fp(black_box(n)));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_merge_step_styles,
    bench_sieve_styles,
    bench_word_count_styles,
    bench_group_by_styles,
    bench_channel_styles
);
criterion_main!(benches);
