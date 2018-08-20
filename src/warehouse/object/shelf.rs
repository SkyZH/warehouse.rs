extern crate uuid;

use warehouse::object::{ Object, Location };
use warehouse::Storage;
use warehouse::command::Command;
use std::sync::{ Arc, Mutex };
use self::uuid::Uuid;

pub struct Shelf {
    id: String,
    location: Location,
    locked: bool,
    storage: Storage
}

impl Shelf {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            id: format!("slf-{}", Uuid::new_v4()),
            location: Location { x: 0, y: 0, z: 0 },
            locked: false,
            storage: Storage::new()
        }))
    }
}

impl Object for Shelf {
    fn id(&self) -> &str {
        &self.id
    }
    fn storage(&self) -> &Storage {
        &self.storage
    }
    fn location(&self) -> &Location {
        &self.location
    }
    fn get_storage(&mut self) -> &mut Storage {
        &mut self.storage
    }
    fn get_location(&mut self) -> &mut Location {
        &mut self.location
    }
    fn get_lock(&mut self) -> &mut bool {
        &mut self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let obj = Shelf::new();
        let obj = obj.lock().unwrap();
        assert_eq!(&obj.id()[..4], "slf-");
        Uuid::parse_str(&obj.id()[4..]).unwrap();
    }
}
