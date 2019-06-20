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
    let args = &vec[1..];

    match first[..].to_lowercase().as_ref() {
        "dir" | "ls" => {
            run_command(args, &commands::Directory::run, &commands::Directory::help);
        }
        "pwd" => {
            run_command(
                args,
                &commands::PrintWorkingDirectory::run,
                &commands::PrintWorkingDirectory::help,
            );
        }
        "cd" => {
            run_command(
                args,
                &commands::ChangeDirectory::run,
                &commands::ChangeDirectory::help,
            );
        }
        "cp" | "copy" => {
            run_command(args, &commands::CopyFile::run, &commands::CopyFile::help);
        }
        "mv" | "move" => {
            run_command(args, &commands::MoveFile::run, &commands::MoveFile::help);
        }
        "touch" | "create" => {
            run_command(args, &commands::Touch::run, &commands::Touch::help);
        }
        "del" | "rm" | "remove" => {
            run_command(
                args,
                &commands::DeleteFile::run,
                &commands::DeleteFile::help,
            );
        }
        "cat" | "type" | "read" => {
            run_command(args, &commands::ReadFile::run, &commands::ReadFile::help);
        }
        "same" => {
            return run_special_command(args, &same_command_run, &same_command_help);
        }
        "exit" | "q" => {
            return run_special_command(args, &exit_command_run, &exit_command_help);
        }
        _ => {
            println!("Nieznana komenda!");
            return true;
        }
    }
    false
}

fn same_command_run() {
    let previous_command = previous_input();

    prompt(previous_command.as_ref());
    handle_input(previous_command.as_ref());
}

fn same_command_help() {
    commands::message_helper("same:", "Powtarza poprzednią komendę.", "");
}

fn exit_command_run() {
    println!("Do widzenia!");
    unsafe {
        EXIT = true;
    }
}

fn exit_command_help() {
    commands::message_helper("exit:", "Wychodzi z programu", "[q]");
}

fn run_special_command(args: &[&str], run: &Fn(), help: &Fn()) -> bool {
    if args.len() == 0 || (args[0] != "help" && args[0] != "--help") {
        run();
        true
    } else {
        help();
        false
    }
}

fn run_command(args: &[&str], run: &Fn(&[&str]), help: &Fn()) {
    if args.len() == 0 || (args[0] != "help" && args[0] != "--help") {
        run(args);
    } else {
        help();
    }
}

static mut EXIT: bool = false;
static mut COMMAND_QUEUE: Vec<String> = Vec::new();

fn main() {
    let welcome_message =
        "\nWitaj w ez_shell, wprowadź komendę.\nDostępne komendy to: dir pwd cd cp mv touch del cat same exit.\nAby uzyskać pomoc do komend użyj przełącznika --help [przykład: exit --help].\n";

    println!("{}", welcome_message);
    exit_command_help();

    let welcome_message = "\nWielkości znaków w nazwach komend nie mają znaczenia\n\n";
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
