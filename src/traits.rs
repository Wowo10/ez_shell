pub trait Command{
    fn run();
    fn help() -> &'static str;
}