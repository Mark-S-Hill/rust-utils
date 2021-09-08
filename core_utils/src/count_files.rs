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
    let threads = args.threads;
    let path = args.path;
     
    let file_count = WalkDir::new(&path)
        .parallelism(Parallelism::RayonNewPool(threads))
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .count();
    
    println!("{}\t{:?}\t{}", path, Utc::now(), file_count);
}
