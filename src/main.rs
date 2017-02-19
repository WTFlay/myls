mod myls;

use std::env;
use std::fs;

fn main() {
    let input_file_names: Vec<String> = match env::args().count() {
        1   => { vec![".".to_string()] },
        _   => { env::args().skip(1).map(|x| x.to_string()).collect() },
    };

    let options = myls::Options { list: true };

    let mut simple_files: Vec<&str> = Vec::new();
    let mut dir_files: Vec<&str> = Vec::new();

    // For each file given in parametre, sort directory and
    // regular file in different vector
    for file_name in &input_file_names {
        match fs::metadata(file_name) {
            Ok(file_metadata) => {
                if file_metadata.is_dir() {
                    dir_files.push(file_name);
                } else {
                    simple_files.push(file_name);
                }
            },
            Err(err) => {
                println!("myls: {}: {}", file_name, err);
            },
        }
    }

    // Show all simple files
    if myls::show_files(simple_files, &options) && dir_files.len() > 0 {
        print!("\n");
    }

    // For each directory file, show files are in
    for (index, dir_name) in dir_files.iter().enumerate() {
        if let Some(files_name) = myls::get_files_in_dir(dir_name) {
            if index > 0 {
                print!("\n");
            }
            if input_file_names.len() > 1 {
                print!("{}:\n", dir_name);
            }
            myls::show_files(files_name.iter().map(|s| s.as_ref()).collect(), &options);
        }
    }
}
