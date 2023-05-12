use std::collections::{HashSet, HashMap};
use std::fs;
use rand::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    directory: String,
    prefix_pattern: String,
    barcode_length: usize,
    barcode_alphabet: String
}

fn get_matching_basenames(directory: &String, pattern: &String) -> HashSet<String> {
    let re = regex::Regex::new(pattern).unwrap();
    let mut result = HashSet::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    if re.is_match(filename) {
                        if let Some(captures) = re.captures(filename) {
                            if let Some(match_str) = captures.get(0) {
                                result.insert(match_str.as_str().to_owned());
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn generate_random_string(alphabet: &String, length: usize) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| alphabet.chars().choose(&mut rng).unwrap())
        .collect()
}

fn generate_random_strings_for_basenames(basenames: &HashSet<String>, alphabet: &String, length: usize) -> Vec<(String, String)> {
    let mut random_strings = HashMap::new();
    let mut result = Vec::new();
    let mut retry_count = 0;
    while random_strings.len() < basenames.len() {
        for basename in basenames {
            if !random_strings.contains_key(basename) {
                let random_string = generate_random_string(alphabet, length);
                if !random_strings.values().any(|v| v == &random_string) {
                    random_strings.insert(basename.clone(), random_string.clone());
                } else {
                    retry_count += 1;
                    if retry_count > 10 {
                        panic!("Failed to generate unique random string for basename: {}", basename);
                    }
                }
            }
        }
    }
    for basename in basenames {
        result.push((basename.clone(), random_strings[basename].clone()));
    }
    result
}

fn main() {

    let args = Cli::from_args();
    let directory = args.directory;
    let pattern = args.prefix_pattern;
    let length = args.barcode_length;
    let alphabet = args.barcode_alphabet;
    
    let basenames = get_matching_basenames(&directory, &pattern);
    let random_strings = generate_random_strings_for_basenames(&basenames, &alphabet, length);
    
    for (basename, random_string) in random_strings {
        println!("{} \t {}", basename, random_string);
    }
}