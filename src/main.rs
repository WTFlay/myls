use std::env;
use std::fs;

struct Options {
    list: bool,
}

fn get_files_in_dir<'a>(file_name: &'a str) -> Result<Vec<String>, &'a str> {
    match fs::metadata(file_name) {
        Ok(file_metadata) => {
            if file_metadata.is_dir() {
                let mut file_names = Vec::new();
                if let Ok(entries) = fs::read_dir(file_name) {
                    for entry in entries {
                        if let Ok(file) = entry {
                            if let Some(name) = file.path().as_path().to_str() {
                                file_names.push(name.to_string());
                            }
                        }
                    }
                }
                Ok(file_names)
            } else {
                Err(file_name)
            }
        },
        Err(err) => {
            println!("myls: {}: {}", file_name, err);
            Err(file_name)
        },
    }
}

fn show_files(name_files: Vec<String>, options: &Options) {
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
}

fn main() {
    let input_file_names: Vec<String> = match env::args().count() {
        1   => { vec![".".to_string()] },
        _   => { env::args().skip(1).map(|x| x.to_string()).collect() },
    };
    let options = Options { list: false };

    let mut simple_files: Vec<String> = Vec::new();
    for (index, file_name) in input_file_names.iter().enumerate() {
        match get_files_in_dir(file_name) {
            Ok(files) => {
                if input_file_names.len() > 1 {
                    print!("{}:\n", file_name);
                }
                show_files(files, &options);
                if index < (input_file_names.len() - 1) {
                    print!("\n");
                }
            },
            Err(file_name) => {
                simple_files.push(file_name.to_string());
            },
        }
    }
    show_files(simple_files, &options);
}
