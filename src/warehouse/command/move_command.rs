use warehouse::command::Command;
use warehouse::object::{ Bot, Location, Object };
use warehouse::World;

use std::sync::{ Arc, Mutex };

pub struct BotMoveCommand {
    bot: Arc<Mutex<Bot>>,
    location: Location,
    world: Arc<Mutex<World>>
}

impl BotMoveCommand {
    pub fn new(bot: Arc<Mutex<Bot>>, location: Location, world: Arc<Mutex<World>>) -> Box<Self> {
        Box::new(Self {
            bot: bot,
            location: location,
            world: world
        })
    }
}

impl Command for BotMoveCommand {
    fn initialize(&mut self) -> Result<(), &'static str> {
        let mut bot = self.bot.lock().unwrap();
        if !self.location.nearby(*bot.location()) {
            return Err("target location far away")
        }
        bot.lock()
    }
    fn consume(&mut self) -> Result<bool, &'static str> {
        let mut bot = self.bot.lock().unwrap();
        let mut world = self.world.lock().unwrap();
        if world.check_location(self.location) > 0 {
            bot.unlock().unwrap();
            return Err("target location not available");
        }
        world.notify_will_move(self.bot.clone(), self.location);
        (*bot.get_location()) = self.location;
        bot.unlock().unwrap();
        Ok(false)
    }
    fn render(&self) -> Result<String, &'static str> {
        let bot = self.bot.lock().unwrap();
        Ok(format!("{{ type: \"{}\", bot: \"{}\", location: {} }}", 
            "move", bot.id(), self.location.render()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warehouse::object::Bot;
    use warehouse::World;

}
