use warehouse::object::{ Object, Location };
use std::sync::{ Arc, Mutex };

pub fn move_location(obj: Arc<Mutex<Object>>, location: Location) {
    let mut obj = obj.lock().unwrap();
    let loc = obj.get_location();
    *loc = location;
}
