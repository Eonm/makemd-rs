use std::fs;
use std::path::Path;

extern crate glob;
use glob::glob;

use std::fs::File;
use std::io::Read;
use std::process;


pub fn get_input_files(path: &str) -> Vec<String> {
    let pattern = format!("{}/**/*.md", path);
    glob(&pattern).expect("failed to get globals").filter_map(Result::ok).map(|p| p.display().to_string()).collect()
}

pub fn get_filename (document_path: &str) -> String {
    let mut file = match File::open(document_path) {
        Err(_e) => {
            error!("failed to open file : {}. Check your .env file entries", document_path);
            process::exit(0x0100);
        },
        Ok(f) => f,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read content of markdown file");

    for line in contents.lines() {
        if line.starts_with("title: ") {
            let title = line.split("title: ").collect::<Vec<&str>>();
             return title_to_filename(Path::new(title[1].trim()).to_string_lossy().to_string());
        }
    };
    return "no_title".to_string()
}

//-----------------------------------------------------------------------------
// Format
//-----------------------------------------------------------------------------

pub fn title_to_filename(title: String) -> String {
    let forbidden_chars = ['!', '*', '(', ')', '"'];
    let mut title = title.chars().filter(|c| !forbidden_chars.contains(c)).collect::<String>();
    title = title.replace(" ", "-");
    title = title.replace("'", "-");
    title
}

//-----------------------------------------------------------------------------
// Mkdir
//-----------------------------------------------------------------------------

pub fn is_file(path: &str) -> bool {
    if path.ends_with("/"){
        false
    } else {
        true
    }
}

pub fn mkdir_all (path: &str) {
 if is_file(&path) {
     let parent_dir = Path::new(&path).parent();
     match parent_dir {
         Some(dir) => fs::create_dir_all(dir).expect("failed to create dir"),
         None => (),
     }
 } else {
     fs::create_dir_all(path).expect("failed to create dir");
 }
}
