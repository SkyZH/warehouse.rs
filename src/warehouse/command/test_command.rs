use super::command::Command;
use std::sync::{ Arc, Mutex };

pub struct TestCommandStatus {
    pub consumed: bool,
    pub initialized: bool
}

pub struct TestCommand {
    pub data: Arc<Mutex<TestCommandStatus>>
}

impl TestCommand {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(TestCommandStatus {
                consumed: false,
                initialized: false
            }))
        }
    }
}

impl Command for TestCommand {
    fn initialize(&mut self) -> Result<(), &'static str> {
        (*self.data.lock().unwrap()).initialized = true;
        Ok(())
    }
    fn consume(&mut self) -> Result<bool, &'static str> {
        (*self.data.lock().unwrap()).consumed = true;
        Ok(false)
    }
    fn render(&self) -> Result<String, &'static str> {
        Ok("{ \"type\": \"test\" }".to_owned())
    }
}

pub struct TestNextCommand {
}

impl TestNextCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for TestNextCommand {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
    fn consume(&mut self) -> Result<bool, &'static str> {
        Ok(true)
    }
    fn render(&self) -> Result<String, &'static str> {
        Ok("{ \"type\": \"testnext\" }".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let mut cmd = TestCommand::new();
        match cmd.initialize() {
            Ok(r) => {
                assert_eq!(r, ());
                assert!(cmd.data.lock().unwrap().initialized);
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_consume() {
        let mut cmd = TestCommand::new();
        cmd.initialize().unwrap();
        match cmd.consume() {
            Ok(r) => {
                assert_eq!(r, false);
                assert!(cmd.data.lock().unwrap().consumed);
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_next_consume() {
        let mut cmd = TestNextCommand::new();
        cmd.initialize().unwrap();
        assert!(cmd.consume().unwrap());
    }
}
