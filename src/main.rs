#![feature(const_vec_new)]

mod commands;
use commands::*;

fn wait_for_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Nie wpisałeś odpowiedniego ciągu znaków.");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn prompt(command: &str) {
    use std::env;
    print!(
        "{} ~> {}{}",
        env::current_dir()
            .expect("Nie mogłem odczytać bierzącej lokalizacji.")
            .display(),
        command,
        if command != "" { "\n" } else { "" }
    );
}

fn previous_input() -> String {
    unsafe {
        match COMMAND_QUEUE.last() {
            None => "".to_string(),
            Some(command) => command.clone(),
        }
    }
}

fn handle_input(input: &str) -> bool {
    let split = input.split(" ");
    let vec: Vec<&str> = split.collect();

    let first = vec.first().unwrap();
    let vec = &vec[1..];

    println!("{:?}, {}", vec, vec.len()); //DEBUG

    match first[..].to_lowercase().as_ref() {
        "dir" | "ls" => {
            if vec.len() == 0 || (vec[0] != "help" && vec[0] != "--help"){
                commands::Directory::run(&vec);
            }else{
                commands::Directory::help();
            }
        }
        "pwd" => {
            if vec.len() == 0 || (vec[0] != "help" && vec[0] != "--help"){
                commands::PrintWorkingDirectory::run(&vec);
            }else{
                commands::PrintWorkingDirectory::help();
            }
        }
        "cd" => {
            if vec.len() == 0 || (vec[0] != "help" && vec[0] != "--help"){
                commands::ChangeDirectory::run(&vec);
            }else{
                commands::ChangeDirectory::help();
            }
        }
        "cp" | "copy" =>{
            if vec.len() == 0 || (vec[0] != "help" && vec[0] != "--help"){
                commands::CopyFile::run(&vec);
            }else{
                commands::CopyFile::help();
            }
        }
        "touch" => {
            if vec.len() == 0 || (vec[0] != "help" && vec[0] != "--help"){
                commands::Touch::run(&vec);
            }else{
                commands::Touch::help();
            }
        }
        "same" => {
            let previous_command = previous_input();

            prompt(previous_command.as_ref());
            handle_input(previous_command.as_ref());
            return true;
        }
        "exit" | "q" => {
            println!("Do widzenia!");
            unsafe {
                EXIT = true;
            }
        }
        _ => {
            println!("Nieznana komenda!");
            return true;
        }
    }
    false
}

static mut EXIT: bool = false;
static mut COMMAND_QUEUE: Vec<String> = Vec::new();

fn main() {
    let welcome_message =
        "\nWitaj w EZ_Shell, wprowadź komendę, możesz wyjść za pom,ocą komendy exit.\nWielkości znaków w nazwach komend nie mają znaczenia\n\n";

    println!("{}", welcome_message);

    let mut local_exit: bool = false;

    while !local_exit {
        prompt("");
        let input = wait_for_input();

        let same_command = handle_input(input.as_ref());

        unsafe {
            local_exit = EXIT;

            if !same_command {
                COMMAND_QUEUE.push(input.clone());
            }
        }
    }
}
