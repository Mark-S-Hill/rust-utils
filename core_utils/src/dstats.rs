use std::env;
use fs_extra::dir::get_size;
use pretty_bytes::converter::convert;
use jwalk::{WalkDir, Parallelism};
use rayon::prelude::*;
use rayon::iter::ParallelBridge;

fn count_objects(obj: &str, path: &str, threads: usize) -> usize {
    
    let file_dirs = WalkDir::new(path)
                    .parallelism(Parallelism::RayonNewPool(threads))
                    .into_iter()
                    .par_bridge()
                    .filter_map(Result::ok);

    if obj == "file" {    
        
        file_dirs
        .filter(|entry| entry.path().is_file())
        .count()    
    
    } else if obj == "directory" {
        
        file_dirs
        .filter(|entry| entry.path().is_dir())
        .count()      
    
    } else {
        0
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let threads: usize = args[1].parse::<usize>().unwrap();
    let path = args[2].to_string();                         
    
    let dir_size = get_size(&path).unwrap() as f64;    
    let files = count_objects("file", &path, threads);
    let sub_dirs = count_objects("directory", &path, threads); 
                                                              
    println!("Parent-directory: {}\tSub-directories: {}\tTotal-files: {}", 
             convert(dir_size),
             sub_dirs, 
             files);
}