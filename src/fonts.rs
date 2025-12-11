// This file handles bundling and extracting the corefonts from 
// github@pushcx/corefonts into this binary.

// Imports
use std::fs::{ write, create_dir_all };
use crate::EXTRACT_DIRECTORY;

// Font Includes
const FONT_AndaleMono:    &[u8; 198384] = include_bytes!("../corefonts/andale32.exe");
const FONT_Arial:         &[u8; 554208] = include_bytes!("../corefonts/arial32.exe");
const FONT_ArialBlack:    &[u8; 168176] = include_bytes!("../corefonts/arialb32.exe");
const FONT_ComicSansMS:   &[u8; 246008] = include_bytes!("../corefonts/comic32.exe");
const FONT_CourierNew:    &[u8; 646368] = include_bytes!("../corefonts/courie32.exe");
const FONT_Georgia:       &[u8; 392440] = include_bytes!("../corefonts/georgi32.exe");
const FONT_Impact:        &[u8; 173288] = include_bytes!("../corefonts/impact32.exe");
const FONT_TimesNewRoman: &[u8; 661728] = include_bytes!("../corefonts/times32.exe");
const FONT_TrebuchetMS:   &[u8; 357200] = include_bytes!("../corefonts/trebuc32.exe");
const FONT_Verdana:       &[u8; 351992] = include_bytes!("../corefonts/verdan32.exe");
const FONT_Webdings:      &[u8; 185072] = include_bytes!("../corefonts/webdin32.exe");

// Standardized Writeout
fn extract_font(exe: &str, content: &[u8]) {
    let path = format!("{}/{}", EXTRACT_DIRECTORY, exe);
    println!("Extracting: {} [{} bytes]", path, content.len());
    write(&path, content).expect(format!("Unable to write file: '{}'", path).as_str());
}

fn ensure() {
    create_dir_all(EXTRACT_DIRECTORY).expect(format!("Unable to create directory: '{}'", EXTRACT_DIRECTORY).as_str());
}

// Main
pub fn main() {
    ensure();
    extract_font("andale32.exe", FONT_AndaleMono);
    extract_font("arial32.exe", FONT_Arial);
    extract_font("arialb32.exe", FONT_ArialBlack);
    extract_font("comic32.exe", FONT_ComicSansMS);
    extract_font("courie32.exe", FONT_CourierNew);
    extract_font("georgi32.exe", FONT_Georgia);
    extract_font("impact32.exe", FONT_Impact);
    extract_font("times32.exe", FONT_TimesNewRoman);
    extract_font("trebuc32.exe", FONT_TrebuchetMS);
    extract_font("verdan32.exe", FONT_Verdana);
    extract_font("webdin32.exe", FONT_Webdings);
}