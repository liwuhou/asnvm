use clap::Parser;
use cli::Cli;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod cli;

fn main() {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(p) => p,
        _ => ".".to_string(),
    };

    let (nvm_config_path, package_config_path) = find_config(&path);
    if let Some(path) = nvm_config_path {
        println!("nvm config path is :{:?}", path);
        excute_nvm_config(&path)
    } else if let Some(path) = package_config_path {
        println!("package config path is :{:?}", path);
        excute_package_config(&path);
    } else {
        println!("Found nothing");
    }
}

fn find_config(path: &String) -> (Option<String>, Option<String>) {
    let mut nvm_config_path: Option<String> = None;
    let mut package_config_path: Option<String> = None;

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

fn get_file_path(path: PathBuf) -> Option<String> {
    match path.to_str() {
        Some(path_str) => Some(path_str.to_string()),
        _ => None,
    }
}

fn excute_nvm_config(path: &String) {
    if let Ok(content) = fs::read(path) {
        if let Some(version) = String::from_utf8_lossy(&content).lines().next() {
            println!("{}", version);
            // 调用外部命令，nvm 切换
        }
    }
}

fn excute_package_config(_path: &String) {
    todo!()
}

fn excute_nvm_switch(version: &str) -> bool {
    todo!()
}
