use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

pub trait Command {
    fn run(args: &[&str]);
    fn help();
}

pub fn message_helper(header: &str, explanation: &str, aliases: &str){
    println!("{}\n{}\naliasy: {}", header, explanation, aliases);
}

pub struct Directory {}

impl Command for Directory {
    fn run(_: &[&str]) {
        println!("Bierząca lokacja:");
        visit_dirs().unwrap();
    }

    fn help() {
        message_helper("dir:","Wypisuje na ekran zawartość obecnego katalogu.","[ls]");
    }
}

fn visit_dirs() -> io::Result<()> {
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
        message_helper("cd:","Zmienia katalog na podany.","");
    }
}

pub struct PrintWorkingDirectory {}

impl Command for PrintWorkingDirectory {
    fn run(_: &[&str]) {
        println!("{}", env::current_dir().unwrap().display());
    }

    fn help() {
        message_helper("pwd:","Wypisuje na terminal ścieżkę do obecnego katalogu.","");
    }
}

pub struct Touch {}

impl Command for Touch {
    fn run(args: &[&str]) {
        File::create(&args[0]).expect("Nie mogłem stworzyć pliku.");
    }

    fn help() {
        message_helper("touch: <nazwa_pliku>","Tworzy nowy plik o podanej nazwie.", "[create]");
    }
}

pub struct DeleteFile {}

impl Command for DeleteFile {
    fn run(args: &[&str]) {
        fs::remove_file(&args[0])
            .expect("Nie udało mi się usunąć pliku, sprawdź czy istnieje.");
    }

    fn help() {
        message_helper("remove: <nazwa_pliku>","Usuwa podany plik.", "[rm] [del]");
    }
}

pub struct ReadFile {}

impl Command for ReadFile {
    fn run(args: &[&str]) {
        let mut file =
            File::open(&args[0]).expect("Nie mogę otworzyć pliku, sprawdź czy istnieje.");

        let mut s = String::new();
        file.read_to_string(&mut s).expect(
            "Nie mogę przeczytać pliku, sprawdź czy nie jest używany przez inny program.",
        );
        println!("{}", s);
    }

    fn help() {
        message_helper("cat: <filename>","Wypisuje zawartość pliku na konsolę.", "[type] [read]");
    }
}

pub struct CopyFile {}

impl Command for CopyFile {
    fn run(args: &[&str]) {
        let mut file =
            File::open(&args[0]).expect("Nie mogę otworzyć pliku, sprawdź czy istnieje.");

        let mut s = String::new();
        file.read_to_string(&mut s).expect(
            "Nie mogę przeczytać pliku, sprawdź czy nie jest używany przez inny program.",
        );

        let mut target_file =
            File::create(&args[1]).expect("Nie mogłem stworzyć pliku docelowego.");
        target_file.write_all(s.as_bytes()).expect("");
    }

    fn help() {
        message_helper("copy: <źródło> <cel>","Kopiuje plik z jednego miejsca w drugie.", "[cp]");
    }
}

pub struct MoveFile {}

impl Command for MoveFile {
    fn run(args: &[&str]) {
        let mut file =
            File::open(&args[0]).expect("Nie mogę otworzyć pliku, sprawdź czy istnieje.");

        let mut s = String::new();
        file.read_to_string(&mut s).expect(
            "Nie mogę przeczytać pliku, sprawdź czy nie jest używany przez inny program.",
        );
        

        let mut target_file =
            File::create(&args[1]).expect("Nie mogłem stworzyć pliku docelowego.");
        target_file.write_all(s.as_bytes()).expect("");

        fs::remove_file(&args[0])
            .expect("Nie udało mi się usunąć pliku, sprawdź czy istnieje.");
    }

    fn help() {
        message_helper("move: <źródło> <cel>","Przenosi plik z jednego miejsca w drugie.", "[mv]");
    }
}
