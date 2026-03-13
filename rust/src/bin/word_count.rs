use clap::Parser;
use fp_benchmarks::data_processing::word_count;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let counts = word_count::count_words(&args.input);
    // Output total unique words and total count
    let total_words: u64 = counts.values().sum();
    println!("unique={} total={total_words}", counts.len());
}
