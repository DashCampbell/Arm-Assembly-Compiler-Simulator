use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::path::Path;
use std::result;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    name: String,
    kind: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    created: String,
    link: String,
    description: String,
    content: String,
    author: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    files: String,
    folder: String,
}

/// Returns JSON string of data
pub fn read_directory(dir_path: &str) -> Folder {
    let new_path = Path::new(dir_path);
    let paths = fs::read_dir(new_path).unwrap(); // files & folders
    println!("new path {:?}", new_path);

    let mut files: Vec<FileInfo> = Vec::new();

    for path in paths {
        let path = path.unwrap();
        let meta = path.metadata().unwrap();

        let filename = path.file_name().into_string().unwrap_or("ERROR".into());
        let kind = String::from(if meta.is_dir() { "directory" } else { "file" });
        let file_path = dir_path.to_owned() + &filename;

        let new_file_info = FileInfo {
            name: filename,
            kind,
            path: file_path,
        };
        files.push(new_file_info);
    }
    let files_str = match serde_json::to_string(&files) {
        Ok(str) => str,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    Folder {
        files: files_str,
        folder: new_path.file_name().unwrap().to_str().unwrap().into(),
    }
}

/// Returns the contents of the file.
/// Returns an error, if file is not readable.
pub fn read_file(path: &str) -> result::Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(_) => {
            eprintln!("Error reading: {}", path);
            let path = Path::new(path);
            return Err(path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or("Can't read file name.")
                .to_string());
        }
    }
}

pub fn write_file(path: &str, content: &str) -> String {
    let file_path = Path::new(path);
    match fs::write(file_path, content) {
        Ok(()) => String::from("OK"),
        Err(_err) => format!("ERROR Writing to file {}", path),
    }
}

pub fn create_directory(path: &str) -> Result<()> {
    let dir_path = Path::new(path);
    fs::create_dir(dir_path).unwrap_or_default();
    Ok(())
}
pub fn remove_file(path: &str) -> Result<()> {
    let file_path = Path::new(path);
    fs::remove_file(file_path).unwrap_or_default();
    Ok(())
}

pub fn remove_folder(path: &str) -> Result<()> {
    let folder_path = Path::new(path);
    fs::remove_dir_all(folder_path).unwrap_or_default();
    Ok(())
}
