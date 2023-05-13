use rand::seq::IteratorRandom;
use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    cell_names: PathBuf,

    #[structopt(short, long)]
    dictionary: String,

    #[structopt(short, long)]
    length: usize,
}

fn get_cell_names(filename: &Path) -> HashSet<String> {

    let mut basenames = HashSet::new();
    let file = File::open(filename).expect("could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("could not read line from file");
        let basename = match line.split('\t').next() {
            Some(basename) => basename,
            None => continue,
        };
        basenames.insert(basename.to_string());
    }
    basenames
}

fn generate_random_strings_for_cell_names(
    cell_names: &HashSet<String>, dictionary: &str, length: usize,) -> HashSet<String> {
    let mut generated_strings = HashSet::new();
    let mut retries = 0;

    while generated_strings.len() < cell_names.len() {
        let random_string = (0..length)
            .map(|_| dictionary.chars().choose(&mut rand::thread_rng()).unwrap())
            .collect::<String>();
        if !generated_strings.contains(&random_string) {
            generated_strings.insert(random_string);
        } else {
            retries += 1;
        }
        if retries > 100 {
            panic!("Unable to generate unique barcode after 100 retries");
        }
    }
    generated_strings
}

fn main() {
    let args = Cli::from_args();
    let cellnames = get_cell_names(&args.cell_names);
    let generated_strings = generate_random_strings_for_cell_names(&cellnames, &args.dictionary, args.length);

    for (cellname, generated_string) in cellnames.iter().zip(generated_strings.iter()) {
        println!("{} \t {}", cellname, generated_string);
    }
}