use std::env;
use std::fs;
use std::io;

pub trait Command{
    fn run(args: &[&str]);
    fn help();
}

pub struct Directory {}

impl Command for Directory {
    fn run(_: &[&str]){
        println!("Bierząca lokacja:");
        visit_dirs().unwrap();
    }

    fn help(){
        println!("{}", "Wypisuje na ekran zawartość obecnego katalogu.");
    }
}

fn visit_dirs() -> io::Result<()> {
    for entry in fs::read_dir(env::current_dir().expect("Nie mogłem odczytać bierzącej lokalizacji."))? {
        let file_name = entry?.file_name();

        if let Some(fc) = file_name.to_str() {
            println!("{}", fc);
        }
    }
    Ok(())
}

pub struct ChangeDirectory {}

impl Command for ChangeDirectory {
    fn run(args: &[&str]){
        let mut current = env::current_dir().unwrap();
        current.push(&args[0]);

        match env::set_current_dir(current) {
            Ok(_) => {}
            Err(e) => {
                println!("Nie mogłem zmienić lokalizacji, sprawdź czy istnieje. {}", e);
            }
        }
    }

    fn help() {
        println!("{}", "Zmienia katalog na podany.");
    }
}

pub struct PrintWorkingDirectory{}

impl Command for PrintWorkingDirectory{
    fn run(_: &[&str]){
        println!("{}", env::current_dir().unwrap().display());
    }

    fn help() {
        println!("{}", "Wypisuje na terminal ścieżkę do obecnego katalogu.");
    }
}

pub struct Touch{}

impl Command for Touch{
    fn run(args: &[&str]){
        use std::fs::File;
        File::create(&args[0]).expect("Nie mogłem stworzyć pliku.");
    }

    fn help() {
        println!("{}", "Tworzy nowy plik o podanej nazwie.");
    }
}

pub struct CopyFile {}

impl Command for CopyFile {
    fn run(args: &[&str]){

    }

    fn help() {
        println!("{}", "Kopiuje plik z jednego miejsca w drugie.");
    }
}
