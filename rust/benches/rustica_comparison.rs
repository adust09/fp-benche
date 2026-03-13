use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fp_benchmarks::algorithms::{
    fibonacci, fibonacci_rustica, merge_sort, merge_sort_fp, merge_sort_rustica, sieve, sieve_fp,
    sieve_rustica,
};
use fp_benchmarks::concurrency::{
    channel_throughput, channel_throughput_fp, channel_throughput_rustica, parallel_map,
    parallel_map_rustica,
};
use fp_benchmarks::data_processing::{
    group_by, group_by_fp, group_by_rustica, json_filter, json_filter_rustica, word_count_fp,
    word_count_rustica,
};

/// Fibonacci: iterator fold vs PVec fold.
/// No _fp variant exists (fib_iter is already FP-style), so this is a 2-way comparison.
fn bench_fibonacci_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci-styles");
    for n in [10_000u64, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::new("iterator-fold", n), &n, |b, &n| {
            b.iter(|| fibonacci::fib_iter(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("pvec-fold", n), &n, |b, &n| {
            b.iter(|| fibonacci_rustica::fib_rustica(black_box(n)));
        });
    }
    group.finish();
}

/// Merge sort: index-based vs iterator-based vs PVec-based (3-way).
fn bench_merge_sort_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge-sort-styles");
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
        group.bench_with_input(BenchmarkId::new("pvec-based", size), &data, |b, data| {
            b.iter(|| merge_sort_rustica::merge_sort_rustica(black_box(data)));
        });
    }
    group.finish();
}

/// Trial division styles: imperative sieve vs iterator filter vs PVec filter (3-way).
/// Note: imperative uses Eratosthenes sieve (different algorithm), while FP/rustica use trial division.
fn bench_sieve_trial_division_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("sieve-trial-division-styles");
    group.sample_size(20);
    for n in [100_000, 1_000_000, 10_000_000] {
        group.bench_with_input(BenchmarkId::new("imperative-sieve", n), &n, |b, &n| {
            b.iter(|| sieve::sieve_count(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("iterator-filter", n), &n, |b, &n| {
            b.iter(|| sieve_fp::sieve_count_fp(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("pvec-filter", n), &n, |b, &n| {
            b.iter(|| sieve_rustica::sieve_count_rustica(black_box(n)));
        });
    }
    group.finish();
}

/// Word count: nested loops vs flat_map+fold vs PVec pipeline (3-way).
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
        group.bench_with_input(
            BenchmarkId::new("pvec-pipeline", size),
            &input,
            |b, input| {
                b.iter(|| word_count_rustica::count_words_mem_rustica(black_box(input)));
            },
        );
    }
    group.finish();
}

/// JSON filter: iterator filter vs Either+PVec filter (2-way).
fn bench_json_filter_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("json-filter-styles");
    for count in [1_000, 10_000, 100_000] {
        let json = generate_users_json(count);
        group.bench_with_input(
            BenchmarkId::new("iterator-filter", count),
            &json,
            |b, json| {
                b.iter(|| json_filter::filter_users_from_str(black_box(json)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("either-pvec-filter", count),
            &json,
            |b, json| {
                b.iter(|| json_filter_rustica::filter_users_from_str_rustica(black_box(json)));
            },
        );
    }
    group.finish();
}

/// Group-by: for-loop vs fold vs PVec fold (3-way).
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
        group.bench_with_input(BenchmarkId::new("fold", count), &records, |b, records| {
            b.iter(|| group_by_fp::group_by_category_mem_fp(black_box(records)));
        });
        group.bench_with_input(
            BenchmarkId::new("pvec-fold", count),
            &records,
            |b, records| {
                b.iter(|| group_by_rustica::group_by_category_mem_rustica(black_box(records)));
            },
        );
    }
    group.finish();
}

/// Channel throughput: for-loop vs for_each/fold vs Maybe-wrapped (3-way control bench).
fn bench_channel_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel-styles");
    for n in [10_000u64, 100_000] {
        group.bench_with_input(BenchmarkId::new("for-loop", n), &n, |b, &n| {
            b.iter(|| channel_throughput::channel_roundtrip(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("for_each-fold", n), &n, |b, &n| {
            b.iter(|| channel_throughput_fp::channel_roundtrip_fp(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("maybe-wrapped", n), &n, |b, &n| {
            b.iter(|| channel_throughput_rustica::channel_roundtrip_rustica(black_box(n)));
        });
    }
    group.finish();
}

/// Sequential map+sum: iterator vs PVec map+fold (2-way).
fn bench_sequential_map_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential-map-styles");
    for size in [100_000, 1_000_000] {
        let data: Vec<f64> = (1..=size).map(|i| i as f64).collect();
        group.bench_with_input(
            BenchmarkId::new("iterator", size),
            &data,
            |b, data| {
                b.iter(|| parallel_map::sequential_map_sum(black_box(data)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("pvec-map-fold", size),
            &data,
            |b, data| {
                b.iter(|| parallel_map_rustica::sequential_map_sum_rustica(black_box(data)));
            },
        );
    }
    group.finish();
}

fn generate_users_json(count: usize) -> String {
    let cities = ["Tokyo", "Osaka", "Kyoto", "Nagoya", "Sapporo"];
    let mut json = String::from("[");
    for i in 0..count {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!(
            r#"{{"id":{},"name":"User{}","age":{},"city":"{}","active":{}}}"#,
            i,
            i,
            20 + (i % 40),
            cities[i % cities.len()],
            i % 2 == 0
        ));
    }
    json.push(']');
    json
}

criterion_group!(
    benches,
    bench_fibonacci_styles,
    bench_merge_sort_styles,
    bench_sieve_trial_division_styles,
    bench_word_count_styles,
    bench_json_filter_styles,
    bench_group_by_styles,
    bench_channel_styles,
    bench_sequential_map_styles
);
criterion_main!(benches);
