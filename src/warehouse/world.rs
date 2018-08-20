use warehouse::object::Object;
use std::sync::{ Arc, Mutex };

pub trait World {
    fn initialize(&mut self);
    fn get_items(&mut self) -> &mut Vec<Arc<Mutex<Object>>>;
}
