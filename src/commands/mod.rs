pub trait Command {
    fn run(args: &[&str]);
    fn help();
}

pub fn message_helper(header: &str, explanation: &str, aliases: &str) {
    println!("{}\n{}\naliasy: {}", header, explanation, aliases);
}

fn check_args_len(args: &[&str], expected_amount: u8) -> bool{
    let args_len = args.len();

    if args_len < expected_amount.into() {
        println!("Zbyt mało parametrów przekazanych oczekuję {}, otrzymałem: {}", expected_amount, args_len);
        false
    } else {
        true
    }
}

pub struct Directory {}

impl Command for Directory {
    fn run(_: &[&str]) {
        println!("Bierząca lokalizacja:");
        visit_dirs().unwrap();
    }

    fn help() {
        message_helper(
            "dir:",
            "Wypisuje na ekran zawartość obecnego katalogu.",
            "[ls]",
        );
    }
}

use std::io;

fn visit_dirs() -> io::Result<()> {
    use std::env;
    use std::fs;

    for entry in
        fs::read_dir(env::current_dir().expect("Nie mogłem odczytać bierzącej lokalizacji."))?
    {
        let file_name = entry?.file_name();

        if let Some(fc) = file_name.to_str() {
            println!("{}", fc);
        }
    }
    Ok(())
}

pub struct ChangeDirectory {}

impl Command for ChangeDirectory {
    fn run(args: &[&str]) {
        check_args_len(args, 1);
        
        use std::env;

        let mut current = env::current_dir().unwrap();
        current.push(&args[0]);

        match env::set_current_dir(current) {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "Nie mogłem zmienić lokalizacji, sprawdź czy istnieje. {}",
                    e
                );
            }
        }
    }

    fn help() {
        message_helper("cd:", "Zmienia katalog na podany.", "");
    }
}

pub struct PrintWorkingDirectory {}

impl Command for PrintWorkingDirectory {
    fn run(_: &[&str]) {
        use std::env;

        println!("{}", env::current_dir().unwrap().display());
    }

    fn help() {
        message_helper(
            "pwd:",
            "Wypisuje na terminal ścieżkę do obecnego katalogu.",
            "",
        );
    }
}

////////////////////////////////FILE HANDLING

fn create_file(name: &str) -> std::fs::File {
    use std::fs::File;

    File::create(name).expect("Nie mogłem stworzyć pliku.")
}

fn write_to_file(file: &mut std::fs::File, content: &[u8]){
    use std::io::Write;

    file.write_all(content).expect(
        "Nie otrzymałem prawa do zapisu, sprawdź czy plik nie jest otwarty w innym programie",
    );
}

fn delete_file(name: &str) {
    use std::fs;

    fs::remove_file(name).expect("Nie udało mi się usunąć pliku, sprawdź czy istnieje.");
}

fn read_file(name: &str) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(name).expect("Nie mogę otworzyć pliku, sprawdź czy istnieje.");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Nie mogę przeczytać pliku, sprawdź czy nie jest używany przez inny program.");
       
    content
}

fn copy_file(sourcename: &str, targetname: &str) {
    let content = read_file(sourcename);

    let mut target_file = create_file(targetname);

    write_to_file(&mut target_file, content.as_bytes());
}

pub struct Touch {}

impl Command for Touch {
    fn run(args: &[&str]) {
        check_args_len(args, 1);
        
        let mut file = create_file(args[0]);

        if args.len() > 1 && args[1] != ""{
            write_to_file(&mut file, args[1].as_bytes());
        }
    }

    fn help() {
        message_helper(
            "touch: <nazwa_pliku> <*tekst*>",
            "Tworzy nowy plik o podanej nazwie, opcjonalnie zapisuje tekst do pliku",
            "[create]",
        );
    }
}

pub struct DeleteFile {}

impl Command for DeleteFile {
    fn run(args: &[&str]) {
        check_args_len(args, 1);
        
        delete_file(args[0]);
    }

    fn help() {
        message_helper("remove: <nazwa_pliku>", "Usuwa podany plik.", "[rm] [del]");
    }
}

pub struct ReadFile {}

impl Command for ReadFile {
    fn run(args: &[&str]) {
        check_args_len(args, 1);
        
        let content = read_file(args[0]);
        println!("{}", content);
    }

    fn help() {
        message_helper(
            "cat: <filename>",
            "Wypisuje zawartość pliku na konsolę.",
            "[type] [read]",
        );
    }
}

pub struct CopyFile {}

impl Command for CopyFile {
    fn run(args: &[&str]) {
        check_args_len(args, 2);

        copy_file(args[0], args[1]);
    }

    fn help() {
        message_helper(
            "copy: <źródło> <cel>",
            "Kopiuje plik z jednego miejsca w drugie.",
            "[cp]",
        );
    }
}

pub struct MoveFile {}

impl Command for MoveFile {
    fn run(args: &[&str]) {
        check_args_len(args, 2);

        copy_file(args[0], args[1]);

        delete_file(args[0]);
    }

    fn help() {
        message_helper(
            "move: <źródło> <cel>",
            "Przenosi plik z jednego miejsca w drugie.",
            "[mv]",
        );
    }
}
