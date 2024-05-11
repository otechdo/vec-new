#![allow(clippy::multiple_crate_versions)]

use chrono::Utc;
use std::env::args;
use std::fs::{remove_file, File, OpenOptions};
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, ExitCode};
use inquire::{Confirm, Text};

const TYPE: &str = "&str";
const NAME: &str = "vec.rs";

///
/// # Panics
///
fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file: File = File::open(filename).expect("failed to open filename");
    io::BufReader::new(file).lines()
}

fn parse_file_lines(filename: &str) -> Vec<String> {
    let mut file_lines: Vec<String> = Vec::new();
    read_lines(filename).for_each(|line| match line {
        Ok(l) => {
            file_lines.push(l);
        }
        Err(x) => println!("{x}"),
    });
    file_lines
}

///
/// # Panics
///
#[must_use]
pub fn exec(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to execute cmd")
        .success()
}

fn get_file() -> String {
    let x = format!(
        "/tmp/{}.rs",
        Utc::now().to_string().split_whitespace().next().expect("")
    );
    x
}

fn run(dest: &str, src: &str, const_name: &str) -> ExitCode {
    gen(dest, src, parse_file_lines(src).len(), const_name)
}

fn const_name() -> String {
    let args: Vec<String> = args().collect();
    args.get(2)
        .expect("Constant name is missing")
        .to_uppercase()
}

fn write(file: &str) -> File {
    let f = OpenOptions::new().append(true).open(file).unwrap();
    f
}

fn gen(dest: &str, src: &str, lines: usize, constant: &str) -> ExitCode {
    assert!(File::create(dest).is_ok());
    let mut f = write(dest);

    writeln!(f, "pub const {constant} : [{TYPE};{lines}]  = [").expect("Failed to create vector");
    for line in &parse_file_lines(src) {
        writeln!(f, "\"{line}\",").expect("Failed to create vector");
    }
    writeln!(f, "];").expect("Failed to create vector");
    assert!(exec("sh", &["-c", format!("xdg-open {dest}").as_str()]));
    remove_file(src).expect("failed to remove");
    exit(0);
}

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        if args.get(1).unwrap().eq("--interactive") {
            let mut constant: String = String::new();
            loop {
                constant.clear();
                constant.push_str(Text::new("Please enter the constant name : ").prompt().unwrap().to_uppercase().as_str());
                if !constant.is_empty() {
                    break;
                }
            }
            if Path::new(NAME).exists() {
                remove_file(NAME).unwrap();
            }
            File::create_new(NAME).expect("file already exist");
            let mut f = OpenOptions::new().append(true).open(NAME).expect("failed to open file");
            loop {
                let data = Text::new("Enter the value append in vec : ").prompt().expect("");
                f.write(format!("{}\n",data).as_bytes()).expect("failed to add dada");
                if Confirm::new("Add an other data to vec ? ").with_default(true).prompt().unwrap().eq(&false)
                {
                    break;
                }
            }
            return run(
                get_file().as_str(),
                NAME,
                constant.as_str(),
            );
        }
    }
    if args.len() == 3 {
        return run(
            get_file().as_str(),
            args.get(1).unwrap().as_str(),
            const_name().as_str(),
        );
    }
    if args.len() == 2 {
        return run(
            get_file().as_str(),
            args.get(1).unwrap().as_str(),
            args.get(1).unwrap().as_str(),
        );
    }
    println!("vec-new <file>");
    println!("vec-new <file> <constant_name>");
    exit(1);
}
