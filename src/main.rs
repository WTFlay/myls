use std::env;
use std::fs;

struct Options {
    list: bool,
}

fn get_files_in_dir(dir_name: &str) -> Option<Vec<String>> {
    match fs::metadata(dir_name) {
        Ok(file_metadata) => {
            if file_metadata.is_dir() {
                let mut file_names = Vec::new();
                if let Ok(entries) = fs::read_dir(dir_name) {
                    for entry in entries {
                        if let Ok(file) = entry {
                            match file.path().as_path().to_str() {
                                Some(name) => {
                                    file_names.push(name.to_string());
                                },
                                None => {},
                            }
                        }
                    }
                }
                Some(file_names)
            } else {
                None
            }
        },
        Err(err) => {
            println!("myls: {}: {}", dir_name, err);
            None
        },
    }
}

fn show_files(name_files: Vec<&str>, options: &Options) -> bool {
    for (index, name) in name_files.iter().enumerate() {
        if options.list {
            println!("{}", name);
        } else {
            if index > 0 {
                print!(" ");
            }
            print!("{}", name);
            if index == (name_files.len() - 1) {
                print!("\n");
            }
        }
    }
    name_files.len() > 0
}

fn main() {
    let input_file_names: Vec<String> = match env::args().count() {
        1   => { vec![".".to_string()] },
        _   => { env::args().skip(1).map(|x| x.to_string()).collect() },
    };

    let options = Options { list: true };

    let mut simple_files: Vec<&str> = Vec::new();
    let mut dir_files: Vec<&str> = Vec::new();

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

    if show_files(simple_files, &options) && dir_files.len() > 0 {
        print!("\n");
    }

    for (index, dir_name) in dir_files.iter().enumerate() {
        if let Some(files_name) = get_files_in_dir(dir_name) {
            if index > 0 {
                print!("\n");
            }
            if input_file_names.len() > 1 {
                print!("{}:\n", dir_name);
            }
            show_files(files_name.iter().map(|s| s.as_ref()).collect(), &options);
        }
    }
}
