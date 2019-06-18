pub trait Command{
    fn name() -> &'static str;
    fn run();
    fn help() -> &'static str;
}