use clap::Parser;
use fp_benchmarks::algorithms::merge_sort;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    n: usize,
}

fn main() {
    let args = Args::parse();
    // Generate deterministic data (reversed sequence for worst case)
    let data: Vec<i64> = (0..args.n as i64).rev().collect();
    let sorted = merge_sort::merge_sort(&data);
    println!("{}", sorted.len());
}
