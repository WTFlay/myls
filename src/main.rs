use std::env;
use std::fs;
use std::os::freebsd::fs::MetadataExt;

#[macro_use] extern crate fomat_macros;

fn get_files(dir_name: &str) -> Vec<String> {

    let mut file_names = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_name) {
        for entry in entries {
            if let Ok(file) = entry {
                if let Some(name) = file.path().as_path().to_str() {
                    file_names.push(name.to_string());
                }
            }
        }
    }
    file_names
}

fn show_files(list_name: Vec<String>) {
    for file_name in &list_name {
        match fs::metadata(file_name) {
            Ok(file_metadata)    => {
                if file_metadata.is_dir() {
                    println!("{} d {}", file_metadata.st_ino(), file_name);
                } else {
                    println!("{} - {}", file_metadata.st_ino(), file_name);
                }
            },
            Err(err)    => {
                println!("myls: {}: {}", file_name, err);
            },
        }
    }
}

fn main() {
    let input_file_names: Vec<String> = match env::args().count() {
        1   => { vec![".".to_string()] },
        _   => { env::args().skip(1).collect() },
    };

    for file_name in &input_file_names {
        match fs::metadata(file_name) {
            Ok(file_metadata)    => {
                if file_metadata.is_dir() {
                    show_files(get_files(file_name));
                } else {
                    println!("{}", file_name);
                }
            },
            Err(err)    => {
                println!("myls: {}: {}", file_name, err);
            },
        }
    }
}
