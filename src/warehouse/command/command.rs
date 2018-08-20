pub trait Command {
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn consume(&mut self) -> Result<bool, &'static str>;
    fn render(&self) -> Result<String, &'static str>;
}
