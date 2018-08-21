use warehouse::object::{ Object, Location };
use warehouse::Storage;
use std::sync::{ Arc, Mutex };

pub struct TestObject {
    location: Location,
    locked: bool,
    storage: Storage
}

impl TestObject {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            location: Location { x: 0, y: 0, z: 0 },
            locked: false,
            storage: Storage::new()
        }))
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
    fn get_lock(&mut self) -> &mut bool {
        &mut self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let obj = TestObject::new();
        let obj = obj.lock().unwrap();
        assert_eq!(obj.id(), "test-0");
    }
    #[test]
    fn test_storage() {
        let obj = TestObject::new();
        let mut obj = obj.lock().unwrap();
        let storage = obj.get_storage();
        storage.add(1, 1).unwrap();
    }
    #[test]
    fn test_location() {
        let obj = TestObject::new();
        let mut obj = obj.lock().unwrap();
        let location = obj.get_location();
        location.x = 233;
    }
    #[test]
     #[should_panic(expected="object already locked")]
    fn test_lock() {
        let obj = TestObject::new();
        let mut obj = obj.lock().unwrap();
        obj.lock().unwrap();
        obj.lock().unwrap();
        
    }
    #[test]
    #[should_panic(expected="object already unlocked")]
    fn test_unlock() {
        let obj = TestObject::new();
        let mut obj = obj.lock().unwrap();
        obj.unlock().unwrap();
    }
    #[test]
    fn test_lock_and_unlock() {
        let obj = TestObject::new();
        let mut obj = obj.lock().unwrap();
        obj.lock().unwrap();
        assert!(obj.locked);
        obj.unlock().unwrap();
        assert!(!obj.locked);
    }
    #[test]
    fn test_render() {
        let obj = TestObject::new();
        let mut obj = obj.lock().unwrap();
        {
            let storage = obj.get_storage();
            storage.add(233, 1).unwrap();
        }
        {
            let location = obj.get_location();
            location.x = 233; location.y = 234; location.z = 235;
        }
        assert_eq!(obj.render().unwrap(), "{ \"id\": \"test-0\", \"storage\": [{ \"item\": 233, \"count\": 1 }], \"location\": { \"x\": 233, \"y\": 234, \"z\": 235 } }");
    }
}
