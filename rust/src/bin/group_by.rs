use clap::Parser;
use fp_benchmarks::data_processing::group_by;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let totals = group_by::group_by_category(&args.input);
    println!("{} categories", totals.len());
}
