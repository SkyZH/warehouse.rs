use warehouse::command::Command;
use warehouse::object::{ Site, Object };
use warehouse::ItemSet;

use std::sync::{ Arc, Mutex };

pub struct PickCommand {
    site: Arc<Mutex<Site>>,
    item: ItemSet,
    reverse: bool
}

pub struct SitePutCommand {
}

pub struct SitePickCommand {
}

impl SitePickCommand {
    pub fn new(site: Arc<Mutex<Site>>, item: ItemSet) -> Box<PickCommand> {
        Box::new(PickCommand {
            site: site,
            item: item,
            reverse: false
        })
    }
}

impl SitePutCommand {
    pub fn new(site: Arc<Mutex<Site>>, item: ItemSet) -> Box<PickCommand> {
        Box::new(PickCommand {
            site: site,
            item: item,
            reverse: true
        })
    }
}

impl Command for PickCommand {
    fn initialize(&mut self) -> Result<(), &'static str> {
        let mut site = self.site.lock().unwrap();
        {
            let storage = site.storage();
            if self.reverse {
                if site.pick_storage.have(self.item.0) < self.item.1 {
                    return Err("not enough item");
                }
            } else {
                if storage.have(self.item.0) < self.item.1 {
                    return Err("not enough item");
                }
            }
        }
        site.lock()
    }
    fn consume(&mut self) -> Result<bool, &'static str> {
        let mut site = self.site.lock().unwrap();
        {
            let storage = site.get_storage();
            match self.reverse {
                false => storage.take(self.item.0, self.item.1).unwrap(),
                true => storage.add(self.item.0, self.item.1).unwrap()
            }
        }
        {
            match self.reverse {
                true => site.pick_storage.take(self.item.0, self.item.1).unwrap(),
                false => site.pick_storage.add(self.item.0, self.item.1).unwrap()
            }
            
        }
        site.unlock()?;
        Ok(false)
    }
    fn render(&self) -> Result<String, &'static str> {
        let site = self.site.lock().unwrap();
        match self.reverse {
            true => Ok(format!("{{ \"type\": \"{}\", \"site\": \"{}\", \"item\": {}, \"count\": {} }}", 
            "put", site.id(), self.item.0, self.item.1)),
            false => Ok(format!("{{ \"type\": \"{}\", \"site\": \"{}\", \"item\": {}, \"count\": {} }}", 
            "pick", site.id(), self.item.0, self.item.1))
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected="not enough item")]
    fn test_pick_initialize() {
        let site = Site::new();
        {
            let mut site = site.lock().unwrap();
            let storage = site.get_storage();
            storage.add(1, 3).unwrap();
        }
        let mut cmd = SitePickCommand::new(site, (1, 4));
        cmd.initialize().unwrap();
    }

    #[test]
    fn test_pick_initialize_lock() {
        let site = Site::new();
        {
            let mut site = site.lock().unwrap();
            let storage = site.get_storage();
            storage.add(1, 3).unwrap();
        }
        let mut cmd = SitePickCommand::new(site.clone(), (1, 1));
        cmd.initialize().unwrap();
        assert!(*site.lock().unwrap().get_lock());
    }

    #[test]
    fn test_pick_consume() {
        let site = Site::new();
        {
            let mut site = site.lock().unwrap();
            let storage = site.get_storage();
            storage.add(1, 3).unwrap();
        }
        let mut cmd = SitePickCommand::new(site.clone(), (1, 2));
        cmd.initialize().unwrap();
        cmd.consume().unwrap();
        {
            let site = site.lock().unwrap();
            let storage = site.storage();
            assert_eq!(storage.have(1), 1);  
        }
        assert_eq!((*site.lock().unwrap()).pick_storage.have(1), 2);
        assert!(!*site.lock().unwrap().get_lock());
    }
    #[test]
    fn test_pick_render() {
        let site = Site::new();
        let cmd = SitePickCommand::new(site.clone(), (2, 233));
        assert_eq!(cmd.render().unwrap(), format!("{{ \"type\": \"pick\", \"site\": \"{}\", \"item\": 2, \"count\": 233 }}", site.lock().unwrap().id()));
    }

    #[test]
    #[should_panic(expected="not enough item")]
    fn test_put_initialize() {
        let site = Site::new();
        {
            let mut site = site.lock().unwrap();
            site.pick_storage.add(1, 3).unwrap();
        }
        let mut cmd = SitePutCommand::new(site, (1, 4));
        cmd.initialize().unwrap();
    }

    #[test]
    fn test_put_initialize_lock() {
        let site = Site::new();
        {
            let mut site = site.lock().unwrap();
            site.pick_storage.add(1, 3).unwrap();
        }
        let mut cmd = SitePutCommand::new(site.clone(), (1, 2));
        cmd.initialize().unwrap();
        assert!(*site.lock().unwrap().get_lock());
    }

    #[test]
    fn test_put_consume() {
        let site = Site::new();
        {
            let mut site = site.lock().unwrap();
            site.pick_storage.add(1, 3).unwrap();
        }
        let mut cmd = SitePutCommand::new(site.clone(), (1, 2));
        cmd.initialize().unwrap();
        cmd.consume().unwrap();
        {
            let site = site.lock().unwrap();
            assert_eq!(site.pick_storage.have(1), 1);  
        }
        assert_eq!((*site.lock().unwrap()).storage().have(1), 2);
        assert!(!*site.lock().unwrap().get_lock());
    }
    #[test]
    fn test_put_render() {
        let site = Site::new();
        let cmd = SitePutCommand::new(site.clone(), (2, 233));
        assert_eq!(cmd.render().unwrap(), format!("{{ \"type\": \"put\", \"site\": \"{}\", \"item\": 2, \"count\": 233 }}", site.lock().unwrap().id()));
    }
}
