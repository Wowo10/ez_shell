use std::env;
use std::fs;
use std::io;

pub fn dir() {
    println!("Current directory:");
    visit_dirs().unwrap();
}

fn visit_dirs() -> io::Result<()> {
    for entry in fs::read_dir(env::current_dir().expect("cannot read current directory"))? {
        let file_name = entry?.file_name();

        if let Some(fc) = file_name.to_str() {
            println!("{}", fc);
        }
    }
    Ok(())
}