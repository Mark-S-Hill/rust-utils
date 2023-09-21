use std::fs;
use std::io::Write;
use structopt::StructOpt;
use git2::Repository;

#[derive(StructOpt)]
struct Cli {
    project_name: String,
    author: String
}

fn trim_margin(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_start())
        .collect::<Vec<&str>>()
        .join("\n")
}

fn setup_project(project_name: &String, author: &String) -> std::io::Result<()> {

    fs::create_dir(project_name)?;
    let subdirs = vec!["/data", "/src", "/models", "/notebooks", "/docs", "/reports", "/ENV", "/tests", "/config"];
    for subdir in subdirs.iter(){
        fs::create_dir(format!("{}{}", project_name, subdir))?;
    }

    let mut readme = fs::File::create(format!("{}{}", project_name, "/README.md"))?;
    readme.write_all((format!("{}{}; {}{}", "# Project: ", project_name, "Author: ", author)).as_bytes())?;

    let mut gitignore = fs::File::create(format!("{}{}", project_name, "/.gitignore"))?;
    let gitignore_content = format!(
        "
        __pycache__/
        .env
        .vscode/
        /data/
        .DS_Store
        "
    );
    gitignore.write_all(trim_margin(&gitignore_content).as_bytes())?;

    let files = vec!["/requirements.txt", "/setup.py", "/LICENSE"]; 
    for file in files.iter(){
        fs::File::create(format!("{}{}", project_name, file))?;
    }

    let subsubdirs = vec!["/raw", "/processed", "/final"];
    for subsubdir in subsubdirs.iter(){
        fs::create_dir(format!("{}{}{}", project_name, "/data", subsubdir))?;
    }

    Ok(())
}

fn main() {

    let args = Cli::from_args();
    let _ = setup_project(&args.project_name, &args.author);

    let _repo = match Repository::init(format!("{}{}", "./", &args.project_name)) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

}