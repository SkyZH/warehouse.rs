#[derive(Hash, Eq, PartialEq, Clone, Copy, Default)]
pub struct Location {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

impl Location {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self {
            x: x, y: y, z: z
        }
    }
    pub fn nearby(&self, target: Location) -> bool {
        (self.x as i64 - target.x as i64).abs() + (self.y as i64 - target.y as i64).abs() + (self.z as i64 - target.z as i64).abs() <= 1
    }
    pub fn render(&self) -> String {
        format!("{{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let location = Location::new(1, 2, 3);
        assert_eq!(location.render(), "{ x: 1, y: 2, z: 3 }");
    }
}
