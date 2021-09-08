use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    error: String,
}

fn main() {
    
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");    
    let re = Regex::new(r"[a-z0-9]{2}/[a-z0-9]{6}").unwrap();    
    let cached = "Cached process";
    let submitted = "Submitted process";
    let complete = "Task completed";
    //let no_error = "exit: 0;";
    let error = Regex::new(r"exit: [1-9][0-9]*").unwrap();
            
    for line in content.lines() {
        if line.contains(&args.pattern) && 
            re.is_match(line) {
               
                if line.contains(cached){
                   
                    println!("WorkDir: {}\t{} {}: {} {}", 
                            line.split(" ").nth(9).unwrap(),
                            line.split(" ").nth(10).unwrap(),
                            line.split(" ").nth(11).unwrap(),
                            line.split(" ").nth(13).unwrap(),
                            line.split(" ").nth(14).unwrap())
                            
                } else if line.contains(submitted){
                   
                    println!("WorkDir: {}\t{} {}: {} {}", 
                            line.split(" ").nth(8).unwrap(),
                            line.split(" ").nth(9).unwrap(),
                            line.split(" ").nth(10).unwrap(),
                            line.split(" ").nth(12).unwrap(),
                            line.split(" ").nth(13).unwrap())
                            
                } else if line.contains(complete) {
                   
                   if &args.error == "TRUE" &&
                        error.is_match(line) {
            
                       println!("Process: {}{} {} {} WorkDir:{}", 
                                line.split(" ").nth(15).unwrap(),
                                line.split(" ").nth(16).unwrap(),
                                line.split(" ").nth(19).unwrap(),
                                line.split(" ").nth(20).unwrap(),
                                line.split(" ").nth(24).unwrap()   
                                );
                   }
                   /*
                    println!("Workdir: {}\t{} {}: {} {}", 
                            line.split(" ").nth(24).unwrap(),
                            line.split(" ").nth(7).unwrap(),
                            line.split(" ").nth(8).unwrap(),
                            line.split(" ").nth(15).unwrap(),
                            line.split(" ").nth(16).unwrap()) 
                    */ 
                }    
        }
    }
}
