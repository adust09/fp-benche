use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use fp_benchmarks::consensus::{
    balance_updates, balance_updates_fp, balance_updates_rustica, epoch_processing,
    epoch_processing_fp, epoch_processing_rustica, state_root, state_root_fp, state_root_rustica,
    types::*,
};
use rustica::pvec::PersistentVector;
use std::time::Duration;

const SEEDS: [u64; 3] = [42, 123, 7777];

// ── Epoch Processing ────────────────────────────────────────────────────────

fn bench_epoch_processing(c: &mut Criterion) {
    for &n in &[10_000usize, 100_000] {
        for &seed in &SEEDS {
            let validators = generate_validators(n, seed);
            let label = format!("n={n}/seed={seed}");

            // ── e2e ──
            {
                let mut group = c.benchmark_group(format!("e2e/epoch-processing"));
                group.sample_size(20);

                group.bench_with_input(
                    BenchmarkId::new("imperative", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || {
                                validators
                                    .iter()
                                    .map(|v| v.effective_balance)
                                    .collect::<Vec<_>>()
                            },
                            |mut balances| {
                                epoch_processing::process_epoch(
                                    black_box(&validators),
                                    black_box(&mut balances),
                                );
                                balances
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("functional", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || {
                                validators
                                    .iter()
                                    .map(|v| v.effective_balance)
                                    .collect::<Vec<_>>()
                            },
                            |balances| {
                                black_box(epoch_processing_fp::process_epoch_fp(
                                    black_box(&validators),
                                    black_box(&balances),
                                ))
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("rustica", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || {
                                let bals: Vec<u64> = validators
                                    .iter()
                                    .map(|v| v.effective_balance)
                                    .collect();
                                bals
                            },
                            |balances| {
                                let pv = PersistentVector::from_slice(&balances);
                                black_box(epoch_processing_rustica::process_epoch_rustica(
                                    black_box(&validators),
                                    pv,
                                ))
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.finish();
            }

            // ── best-case ──
            {
                let mut group = c.benchmark_group(format!("best-case/epoch-processing"));
                group.sample_size(20);

                group.bench_with_input(
                    BenchmarkId::new("imperative", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || {
                                validators
                                    .iter()
                                    .map(|v| v.effective_balance)
                                    .collect::<Vec<_>>()
                            },
                            |mut balances| {
                                epoch_processing::process_epoch(
                                    black_box(&validators),
                                    black_box(&mut balances),
                                );
                                balances
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("functional", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || {
                                validators
                                    .iter()
                                    .map(|v| v.effective_balance)
                                    .collect::<Vec<_>>()
                            },
                            |balances| {
                                black_box(epoch_processing_fp::process_epoch_fp(
                                    black_box(&validators),
                                    black_box(&balances),
                                ))
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("rustica", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || {
                                let bals: Vec<u64> = validators
                                    .iter()
                                    .map(|v| v.effective_balance)
                                    .collect();
                                PersistentVector::from_slice(&bals)
                            },
                            |pv| {
                                black_box(epoch_processing_rustica::process_epoch_rustica(
                                    black_box(&validators),
                                    pv,
                                ))
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.finish();
            }
        }
    }
}

// ── Balance Updates ─────────────────────────────────────────────────────────

fn bench_balance_updates(c: &mut Criterion) {
    let n = 100_000usize;
    let ops_configs: [(usize, usize, usize); 3] = [(50, 30, 20), (500, 300, 200), (5000, 3000, 2000)];

    for &seed in &SEEDS {
        let validators = generate_validators(n, seed);
        let initial: Vec<u64> = validators.iter().map(|v| v.effective_balance).collect();

        for &(n_dep, n_wd, n_slash) in &ops_configs {
            let total_ops = n_dep + n_wd + n_slash;
            let deposits = generate_deposits(n_dep, n, seed + 100);
            let withdrawals = generate_withdrawals(n_wd, &initial, &deposits, seed + 200);
            let slash_indices = generate_slash_indices(n_slash, n, seed + 300);
            let label = format!("n={n}/ops={total_ops}/seed={seed}");

            // ── e2e ──
            {
                let mut group = c.benchmark_group("e2e/balance-updates");
                group.sample_size(20);

                group.bench_with_input(
                    BenchmarkId::new("imperative", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || initial.clone(),
                            |mut balances| {
                                balance_updates::process_balance_updates(
                                    black_box(&mut balances),
                                    black_box(&deposits),
                                    black_box(&withdrawals),
                                    black_box(&slash_indices),
                                );
                                balances
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("functional", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || initial.clone(),
                            |balances| {
                                black_box(balance_updates_fp::process_balance_updates_fp(
                                    black_box(&balances),
                                    black_box(&deposits),
                                    black_box(&withdrawals),
                                    black_box(&slash_indices),
                                ))
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("rustica", &label),
                    &n,
                    |b, _| {
                        b.iter_batched(
                            || initial.clone(),
                            |balances| {
                                let pv = PersistentVector::from_slice(&balances);
                                black_box(balance_updates_rustica::process_balance_updates_rustica(
                                    pv,
                                    black_box(&deposits),
                                    black_box(&withdrawals),
                                    black_box(&slash_indices),
                                ))
                            },
                            BatchSize::LargeInput,
                        );
                    },
                );

                group.finish();
            }
        }
    }
}

// ── State Root ───────────────────────────────────────────────────────────────

fn bench_state_root(c: &mut Criterion) {
    for &n in &[1024usize, 8192] {
        for &seed in &SEEDS {
            let balances = generate_balances(n, seed);
            let label = format!("n={n}/seed={seed}");

            // SHA-256 version
            {
                let mut group = c.benchmark_group("e2e/state-root-sha256");
                group.sample_size(if n >= 8192 { 10 } else { 20 });
                if n >= 8192 {
                    group.measurement_time(Duration::from_secs(10));
                }

                group.bench_with_input(
                    BenchmarkId::new("imperative", &label),
                    &n,
                    |b, _| {
                        b.iter(|| black_box(state_root::compute_state_root(black_box(&balances))));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("functional", &label),
                    &n,
                    |b, _| {
                        b.iter(|| {
                            black_box(state_root_fp::compute_state_root_fp(black_box(&balances)))
                        });
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("rustica", &label),
                    &n,
                    |b, _| {
                        b.iter(|| {
                            black_box(state_root_rustica::compute_state_root_rustica(black_box(
                                &balances,
                            )))
                        });
                    },
                );

                group.finish();
            }

            // XOR structure-only version
            {
                let mut group = c.benchmark_group("e2e/state-root-xor");
                group.sample_size(if n >= 8192 { 10 } else { 20 });

                group.bench_with_input(
                    BenchmarkId::new("imperative", &label),
                    &n,
                    |b, _| {
                        b.iter(|| {
                            black_box(state_root::compute_state_root_xor(black_box(&balances)))
                        });
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("functional", &label),
                    &n,
                    |b, _| {
                        b.iter(|| {
                            black_box(state_root_fp::compute_state_root_xor_fp(black_box(
                                &balances,
                            )))
                        });
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("rustica", &label),
                    &n,
                    |b, _| {
                        b.iter(|| {
                            black_box(state_root_rustica::compute_state_root_xor_rustica(
                                black_box(&balances),
                            ))
                        });
                    },
                );

                group.finish();
            }
        }
    }
}

criterion_group!(benches, bench_epoch_processing, bench_balance_updates, bench_state_root);
criterion_main!(benches);
