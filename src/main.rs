#[allow(non_upper_case_globals)]

#[cfg(target_os = "linux")]
const EXTRACT_DIRECTORY: &str = "./truetype4linux";
#[cfg(target_os = "linux")]
mod fonts;
#[cfg(target_os = "linux")]
mod cabextract;
#[cfg(target_os = "linux")]
mod install;

pub fn splash() {
    println!("  _   _   _  _   _ \n | |_| |_| || | | |\n | __| __| || |_| |\n | |_| |_|__   _| |\n  \\__|\\__|  |_| |_|");
    println!("   TrueType4Linux");
    println!("   v{}", env!("CARGO_PKG_VERSION"));
    println!("   https://github.com/frontesque/truetype4linux");
    println!("\nAll done!");
}

#[cfg(target_os = "windows")]
fn main () {
    splash();
}

#[cfg(target_os = "linux")]
fn main() {
    if cabextract::ensure() == false {
        return println!("Please install cabextract to run truetype4linux.");
    }

    println!("Extracting executables...");
    fonts::main();
    println!("Unpacking fonts...");
    cabextract::extract();
    println!("Moving fonts...");
    install::main();
    println!("Installing fonts...");
    install::refresh_font_config();

    splash();

}
