use clap::{
    Args,
    Parser,
    Subcommand
};

use std::fs;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DiskCleaner {
    ///The first argument, this is a flag
    pub path: String,
    ///The second argument this is a directory
    pub pattern: String,
}

//function to search disk for files matching pattern 
fn search_disk_for_files(path: &str, pattern: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if path.is_file() && match_file_pattern(file_name, pattern) {
            files.push(file_name.to_string());
        }
    }
    files
}

//match file pattern
fn match_file_pattern(file_name: &str, pattern: &str) -> bool {
    let mut pattern = pattern.to_string();
    if pattern.starts_with("*") {
        pattern = pattern[1..].to_string();
    }
    if pattern.ends_with("*") {
        pattern = pattern[..pattern.len() - 1].to_string();
    }
    file_name.contains(&pattern)
}



fn main(){
    let args: DiskCleaner = DiskCleaner::parse();
    let files = search_disk_for_files(&args.path, &args.pattern);
    println!("{:?}", files);
}