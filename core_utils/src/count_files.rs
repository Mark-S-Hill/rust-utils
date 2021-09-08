use structopt::StructOpt;
use jwalk::{WalkDir, Parallelism};
use chrono::offset::Utc;
use rayon::prelude::*;

#[derive(StructOpt)]
struct Cli {
    threads: usize,
    path: String,
}

fn main() {
    let args = Cli::from_args();     
    let file_count = WalkDir::new(&args.path)
        .parallelism(Parallelism::RayonNewPool(args.threads))
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .count();
    
    println!("{}\t{:?}\t{}", args.path, Utc::now(), file_count);
}
