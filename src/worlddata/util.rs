use warehouse::object::{ Object, Location };
use std::sync::{ Arc, Mutex };

pub fn move_location(obj: Arc<Mutex<Object>>, location: Location) {
    let mut obj = obj.lock().unwrap();
    let loc = obj.get_location();
    *loc = location;
}

pub fn map_as_object <'a, T: 'a + Object> (vec: &Vec<Arc<Mutex<T>>>) -> Vec<Arc<Mutex<Object + 'a>>>{
    vec.iter().cloned().map(|obj| obj as Arc<Mutex<Object>>).collect()
}
