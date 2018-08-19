use warehouse::object::Object;
use warehouse::object::Location;
use warehouse::Storage;

pub struct TestObject {
    location: Location,
    locked: bool,
    storage: Storage
}

impl TestObject {
    fn new() -> Self {
        Self {
            location: Location { x: 0, y: 0, z: 0 },
            locked: false,
            storage: Storage::new()
        }
    }
}

impl Object for TestObject {
    fn id(&self) -> &str {
        "test-0"
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
    fn lock(&mut self) -> Result<(), &'static str> {
        match self.locked {
            true => Err("already locked"),
            false => {
                self.locked = true;
                Ok(())
            }
        }
    }
    fn unlock(&mut self) -> Result<(), &'static str> {
        match self.locked {
            false => Err("already unlocked"),
            true => {
                self.locked = false;
                Ok(())
            }
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let obj = TestObject::new();
        assert_eq!(obj.id(), "test-0");
    }
    #[test]
    fn test_storage() {
        let mut obj = TestObject::new();
        let mut storage = obj.get_storage();
        storage.items.push(1);
    }
    #[test]
    fn test_location() {
        let mut obj = TestObject::new();
        let mut location = obj.get_location();
        location.x = 233;
    }
    #[test]
    #[should_panic]
    fn test_lock() {
        let mut obj = TestObject::new();
        obj.lock().unwrap();
        obj.lock().unwrap();
        
    }
    #[test]
    #[should_panic]
    fn test_unlock() {
        let mut obj = TestObject::new();
        obj.unlock().unwrap();
    }
    #[test]
    fn test_lock_and_unlock() {
        let mut obj = TestObject::new();
        obj.lock().unwrap();
        assert!(obj.locked);
        obj.unlock().unwrap();
        assert!(!obj.locked);
    }
    #[test]
    fn test_render() {
        let mut obj = TestObject::new();
        {
            let storage = obj.get_storage();
            storage.items.push(233);
        }
        {
            let location = obj.get_location();
            location.x = 233; location.y = 234; location.z = 235;
        }
        assert_eq!(obj.render().unwrap(), "{ id: \"test-0\", storage: [233], x: 233, y: 234, z: 235 }");
    }
    
}
