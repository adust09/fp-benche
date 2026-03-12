use clap::Parser;
use fp_benchmarks::concurrency::channel_throughput;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    n: u64,
}

fn main() {
    let args = Args::parse();
    let sum = channel_throughput::channel_roundtrip(args.n);
    println!("{sum}");
}
