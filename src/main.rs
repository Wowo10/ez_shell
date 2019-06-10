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
    let welcome_message =
        "\nWelcome to EZ_Shell, input your command, you`re welcome to exit any time.\n\n";
    println!("{}", welcome_message);
    prompt();
    let input = wait_for_input();
    println!("{}", input);
}
