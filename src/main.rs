#![allow(non_upper_case_globals)]

pub fn splash() {
    println!("  _   _   _  _   _ \n | |_| |_| || | | |\n | __| __| || |_| |\n | |_| |_|__   _| |\n  \\__|\\__|  |_| |_|");
    println!("   TrueType4Linux");
    println!("   v{}", env!("CARGO_PKG_VERSION"));
    println!("   https://github.com/frontesque/truetype4linux");
    println!("\nAll done!");
}


#[cfg(target_os = "windows")]
mod WINDOWS_extract;
#[cfg(target_os = "windows")]
fn main () {
    WINDOWS_extract::main();
    splash();
    println!("You may now close this window.");
    std::thread::park();
}

#[cfg(target_os = "linux")]
const EXTRACT_DIRECTORY: &str = "./truetype4linux";
#[cfg(target_os = "linux")]
mod fonts;
#[cfg(target_os = "linux")]
mod extract_cabextract;
#[cfg(target_os = "linux")]
mod extract_tt4l;
#[cfg(target_os = "linux")]
mod install;

#[cfg(target_os = "linux")]
fn install_from_bundled() {
    if extract_cabextract::ensure() == false {
        return println!("Please install cabextract to run truetype4linux.");
    }

    println!("Extracting executables...");
    fonts::main();
    println!("Unpacking fonts...");
    extract_cabextract::extract();
    println!("Moving fonts...");
    install::main();
    println!("Installing fonts...");
    install::refresh_font_config();
}

#[cfg(target_os = "linux")]
fn install_from_tt4l(file_name: &str) {
    println!("Unpacking fonts...");
    let _ = extract_tt4l::extract_zip(file_name);
    println!("Moving fonts...");
    install::main();
    println!("Installing fonts...");
    install::refresh_font_config();
}

#[cfg(target_os = "linux")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1].to_lowercase() == "--from" { // This is not elegant and should be improved. Currently there is only one arg.
        install_from_tt4l(&args[2]);
    } else {
        install_from_bundled();
    }
    splash();
}
