use std::env;
use std::fs;
use std::io;

use super::traits::Command;

pub struct Directory {}

impl Command for Directory {
    fn run(){
        println!("Current directory:");
        visit_dirs().unwrap();
    }

    fn help() -> &'static str {
        "Wypisuje na ekran zawartość obecnego katalogu."
    }

    fn name() -> &'static str {
        "dir"
    }
}


pub fn dir() {
    println!("Current directory:");
    visit_dirs().unwrap();
}

fn visit_dirs() -> io::Result<()> {
    for entry in fs::read_dir(env::current_dir().expect("Cannot read current directory"))? {
        let file_name = entry?.file_name();

        if let Some(fc) = file_name.to_str() {
            println!("{}", fc);
        }
    }
    Ok(())
}

pub fn cd(path: &str) {
    let mut current = env::current_dir().unwrap();
    current.push(path);

    match env::set_current_dir(current) {
        Ok(_) => {}
        Err(e) => {
            println!("Couldn`t change directory, check if it does exist. {}", e);
        }
    }
}

pub struct ChangeDirectory {   
}

impl Command for ChangeDirectory {
    fn run(){

    }

    fn help() -> &'static str {
        "zmienia katalog"
    }

    fn name() -> &'static str {
        "cd"
    }
}


pub struct PrintWorkingDirectory{}

impl Command for PrintWorkingDirectory{
    fn run(){
        println!("{}", env::current_dir().unwrap().display());
    }

    fn help() -> &'static str {
        "Wypisuje na terminal obecny katalog."
    }

    fn name() -> &'static str {
        "pwd"
    }
}

pub fn pwd() {
    println!("{}", env::current_dir().unwrap().display());
}

pub struct Copy {}

impl Command for Copy {
    fn run(){

    }

    fn help() -> &'static str {
        "Kopiuje plik z jednego miejsca w drugie."
    }

    fn name() -> &'static str {
        "copy"
    }
}
