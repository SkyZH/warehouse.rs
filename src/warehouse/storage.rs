type Item = u64;
type ItemSet = (Item, u32);
pub struct Storage {
    pub items: Vec<(Item, u32)>
}

impl Storage {
    pub fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }
    pub fn render(&self) -> Result<String, &'static str> {
        let result = self.items.iter()
            .map(|set: &ItemSet| format!("{{ item: {}, count: {} }}", set.0, set.1))
            .collect::<Vec<String>>()
            .join(", ");
        Ok(format!("[{}]", result))
    }
    pub fn items(&self) -> &Vec<(Item, u32)> {
        &self.items
    }
    pub fn add(&mut self, item: Item, count: u32) -> Result<(), u32> {
        {
            let mut iter = self.items.iter_mut();
            match iter.find(|&& mut(c_item, _)| c_item == item) {
                Some((_, cnt)) => {
                    *cnt += count;
                    return Ok(())
                },
                None => {}
            }
        }
        self.items.push((item, count));
        Ok(())
    }

    pub fn take(&mut self, item: Item, count: u32) -> Result<(), u32> {
        let mut result: Result<(), u32> = Err(0);
        {
            let mut iter = self.items.iter_mut();
            result = match iter.find(|&& mut(c_item, _)| c_item == item) {
                Some((_, cnt)) => {
                    if *cnt < count {
                        Err(*cnt)
                    } else {
                        *cnt -= count;
                        Ok(())
                    }
                },
                None => Err(0)
            }
        }
        self.items.retain(|(_, cnt)| *cnt > 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;

    #[test]
    fn test_render() {
        let mut storage = Storage::new();
        storage.add(1, 1).unwrap();
        assert_eq!(storage.render().unwrap(), "[{ item: 1, count: 1 }]");
    }

    #[test]
    fn test_render_multiple() {
        let mut storage = Storage::new();
        storage.add(1, 1).unwrap();
        storage.add(2, 2).unwrap();
        assert_eq!(storage.render().unwrap(), "[{ item: 1, count: 1 }, { item: 2, count: 2 }]");
    }
    #[test]
    fn test_push_multiple() {
        let mut storage = Storage::new();
        storage.add(1, 1).unwrap();
        storage.add(1, 1).unwrap();
        assert_eq!(storage.render().unwrap(), "[{ item: 1, count: 2 }]");
    }
    #[test]
    #[should_panic(expected="3")]
    fn test_take_multiple() {
        let mut storage = Storage::new();
        storage.add(1, 3).unwrap();
        storage.take(1, 4).unwrap();
    }
    #[test]
    fn test_take_empty() {
        let mut storage = Storage::new();
        storage.add(1, 3).unwrap();
        storage.take(1, 3).unwrap();
        assert_eq!(storage.items.len(), 0);
    }
}
