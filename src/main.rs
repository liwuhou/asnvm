use clap::Parser;
use cli::Cli;
use std::fs;
use std::path::PathBuf;

mod cli;

fn main() {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(p) => p,
        _ => ".".to_string(),
    };

    println!("{}", path);

    let (nvm_config_path, package_config_path) = find_config(&path);
    println!("nvm config path is :{:?}", nvm_config_path);
    println!("package config path is :{:?}", package_config_path);
}

fn find_config(path: &String) -> (String, String) {
    let mut nvm_config_path = String::new();
    let mut package_config_path = String::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if file_name == ".nvmrc" {
                    nvm_config_path = get_file_path(entry.path());
                } else if file_name == "package.json" {
                    package_config_path = get_file_path(entry.path());
                }
            }
        }
    }

    (nvm_config_path, package_config_path)
}

fn get_file_path(path: PathBuf) -> String {
    match path.to_str() {
        Some(path_str) => path_str.to_string(),
        _ => String::new(),
    }
}
