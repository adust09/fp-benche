use clap::Parser;
use fp_benchmarks::concurrency::parallel_map;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    n: usize,
    #[arg(long, default_value = "0")]
    threads: usize,
}

fn main() {
    let args = Args::parse();

    if args.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()
            .ok();
    }

    let data: Vec<f64> = (1..=args.n).map(|i| i as f64).collect();
    let sum = parallel_map::parallel_map_sum(&data);
    println!("{sum:.6}");
}
