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

fn out_to_string(output: Vec<u8>) -> String {
    let mut word = String::new();
    for c in output {
        match c {
            10 => break,
            _ => word.push(c as char),
        }
    }
    return word;
}

fn list_files(dir: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    files.push(String::from(".."));

    let output = Command::new("ls")
        .current_dir(dir)
        .output()
        .expect("ls command failed to start");
    let out = output.stdout;

    let mut all_files: Vec<String> = to_string(out);
    files.append(&mut all_files);

    return files;
}
fn print_files(files: &Vec<String>, index: &usize) {
    println!("index: {}", index);
    for f in files {
        println!("{}", f);
    }
}

fn change_directory(dir: &String, location: String) -> String {
    // Command::new("cd dir");
    // Command::new("cd").current_dir(dir).arg(location);
    let new_dir: String = vec![dir.to_owned(), location.to_owned()].join("/");
    let output = Command::new("pwd")
        .current_dir(new_dir)
        .output()
        .expect("error");

    return out_to_string(output.stdout);
}

fn main() {
    println!("{}[2J", 27 as char); // clear terminal

    // Get home directory
    let mut dir = String::new();
    match env::var("HOME") {
        Ok(v) => dir = v,
        Err(e) => panic!("$HOME is not set ({})", e),
    }

    let mut index: usize = 0;

    let stdout = Term::buffered_stdout();
    let mut files: Vec<String> = vec![];

    loop {
        println!("{}[2J", 27 as char); // clear terminal
        println!("{}", dir);

        files = list_files(&dir);
        print_files(&files, &index);

        let mut key;

        key = stdout.read_char().expect("Error");
        match key {
            'w' => {
                // move up
                if (index == 0) {
                    index = 0;
                } else {
                    index -= 1;
                }
            }
            's' => {
                // move down
                index += 1;
                if (index >= files.len().try_into().unwrap()) {
                    index = files.len() - 1;
                }
            }
            'a' => {
                // move back
                dir = change_directory(&dir, "..".to_owned());
                index = 0;
            }
            'd' => {
                // move forward
                let location = &files[index];
                dir = change_directory(&dir, location.to_string());
                index = 0;
            }
            _ => continue,
        }
    }

    // print!("{:?}", my_files);

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
