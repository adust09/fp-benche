use clap::Parser;
use fp_benchmarks::algorithms::fibonacci;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    n: u64,
    #[arg(long, default_value = "iter")]
    mode: String,
}

fn main() {
    let args = Args::parse();
    let result = match args.mode.as_str() {
        "naive" => fibonacci::fib_naive(args.n),
        "iter" => fibonacci::fib_iter(args.n),
        other => {
            eprintln!("unknown mode: {other}, use 'naive' or 'iter'");
            std::process::exit(1);
        }
    };
    println!("{result}");
}
