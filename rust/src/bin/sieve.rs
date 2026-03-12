use clap::Parser;
use fp_benchmarks::algorithms::sieve;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    n: usize,
}

fn main() {
    let args = Args::parse();
    let count = sieve::sieve_count(args.n);
    println!("{count}");
}
