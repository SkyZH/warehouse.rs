use warehouse::Storage;
use warehouse::object::Location;

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
        match self.storage().render() {
            Ok(storage) => Ok(format!("{{ \"id\": \"{}\", \"storage\": {}, \"location\": {} }}", 
                            self.id(),
                            storage,
                            self.location().render())),
            Err(err) => Err(err)
        }
    }
}
