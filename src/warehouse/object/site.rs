extern crate uuid;

use warehouse::object::{ Object, Location };
use warehouse::Storage;
use std::sync::{ Arc, Mutex };
use self::uuid::Uuid;

pub struct Site {
    id: String,
    location: Location,
    locked: bool,
    storage: Storage,
    pub pick_storage: Storage
}

impl Site {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            id: format!("ste-{}", Uuid::new_v4()),
            location: Location { x: 0, y: 0, z: 0 },
            locked: false,
            storage: Storage::new(),
            pick_storage: Storage::new()
        }))
    }
}

impl Object for Site {
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
    fn render(&self) -> Result<String, &'static str> {
        Ok(format!("{{ \"id\": \"{}\", \"storage\": {}, \"pick_storage\": {}, \"location\": {} }}", 
                    self.id(),
                    self.storage().render()?,
                    self.pick_storage.render()?,
                    self.location().render()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let obj = Site::new();
        let obj = obj.lock().unwrap();
        assert_eq!(&obj.id()[..4], "ste-");
        Uuid::parse_str(&obj.id()[4..]).unwrap();
    }

    #[test]
    fn test_render() {
        let obj = Site::new();
        let obj = obj.lock().unwrap();
        assert_eq!(obj.render().unwrap(), 
            format!("{{ \"id\": \"{}\", \"storage\": [], \"pick_storage\": [], \"location\": {{ \"x\": 0, \"y\": 0, \"z\": 0 }} }}", obj.id()));
    }
}
