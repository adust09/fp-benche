use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fp_benchmarks::data_processing::{group_by, json_filter};

fn bench_json_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("json-filter");
    for count in [1_000, 10_000, 100_000] {
        // Generate in-memory JSON data
        let users: Vec<serde_json::Value> = (0..count)
            .map(|i| {
                serde_json::json!({
                    "id": i,
                    "name": format!("user_{i}"),
                    "age": 20 + (i % 40) as u32,
                    "city": ["Tokyo", "Osaka", "Kyoto"][i % 3],
                    "active": i % 3 != 0
                })
            })
            .collect();
        let json_str = serde_json::to_string(&users).unwrap();

        group.bench_with_input(BenchmarkId::from_parameter(count), &json_str, |b, data| {
            b.iter(|| json_filter::filter_users_from_str(data));
        });
    }
    group.finish();
}

fn bench_group_by(c: &mut Criterion) {
    let categories = ["food", "drink", "electronics", "clothing", "books"];
    let mut group = c.benchmark_group("group-by");
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
            BenchmarkId::from_parameter(count),
            &records,
            |b, records| {
                b.iter(|| group_by::group_by_category_mem(records));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_json_filter, bench_group_by);
criterion_main!(benches);
