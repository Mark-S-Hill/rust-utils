use jwalk::{WalkDir, Parallelism};
use std::env;
use chrono::offset::Utc;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let threads: usize = args[1].parse::<usize>().unwrap();
    let path = args[2].to_string();
     
    let file_count = WalkDir::new(&path)
                     .parallelism(Parallelism::RayonNewPool(threads))
                     .into_iter()
                     .par_bridge()
                     .filter_map(Result::ok)
                     .filter(|entry| entry.path().is_file())
                     .count();
    
    println!("{}\t{:?}\t{}", path, Utc::now(), file_count);
}
