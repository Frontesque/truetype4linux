use std::fs::{ rename, read_dir, create_dir_all };
use std::path::{ Path, PathBuf };
use std::env;
use std::process::Command;
use crate::EXTRACT_DIRECTORY;

fn install_dir() -> PathBuf {
    let home = env::home_dir().expect("Unable to locate home directory");
    return PathBuf::new().join(home).join(".local/share/fonts/");
}

pub fn refresh_font_config() {
    let _ = Command::new("fc-cache")
        .arg("-fv")
        .status();
}

pub fn main() {
    let _ = create_dir_all(install_dir());
    for entry in read_dir(EXTRACT_DIRECTORY).unwrap() {
        let file_raw = entry.unwrap();
        let file_name = file_raw.file_name();
        let file_ttf = file_name.to_string_lossy();
        println!("Installing... {}", file_ttf);
        let new_path = Path::new(&install_dir()).join(&file_name);
        println!("{:?}", new_path);
        let _ = rename(file_raw.path(), new_path);
    }
}