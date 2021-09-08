use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");    
    let error = Regex::new(r"exit: [1-9][0-9]*").unwrap();
            
    for line in content.lines() {
        if line.contains("COMPLETED") && error.is_match(line) {
            
            println!("Process: {}{} {} {} WorkDir:{}", 
                     line.split(" ").nth(15).unwrap(),
                     line.split(" ").nth(16).unwrap(),
                     line.split(" ").nth(19).unwrap(),
                     line.split(" ").nth(20).unwrap(),
                     line.split(" ").nth(24).unwrap())   
        }
    }
}
