use std::process::Command;
use std::fs::{ remove_file, read_dir };
use crate::EXTRACT_DIRECTORY;

pub fn ensure() -> bool {
    let command_capture = Command::new("cabextract").arg("--version").output();
    let output: String = match command_capture {
        Ok(data) => String::from_utf8(data.stdout).unwrap(),
        Err(error) => error.to_string()
    };
    
    if output.contains("cabextract version") {
        return true;
    } else {
        return false;
        // TODO: Automatically install 'cabextract'
    }
}

fn cleanup(file_name: &str) {
    let _ = remove_file(format!("{}/{}", EXTRACT_DIRECTORY, file_name));
}

fn cabextract(file: &str) {
    let _ = Command::new("cabextract")
        .arg(file)
        .current_dir(EXTRACT_DIRECTORY)
        .status();
}

pub fn extract() {
    for entry in read_dir(EXTRACT_DIRECTORY).unwrap() {
        let file_raw = entry.unwrap().file_name();
        let file_exe = file_raw.to_string_lossy();
        println!("Unpacking... {}", file_exe);
        cabextract(&file_exe);
        let _ = remove_file(format!("{}/{}", EXTRACT_DIRECTORY, file_exe));
    }
    // Post-extract cleanup
    cleanup("FONTINST.EXE");
    cleanup("fontinst.exe");
    cleanup("fontinst.inf");
    cleanup("Licen.TXT");
    cleanup("andale.inf");
    cleanup("ADVPACK.DLL");
    cleanup("W95INF16.DLL");
    cleanup("W95INF32.DLL");
}