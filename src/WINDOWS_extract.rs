use std::fs::read_dir;
use std::path::PathBuf;

use std::fs::File;
use std::io::{self, Read, Write};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const SUPPORTED_FILE_TYPES: [&str; 2] = ["ttf", "ttc"/*, "fon" */];
const OUTPUT_FILE: &str = "./extracted.tt4l";

pub fn get_font_list() -> Vec<PathBuf> {
    let mut file_list = Vec::new();

    if let Ok(entries) = read_dir("C:\\Windows\\Fonts") {
        for entry in entries {
            if let Ok(entry) = entry {
                
                let has_allowed_extension = entry.path().extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| SUPPORTED_FILE_TYPES.contains(&ext.to_lowercase().as_str()))
                    .unwrap_or(false);
                
                if has_allowed_extension {
                    file_list.push(entry.path());
                }

            }
        }
    }
    
    return file_list;
}

fn create_zip_from_paths(paths: Vec<PathBuf>) -> io::Result<()> {
    // 1. Create the destination file
    let path = std::path::Path::new(OUTPUT_FILE);
    let file = File::create(path)?;
    let mut zip = ZipWriter::new(file);

    // 2. Set compression options
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 3. Loop through the list
    for path in paths {
        if path.is_file() {
            // Get just the filename (e.g., "Arial.ttf") instead of the full path
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown_file");

            // Start a new file inside the ZIP
            println!("Packaging: {}", name);
            zip.start_file(name, options)?;

            // Open the actual file and copy its contents
            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    // 4. Finalize the ZIP file
    zip.finish()?;
    Ok(())
}

pub fn main() {

    println!("Looking for system fonts...");
    let fonts = get_font_list();
    println!("Found {} compatible fonts!", fonts.len());

    println!("Packaging fonts...");
    let _ = create_zip_from_paths(fonts);

    println!("\n\nYou may now move '{}' to your linux machine and run './truetypeforlinux --from {}'\n\n", OUTPUT_FILE, OUTPUT_FILE);
}