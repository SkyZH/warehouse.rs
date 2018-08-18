use warehouse::command::Command;

pub trait CommandQueue : Command {
    fn schedule(&mut self, command: Box<Command>) -> Result<(), &'static str>;
}
