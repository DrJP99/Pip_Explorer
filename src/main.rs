#![allow(unused)]
use console::Term;
use std::env;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::vec;

fn to_string(output: Vec<u8>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut word = String::new();
    for c in output {
        match c {
            10 => {
                result.push(word);
                word = String::new();
                continue;
            }
            _ => {
                word.push(c as char);
            }
        }
    }

    return result;
}

fn list_files(dir: String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    files.push(String::from(".."));

    let output = Command::new("ls")
        .output()
        .expect("ls command failed to start");
    let out = output.stdout;

    let mut all_files: Vec<String> = to_string(out);
    files.append(&mut all_files);

    return files;
}

fn main() {
    println!("{}[2J", 27 as char); // clear terminal

    // Get home directory
    let mut dir = String::new();
    match env::var("HOME") {
        Ok(v) => dir = v,
        Err(e) => panic!("$HOME is not set ({})", e),
    }

    let index = 0;

    println!("{}", dir);
    let stdout = Term::buffered_stdout();
    // loop {
    //     let mut key;

    //     key = stdout.read_char().expect("Error");

    //     println!("You pressed: {}", key);
    // }

    let my_files = list_files(dir);
    print!("{:?}", my_files);

    // let mut name = String::new();
    // println!("Enter your name:");
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");

    // println!("{}[2J", 27 as char); // clear terminal

    // println!("Hello, {}", name);

    // let output = Command::new("sh")
    //     .arg("echo $USER")
    //     .output()
    //     .expect("Command failed");

    // println!("{:?}", output.stdout);
}
