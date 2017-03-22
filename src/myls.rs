extern crate users;
use self::users::get_user_by_uid;

use std::fs;
use std::fs::Metadata;
use std::io::Error;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;

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
/// TODO
/// Use try! for reduce «if let»
/// And use map
/// let mut read = fs::read_dir(dir_name);
/// try!(read);
/// let file_names = read.iter().filter_map(|s| s).filter_map(|f| f.path().as_path().to_str())
/// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
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

fn get_charactere_type(metadata: &Metadata) -> char {
    match metadata.file_type() {
        t if t.is_dir()             => 'd',
        t if t.is_symlink()         => 'l',
        t if t.is_block_device()    => 'b',
        t if t.is_char_device()     => 'c',
        t if t.is_fifo()            => 'p',
        t if t.is_socket()          => 's',
        _                           => '-',
    }
}

// TODO Determine the permision of file
// TODO owner name and group name
// TODO print name without begin
fn show_long_file(name_file: &str) {
    match fs::metadata(name_file) {
        Ok(metadata) => {
            let user: String = match get_user_by_uid(metadata.uid()) {
                Some(user) => user.name().to_string(),
                None => "unknow".to_string(),
            };
            println!("{} {} {} {}", get_charactere_type(&metadata), metadata.nlink(), user, name_file);
        },
        Err(err) => {
            print_error(name_file, err);
        },
    }
}
