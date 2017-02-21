use std::fs;
use std::io::Error;
use std::os::unix::fs::FileTypeExt;

pub struct Options {
    pub list: bool,
    pub all: bool,
}

fn print_error(file_name: &str, err: Error) {
    println!("myls: {}: {}", file_name, err);
}

/// Return vector with all files name contain in dir_name
/// given in parameters.
///
/// # Arguments
/// * `dir_name` - A string slice that hold the name of directory
pub fn get_files_in_dir(dir_name: &str) -> Option<Vec<String>> {
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
            print_error(dir_name, err);
            None
        },
    }
}

/// Show all files containes in vector with appropriate format
/// by options given in parameters
pub fn show_files(name_files: Vec<&str>, options: &Options) -> bool {
    for (index, name) in name_files.iter().enumerate() {

        // If option all is false and the filename begin by '.', continue for.

        if options.list {
            show_long_file(name);
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

fn get_charactere_type(name_file: &str) -> char {
    match fs::metadata(name_file) {
        Ok(metadata) => {
            let filetype = metadata.file_type();
            if filetype.is_dir() {
                'd'
            } else if filetype.is_symlink() {
                'l'
            } else if filetype.is_block_device() {
                'b'
            } else if filetype.is_char_device() {
                'c'
            } else if filetype.is_fifo() {
                'p'
            } else if filetype.is_socket() {
                's'
            } else {
                '-'
            }
        },
        Err(err) => {
            print_error(name_file, err);
            '?'
        },
    }
}

fn show_long_file(name_file: &str) {
    // TODO Determine the permision of file

    // TODO number of link
    
    // TODO owner name and group name
    
    // TODO print name without begin
    print!("{} {}\n", get_charactere_type(name_file), name_file);
}
