use warehouse::command::Command;
use warehouse::object::{ Object, Bot, Location };
use warehouse::Storage;
use std::sync::{ Arc, Mutex };
use std::mem::swap;

pub struct BotMoveCommand {
    bot: Arc<Mutex<Bot>>,
    location: Location
}

#[cfg(test)]
mod tests {
    use super::*;
}
