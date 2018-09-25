#![allow(dead_code)]

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::process::Command;
use std::process::Output;
use colored::*;

// Command IO

pub fn run_command(command: String, not_assert: bool) -> Output{
    let output = Command::new("sh")
        .arg("-c")
        .arg(command.clone())
        .output()
        .expect("sh command failed to start");
    /*println!("command: {}", command);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));*/
    if !not_assert {
        //assert!(output.status.success());
        if !output.status.success() {
            println!("{}: {}", command.red(), String::from_utf8_lossy(&output.stderr).red());
            assert!(false);
        }
    }
    return output;
}

pub fn get_home() -> String {
    let home = run_command("echo $HOME".to_string(), false);
    return String::from_utf8_lossy(&home.stdout).replace("\n", "");
}

// File IO
fn correct_path(path: String) -> String {
    let r = path.replace("~", get_home().as_str());
    return r;//r.replace(" ", "\\ ");
}

pub fn read(filename: String) -> String {
    let filename_correct = correct_path(filename);
    return fs::read_to_string(filename_correct).expect("Something went wrong when reading the file");
}

pub fn read_bytes(filename: String) -> Vec<u8> {
    let filename_correct = correct_path(filename);
    return fs::read(filename_correct).expect("Something went wrong when reading the file");
}

pub fn overwrite(filename: String, contents: String) {
    let filename_correct = correct_path(filename);
    fs::write(filename_correct, contents).expect("Couldn't write to file");
}

pub fn overwrite_bytes(filename: String, contents: Vec<u8>) {
    let filename_correct = correct_path(filename);
    fs::write(filename_correct, contents).expect("Couldn't write to file");
}

pub fn write(filename: String, new_contents: String) {
    let old_contents = read(filename.clone());
    let contents = old_contents + &new_contents;
    overwrite(filename, contents);
}

pub fn copy(old:String, new: String){
    overwrite_bytes(new, read_bytes(old));
}

pub fn create_empty_file(filename: String) {
    overwrite(filename, "".to_string());
}

pub fn create_empty_dir(path: String) {
    let path_correct = correct_path(path);
    fs::create_dir(path_correct).expect("Couldn't create new dir");
}

pub fn remove_file(path: String) {
    let path_correct = correct_path(path);
    fs::remove_file(path_correct).expect("Couldn't remove file");
}

pub fn path_exists(path: String) -> bool {
    let path_correct = correct_path(path);
    return Path::new(&path_correct).exists();
}

pub fn add_path_to_rc(path: String){
    let path_correct = correct_path(path.clone());
    let path_export = "export PATH=\"".to_string()+&path_correct+":$PATH\"\n";
    if path_exists("~/.bashrc".to_string()) {
        write("~/.bashrc".to_string(), path_export.to_string());
        if path_exists("~/.zshrc".to_string()) {
            write("~/.zshrc".to_string(), path_export.to_string());
        }
    }else if path_exists("~/.bash_profile".to_string()) {
        write("~/.bash_profile".to_string(), path_export.to_string());
        if path_exists("~/.zshrc".to_string()) {
            write("~/.zshrc".to_string(), path_export.to_string());
        }
    }else if path_exists("~/.zshrc".to_string()) {
        write("~/.zshrc".to_string(), path_export.to_string());
    }else{
        println!("{}{}{}", "Couldn't add ".red(), path.red(), "to the rc/porfile file".red());
    }
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

pub fn download_file(url: String, path: String) {
    let filename_correct = path.replace("~", get_home().as_str());
    let url_str:&str = url.as_ref();
    println!("{} {} -> {}","Downloading".cyan(), url_str, filename_correct);
    let mut resp = reqwest::get(url_str).expect("URL use error");
    assert!(resp.status().is_success());

    let mut buffer = Vec::new();
    resp.read_to_end(&mut buffer).expect("Response error");
    let mut f = File::create(filename_correct).expect("File creation error");
    f.write_all(&buffer).expect("Write error");
    f.flush().expect("Flush error");

}
