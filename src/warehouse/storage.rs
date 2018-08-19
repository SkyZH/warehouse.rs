type Item = u64;

pub struct Storage {
    pub items: Vec<Item>
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
            items_str.push(i.to_string());
        }
        Ok(["[", &*items_str.join(", "), "]"].join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;

    #[test]
    fn test_render() {
        let mut storage = Storage::new();
        storage.items.push(1);
        assert_eq!(storage.render().unwrap(), "[1]");
    }

    #[test]
    fn test_render_multiple() {
        let mut storage = Storage::new();
        storage.items.push(1);
        storage.items.push(2);
        assert_eq!(storage.render().unwrap(), "[1, 2]");
    }
}
