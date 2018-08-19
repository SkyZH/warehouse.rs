use warehouse::Storage;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

pub trait Object {
    fn id(&self) -> &str;
    fn storage(&self) -> &Storage;
    fn location(&self) -> &Location;
    fn get_storage(&mut self) -> &mut Storage;
    fn get_location(&mut self) -> &mut Location;
    fn get_lock(&mut self) -> &mut bool;
    fn lock(&mut self) -> Result<(), &'static str> {
        let locked = self.get_lock();
        match *locked {
            true => Err("object already locked"),
            false => {
                *locked = true;
                Ok(())
            }
        }
    }
    fn unlock(&mut self) -> Result<(), &'static str> {
        let locked = self.get_lock();
        match *locked {
            false => Err("object already unlocked"),
            true => {
                *locked = false;
                Ok(())
            }
        } 
    }
    fn render(&self) -> Result<String, &'static str> {
        let storage = self.storage().render();
        match storage {
            Ok(storage) => Ok([
                "{ id: \"", self.id(), "\", ",
                "storage: ", &*storage, ", ",
                "x: ", &*self.location().x.to_string(), ", ",
                "y: ", &*self.location().y.to_string(), ", ",
                "z: ", &*self.location().z.to_string(), " }"
            ].join("")),
            Err(err) => Err(err)
        }
    }
}

