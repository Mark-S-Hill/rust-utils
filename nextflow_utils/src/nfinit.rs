use std::fs;
use structopt::StructOpt;
use git2::Repository;

#[derive(StructOpt)]
struct Cli {
    project_name: String,
}

fn setup_project(project_name: &String) -> std::io::Result<()> {
    
    fs::create_dir(project_name)?;        
    let subdirs = vec!["/workflows", "/subworkflows", "/modules", "/bin", "/scripts", 
                       "/ENV", "/ancillary", "/conf", "/lib"];    
    for subdir in subdirs.iter(){
        fs::create_dir(format!("{}{}", project_name, subdir))?;
    }

    let files = vec!["/main.nf", "/nextflow.config", "/conf/base.conf", "/.gitignore"]; 
    for file in files.iter(){
        fs::File::create(format!("{}{}", project_name, file))?;
    }

    Ok(())
}

fn main() {
    let args = Cli::from_args();
    let _ = setup_project(&args.project_name);     

    let _repo = match Repository::init(format!("{}{}", "./", &args.project_name)) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

}