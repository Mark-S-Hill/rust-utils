use structopt::StructOpt;
use fs_extra::dir::get_size;
use pretty_bytes::converter::convert;
use jwalk::{WalkDir, Parallelism};
use rayon::prelude::*;
use rayon::iter::ParallelBridge;

#[derive(StructOpt)]
struct Cli {
    threads: usize,
    path: String,
}

fn count_objects(obj: &str, path: &str, threads: usize) -> usize {
    
    let file_dirs = WalkDir::new(path)
        .parallelism(Parallelism::RayonNewPool(threads))
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok);

    if obj == "file" {    
        
        file_dirs.filter(|entry| entry.path().is_file()).count()    
    
    } else if obj == "directory" {
        
        file_dirs.filter(|entry| entry.path().is_dir()).count()      
    
     } else {
         
        0
        
       }    
}

fn main() {
    
    let args = Cli::from_args();    
    let dir_size = get_size(&args.path).unwrap() as f64;    
    let files = count_objects("file", &args.path, args.threads);
    let sub_dirs = count_objects("directory", &args.path, args.threads); 
                                                              
    println!("Parent-directory: {}\tSub-directories: {}\tTotal-files: {}", 
             convert(dir_size),
             sub_dirs, 
             files);
}