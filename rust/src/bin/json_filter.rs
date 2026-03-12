use clap::Parser;
use fp_benchmarks::data_processing::json_filter;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let filtered = json_filter::filter_users_from_file(&args.input);
    println!("{}", filtered.len());
}
