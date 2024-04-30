#![allow(unused)]
use colored::Colorize;
use console::Term;
use file::File;
use std::default;
use std::env;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::vec;

mod file;

fn to_string(output: Vec<u8>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut word = String::new();
    let mut line: Vec<u8> = vec![];
    let mut files: Vec<File> = vec![];
    let mut first: bool = true;

    println!("{:?}", output);
    for c in output {
        match c {
            10 => {
                result.push(word);
                if (first) {
                    first = false;
                } else {
                    files.push(out_to_file(&line));
                }
                word = String::new();
                line = vec![];
                continue;
            }
            _ => {
                line.push(c);
                word.push(c as char);
            }
        }
    }

    return result;
}

fn out_to_string(output: Vec<u8>) -> String {
    println!("{:?}", output);
    let mut word = String::new();
    for c in output {
        match c {
            10 => break,
            _ => word.push(c as char),
        }
    }
    return word;
}

fn out_to_file(output: &Vec<u8>) -> File {
    println!("{:?}", output);

    // let mut permissions: String;
    // let mut num_links: u32;
    // let mut owner: String;
    // let mut group: String;
    // let mut size: String;
    // let mut month: String;
    // let mut day: String;
    // let mut time: String;
    // let mut name: String;

    const TOTAL_COLS: usize = 9;
    let mut cols: [String; TOTAL_COLS] = Default::default();

    let mut curr: usize = 0;

    let mut word = String::new();
    for c in output {
        match c {
            32 => {
                println!("word: {} :: curr: {}", word, curr);
                if (word.to_string().is_empty()) {
                    continue;
                } else {
                    cols[curr] = word.to_string();
                    word = String::from("");
                    curr += 1;
                }
            }
            _ => {
                word.push(*c as char);
            }
        }
    }

    println!("{:?}", cols);

    return File {
        name: cols[8].to_string(),
        extension: file::get_extension(cols[8].to_string()),
        permissions: cols[0].to_string(),
    };
}

fn list_files(dir: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    files.push(String::from(".."));

    let output = Command::new("ls")
        .arg("-l")
        .current_dir(dir)
        .output()
        .expect("ls command failed to start");
    let out = output.stdout;

    let mut all_files: Vec<String> = to_string(out);
    files.append(&mut all_files);

    return files;
}

fn print_files(files: &Vec<String>, index: &usize) {
    // println!("index: {}", index);
    for (i, f) in files.iter().enumerate() {
        if (i == *index) {
            println!("{}", f.bold().black().on_truecolor(225, 225, 225));
        } else {
            println!("{}", f.truecolor(225, 225, 225));
        }
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
    let mut refresh: bool = true;

    loop {
        println!("{}[2J", 27 as char); // clear terminal
        println!("{}\n", dir);

        if (refresh) {
            files = list_files(&dir);
            refresh = false;
        }
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
                refresh = true;
            }
            'd' => {
                // move forward
                let location = &files[index];
                dir = change_directory(&dir, location.to_string());
                index = 0;
                refresh = true;
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
