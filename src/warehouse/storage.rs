type Item = u64;

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
        let mut items_str = Vec::new() as Vec<String>;
        for i in &self.items {
            items_str.push([
                "{ item: ", &*i.0.to_string(), ", ",
                "count: ", &*i.1.to_string(), " }"
            ].join(""));
        }
        Ok(["[", &*items_str.join(", "), "]"].join(""))
    }
    pub fn items(&self) -> &Vec<(Item, u32)> {
        &self.items
    }
    pub fn add(&mut self, item: Item, count: u32) {
        self.items.push((item, count));
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;

    #[test]
    fn test_render() {
        let mut storage = Storage::new();
        storage.add(1, 1);
        assert_eq!(storage.render().unwrap(), "[{ item: 1, count: 1 }]");
    }

    #[test]
    fn test_render_multiple() {
        let mut storage = Storage::new();
        storage.add(1, 1);
        storage.add(2, 2);
        assert_eq!(storage.render().unwrap(), "[{ item: 1, count: 1 }, { item: 2, count: 2 }]");
    }
}
