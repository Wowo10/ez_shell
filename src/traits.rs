pub trait Command{
    fn run(args: &[&str]);
    fn help() -> &'static str;
}