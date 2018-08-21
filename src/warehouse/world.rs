use warehouse::object::{ Object, Location };
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;

pub struct World {
    items: Vec<Arc<Mutex<Object>>>,
    is_available: HashMap<Location, u64>
}

impl World {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            is_available: HashMap::new()
        }
    }
    pub fn add_items(&mut self, items: Vec<Arc<Mutex<Object>>>) {
        let mut items = items;
        for item in &items {
            let item = item.lock().unwrap();
            self.is_available.entry(*item.location()).or_insert(0);
            (*self.is_available.get_mut(item.location()).unwrap()) += 1;
        }
        self.items.append(&mut items);
    }
    pub fn get_items(&mut self) -> &mut Vec<Arc<Mutex<Object>>> {
        &mut self.items
    }
    pub fn check_location(&self, location: Location) -> u64 {
        match self.is_available.get(&location) {
            Some(cnt) => *cnt,
            None => 0
        }
    }
    pub fn notify_will_move(&mut self, obj: Arc<Mutex<Object>>, to_location: Location) -> Result<(), &'static str> {
        let obj = obj.lock().unwrap();
        let from_location = *obj.location();
        self.is_available.entry(to_location).or_insert(0);
        match self.is_available.get_mut(&from_location) {
            Some(cnt) => {
                if *cnt <= 0 {
                    return Err("not enough item");
                }
                *cnt -= 1;
            },
            None => return Err("not enough item")
        }
        (*self.is_available.get_mut(&to_location).unwrap()) += 1;
        Ok(())
    }
    pub fn render(&self) -> Result<String, &'static str> {
        let mut error_flag: Option<&'static str> = None;
        let result = self.items.iter()
            .map(|object: &Arc<Mutex<Object>>| match object.lock().unwrap().render() {
                Ok(result) => result,
                Err(err) => {
                    error_flag = Some(err);
                    "".to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        match error_flag {
            Some(err) => Err(err),
            None => Ok(format!("{{ \"objects\": [{}] }}", result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warehouse::object::Bot;

    #[test]
    fn test_location_available() {
        let mut world = World::new();
        assert_eq!(world.check_location(Location::new(1, 2, 3)), 0);
    }
    #[test]
    fn test_multiple_location() {
        let mut world = World::new();
        let mut bot = Bot::new();
        {
            let mut bot = bot.lock().unwrap();
            let loc = bot.get_location();
            (*loc).x = 1; (*loc).y = 2; (*loc).z = 3;
        }
        let items = vec![bot.clone() as Arc<Mutex<Object>>, bot.clone() as Arc<Mutex<Object>>, bot.clone() as Arc<Mutex<Object>>];
        world.add_items(items);
        assert_eq!(world.check_location(Location { x: 1, y: 2, z: 3 }), 3);
    }
    #[test]
    #[should_panic]
    fn test_notify_move_panic() {
        let mut world = World::new();
        let bot = Bot::new();
        world.notify_will_move(bot, Location::new(1, 1, 1)).unwrap();
    }
    #[test]
    fn test_notify_move() {
        let mut world = World::new();
        let bot = Bot::new();
        world.add_items(vec![bot.clone() as Arc<Mutex<Object>>]);
        world.notify_will_move(bot, Location::new(1, 1, 1)).unwrap();
        assert_eq!(*world.is_available.get(&Location::new(1, 1, 1)).unwrap(), 1);
    }
    #[test]
    fn test_render() {
        let mut world = World::new();
        let bot = Bot::new();
        world.add_items(vec![bot.clone() as Arc<Mutex<Object>>]);
        assert_eq!(world.render().unwrap(), format!("{{ \"objects\": [{{ \"id\": \"{}\", \"storage\": [], \"location\": {{ \"x\": 0, \"y\": 0, \"z\": 0 }} }}] }}", bot.lock().unwrap().id()));
    }
}
