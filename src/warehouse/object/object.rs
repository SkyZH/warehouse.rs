use warehouse::Storage;

pub trait Object {
    fn get_storage(&self) -> Storage;
}
