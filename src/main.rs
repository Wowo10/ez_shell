fn wait_for_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn prompt() {
    print!("{}", "~> ");
}

fn main() {
    let mut exit = false;

    let welcome_message =
        "\nWelcome to EZ_Shell, input your command, you`re welcome to exit any time.\nCommands are case insensitive\n\n";

    let mut command_queue: Vec<String> = Vec::new();

    println!("{}", welcome_message);

    while !exit {
        prompt();
        let input = wait_for_input();
        command_queue.push(input.clone());

        match input.to_lowercase().as_ref() {
            "exit" => {
                println!("Bye!");
                exit = true;
            }
            _ => {
                println!("Unknown Command!");
            }
        }

    }
}
