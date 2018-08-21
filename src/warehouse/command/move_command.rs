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
        let mut world = self.world.lock().unwrap();
        if world.check_location(self.location) > 0 {
            let mut bot = self.bot.lock().unwrap();
            bot.unlock().unwrap();
            return Err("target location not available");
        }
        world.notify_will_move(self.bot.clone(), self.location).unwrap();
        let mut bot = self.bot.lock().unwrap();
        *bot.get_location() = self.location;
        bot.unlock().unwrap();
        Ok(false)
    }
    fn render(&self) -> Result<String, &'static str> {
        let bot = self.bot.lock().unwrap();
        Ok(format!("{{ \"type\": \"{}\", \"bot\": \"{}\", \"location\": {} }}", 
            "move", bot.id(), self.location.render()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warehouse::object::Bot;
    use warehouse::World;
    use warehouse::command::{ CommandQueue, ParallelCommandQueue };

    #[test]
    #[should_panic(expected="target location far away")]
    fn test_initialize() {
        let world = Arc::new(Mutex::new(World::new()));
        let bot = Bot::new();
        let mut cmd = BotMoveCommand::new(bot, Location::new(1, 1, 1), world);
        cmd.initialize().unwrap();
    }

    #[test]
    fn test_initialize_lock() {
        let world = Arc::new(Mutex::new(World::new()));
        let bot = Bot::new();
        let mut cmd = BotMoveCommand::new(bot.clone(), Location::new(0, 0, 1), world);
        cmd.initialize().unwrap();
        assert!(*bot.lock().unwrap().get_lock());
    }

    #[test]
    fn test_consume() {
        let world = Arc::new(Mutex::new(World::new()));
        let bot = Bot::new();
        world.lock().unwrap().add_items(vec![bot.clone()]);
        let mut cmd = BotMoveCommand::new(bot.clone(), Location::new(0, 0, 1), world);
        cmd.initialize().unwrap();
        assert!(!cmd.consume().unwrap());
        assert_eq!(*bot.lock().unwrap().location(), Location::new(0, 0, 1));
        assert!(!*bot.lock().unwrap().get_lock());
    }
    #[test]
    #[should_panic(expected="target location not available")]
    fn test_consume_collision_1() {
        let world = Arc::new(Mutex::new(World::new()));
        let mut target_loc = Location::new(0, 0, 0);
        let (bot1, bot2) = (Bot::new(), Bot::new());
        {
            let (mut bot1, mut bot2) = (bot1.lock().unwrap(),bot2.lock().unwrap());
            let (loc1, loc2) = (bot1.get_location(), bot2.get_location());
            *loc2 = (*loc2).front();
            target_loc = (*loc1).front();
        }
        world.lock().unwrap().add_items(vec![bot1.clone(), bot2.clone()]);
        let mut cmd = BotMoveCommand::new(bot1.clone(), target_loc, world);
        cmd.initialize().unwrap();
        assert!(!cmd.consume().unwrap());
    }
    #[test]
    #[should_panic(expected="target location not available")]
    fn test_consume_collision_2() {
        let world = Arc::new(Mutex::new(World::new()));
        let mut target_loc = Location::new(0, 0, 0);
        let (bot1, bot2) = (Bot::new(), Bot::new());
        {
            let (mut bot1, mut bot2) = (bot1.lock().unwrap(),bot2.lock().unwrap());
            let (loc1, loc2) = (bot1.get_location(), bot2.get_location());
            *loc2 = (*loc2).front().front();
            target_loc = (*loc1).front();
        }
        world.lock().unwrap().add_items(vec![bot1.clone(), bot2.clone()]);
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(BotMoveCommand::new(bot1.clone(), target_loc, world.clone())).unwrap();
        queue.schedule(BotMoveCommand::new(bot2.clone(), target_loc, world.clone())).unwrap();
        queue.consume().unwrap();
    }
    #[test]
    #[should_panic(expected="target location not available")]
    fn test_consume_collision_3() {
        let world = Arc::new(Mutex::new(World::new()));
        let (mut target_loc_1, mut target_loc_2) = (Location::new(0, 0, 0), Location::new(0, 0, 0));
        let (bot1, bot2) = (Bot::new(), Bot::new());
        {
            let (mut bot1, mut bot2) = (bot1.lock().unwrap(),bot2.lock().unwrap());
            let (loc1, loc2) = (bot1.get_location(), bot2.get_location());
            *loc2 = (*loc2).front();
            target_loc_1 = *loc2;
            target_loc_2 = *loc1;
        }
        world.lock().unwrap().add_items(vec![bot1.clone(), bot2.clone()]);
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(BotMoveCommand::new(bot1.clone(), target_loc_1, world.clone())).unwrap();
        queue.schedule(BotMoveCommand::new(bot2.clone(), target_loc_2, world.clone())).unwrap();
        queue.consume().unwrap();
    }
}
