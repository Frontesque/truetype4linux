use crate::EXTRACT_DIRECTORY;

use std::fs;
use std::io;
use std::path::Path;
use zip::read::ZipArchive;

pub fn extract_zip(archive_path: &str) -> io::Result<()> {
    let fname = Path::new(archive_path);
    let file = fs::File::open(fname)?;

    let mut archive = ZipArchive::new(file)?;
    let output_path = Path::new(EXTRACT_DIRECTORY);

    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        println!("Unpacking: {}", file.name());
        let outpath = match file.enclosed_name() {
            Some(path) => output_path.join(path),
            None => continue, // Skip files with suspicious paths (zip slip vulnerability)
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}