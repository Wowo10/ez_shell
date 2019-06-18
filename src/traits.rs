pub trait Command{
    fn run(args: &Vec<String>);
    fn help() -> &'static str;
}