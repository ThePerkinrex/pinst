#![allow(dead_code)]

use std::fs;
use std::io::Read;

// File IO
pub fn read(filename: String) -> String {
    return fs::read_to_string(filename).expect("Something went wrong when reading the file");
}

pub fn overwrite(filename: String, contents: String) {
    fs::write(filename, contents).expect("Couldn't write to file");
}

pub fn write(filename: String, new_contents: String) {
    let old_contents = read(filename.clone());
    let contents = old_contents + &new_contents;
    overwrite(filename, contents);
}

pub fn create_empty_file(filename: String) {
    overwrite(filename, "".to_string());
}

use reqwest;

// Internet IO

pub fn read_from_url(url: String) -> String{
    let url_str:&str = url.as_ref();
    let mut resp = reqwest::get(url_str).expect("URL use error");
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content).expect("Response error");
    return content.clone();
}
