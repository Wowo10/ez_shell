use std::env;
use std::fs;
use std::io;

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

pub fn pwd(){
    println!("{}", env::current_dir().unwrap().display());
}
