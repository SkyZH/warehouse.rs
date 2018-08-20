use warehouse::command::Command;
use warehouse::object::{ Object, Bot };
use std::sync::{ Arc, Mutex };
use std::mem::swap;

pub struct TransferCommand {
    from: Arc<Mutex<Object>>,
    to: Arc<Mutex<Object>>
}

pub struct BotTransferToCommand {
}


impl BotTransferToCommand {
    pub fn new(from: Arc<Mutex<Bot>>, to: Arc<Mutex<Object>>) -> Box<TransferCommand> {
        Box::new(TransferCommand {
            from: from, to: to
        })
    }
}

pub struct BotTransferFromCommand {
}

impl BotTransferFromCommand {
    pub fn new(from: Arc<Mutex<Object>>, to: Arc<Mutex<Bot>>) -> Box<TransferCommand> {
        Box::new(TransferCommand {
            from: from, to: to
        })
    }
}

impl Command for TransferCommand {
    fn initialize(&mut self) -> Result<(), &'static str> {
        let mut from = self.from.lock().unwrap();
        let mut to = self.to.lock().unwrap();
        if to.storage().items.len() > 0 {
            return Err("target storage not empty");
        }
        if !to.location().nearby(*from.location()) {
            return Err("source and target are far away");
        }
        from.lock()?;
        match to.lock() {
            Ok(_) => {},
            Err(err) => {
                from.unlock().unwrap();
                return Err(err);
            }
        };
        Ok(())
    }
    fn consume(&mut self) -> Result<bool, &'static str> {
        let mut from = self.from.lock().unwrap();
        let mut to = self.to.lock().unwrap();
        {
            let from_storage = from.get_storage();
            let to_storage = to.get_storage();
            swap(&mut from_storage.items, &mut to_storage.items);
        }
        from.unlock().unwrap();
        to.unlock().unwrap();
        Ok(false)
    }
    fn render(&self) -> Result<String, &'static str> {
        let from = self.from.lock().unwrap();
        let to = self.to.lock().unwrap();
        Ok(format!("{{ type: \"{}\", from: \"{}\", to: \"{}\" }}", "transfer", from.id(), to.id()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warehouse::object::TestObject;

    #[test]
    fn test_lock() {
        let bot = Bot::new();
        let obj = TestObject::new();
        let mut cmd = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd.initialize().unwrap();
        let bot_locked = *bot.lock().unwrap().get_lock();
        let obj_locked = *obj.lock().unwrap().get_lock();
        assert!(bot_locked);
        assert!(obj_locked);
    }
    #[test]
    fn test_unlock() {
        let bot = Bot::new();
        let obj = TestObject::new();
        let mut cmd = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd.initialize().unwrap();
        assert_eq!(cmd.consume().unwrap(), false);
        let bot_locked = *bot.lock().unwrap().get_lock();
        let obj_locked = *obj.lock().unwrap().get_lock();
        assert!(!bot_locked);
        assert!(!obj_locked);
    }
    #[test]
    #[should_panic]
    fn test_race() {
        let bot = Bot::new();
        let obj = TestObject::new();
        let mut cmd = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd.initialize().unwrap();
        let mut cmd2 = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd2.initialize().unwrap();
    }
    #[test]
    fn test_storage() {
        let bot = Bot::new();
        let obj = TestObject::new();
        {
            let (mut bot, mut obj) = (bot.lock().unwrap(), obj.lock().unwrap());
            let (bot_storage, _obj_storage) = (bot.get_storage(), obj.get_storage());
            bot_storage.add(1, 1).unwrap(); bot_storage.add(3, 3).unwrap();
        }
        let mut cmd = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd.initialize().unwrap();
        cmd.consume().unwrap();
        {
            let (bot, obj) = (bot.lock().unwrap(), obj.lock().unwrap());
            let (bot_storage, obj_storage) = (bot.storage(), obj.storage());
            assert_eq!(bot_storage.items, vec![]);
            assert_eq!(obj_storage.items, vec![(1, 1), (3, 3)]);
        }
    }
    #[test]
    #[should_panic(expected="target storage not empty")]
    fn test_storage_full() {
        let bot = Bot::new();
        let obj = TestObject::new();
        {
            let (mut bot, mut obj) = (bot.lock().unwrap(), obj.lock().unwrap());
            let (bot_storage, obj_storage) = (bot.get_storage(), obj.get_storage());
            bot_storage.add(1, 1).unwrap(); bot_storage.add(3, 1).unwrap();
            obj_storage.add(2, 1).unwrap();
        }
        let mut cmd = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd.initialize().unwrap();
        cmd.consume().unwrap();
    }
    #[test]
    #[should_panic(expected="source and target are far away")]
    fn test_far_away() {
        let bot = Bot::new();
        let obj = TestObject::new();
        {
            let (mut bot, mut obj) = (bot.lock().unwrap(), obj.lock().unwrap());
            let (bot_loc, obj_loc) = (bot.get_location(), obj.get_location());
            bot_loc.x = 2; bot_loc.y = 2; bot_loc.z = 2;
            obj_loc.x = 1; obj_loc.y = 3; obj_loc.z = 2;
        }
        let mut cmd = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd.initialize().unwrap();
        cmd.consume().unwrap();
    }
    #[test]
    fn test_render() {
        let bot = Bot::new();
        let obj = TestObject::new();
        let mut cmd = BotTransferToCommand::new(bot.clone(), obj.clone());
        cmd.initialize().unwrap();
        assert_eq!(cmd.render().unwrap(), 
            format!("{{ type: \"transfer\", from: \"{}\", to: \"{}\" }}", bot.lock().unwrap().id(), obj.lock().unwrap().id()));
    }
}
