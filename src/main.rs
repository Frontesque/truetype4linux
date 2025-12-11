#[allow(non_upper_case_globals)]

const EXTRACT_DIRECTORY: &str = "./truetype4linux";

mod fonts;
mod cabextract;
mod install;
// mod os_release;

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
}
