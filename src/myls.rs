use std::fs;

pub struct Options {
    pub list: bool,
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
            println!("myls: {}: {}", dir_name, err);
            None
        },
    }
}

/// Show all files containes in vector with appropriate format
/// by options given in parameters
pub fn show_files(name_files: Vec<&str>, options: &Options) -> bool {
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
