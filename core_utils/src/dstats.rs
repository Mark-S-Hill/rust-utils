use std::env;
use fs_extra::dir::get_size;
use pretty_bytes::converter::convert;
use jwalk::{WalkDir, Parallelism};
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let threads: usize = args[1].parse::<usize>().unwrap();
    let path = args[2].to_string();                         
    
    let dir_size = get_size(&path).unwrap() as f64;
    
    let sub_dirs = WalkDir::new(&path)
                     .parallelism(Parallelism::RayonNewPool(threads))
                     .into_iter()
                     .par_bridge()
                     .filter_map(Result::ok)
                     .filter(|entry| entry.path().is_dir())
                     .count();

    let files = WalkDir::new(&path)
                    .parallelism(Parallelism::RayonNewPool(threads))
                    .into_iter()
                    .par_bridge()
                    .filter_map(Result::ok)
                    .filter(|entry| entry.path().is_file())
                    .count();    
                                                              
    println!("Parent-directory: {}\tSub-directories: {}\tTotal-files: {}", 
             convert(dir_size), 
             sub_dirs,
             files);
}

