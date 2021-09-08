use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");    
    let re = Regex::new(r"[a-z0-9]{2}/[a-z0-9]{6}").unwrap();    
            
    for line in content.lines() {
        if line.contains(&args.pattern) && re.is_match(line) {
               
                if line.contains("Cached process") {
                   
                    println!("WorkDir: {}\t{} {}: {} {}", 
                             line.split(" ").nth(9).unwrap(),
                             line.split(" ").nth(10).unwrap(),
                             line.split(" ").nth(11).unwrap(),
                             line.split(" ").nth(13).unwrap(),
                             line.split(" ").nth(14).unwrap())
                            
                } else if line.contains("Submitted process") {
                   
                    println!("WorkDir: {}\t{} {}: {} {}", 
                             line.split(" ").nth(8).unwrap(),
                             line.split(" ").nth(9).unwrap(),
                             line.split(" ").nth(10).unwrap(),
                             line.split(" ").nth(12).unwrap(),
                             line.split(" ").nth(13).unwrap())                    
                 }   
        }
    }
}
