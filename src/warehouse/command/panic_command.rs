use super::command::Command;

pub struct PanicCommand {
}

impl PanicCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for PanicCommand {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Err("panic command initialized")
    }
    fn consume(&mut self) -> Result<bool, &'static str> {
        Err("panic command consumed")
    }
}

pub struct ConsumePanicCommand {
}

impl ConsumePanicCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ConsumePanicCommand {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
    fn consume(&mut self) -> Result<bool, &'static str> {
        Err("consume panic command consumed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_initialize() {
        let mut cmd = PanicCommand::new();
        cmd.initialize().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_consume() {
        let mut cmd = PanicCommand::new();
        cmd.consume().unwrap();
    }
    
    #[test]
    #[should_panic]
    fn test_consume_consume() {
        let mut cmd = ConsumePanicCommand::new();
        cmd.consume().unwrap();
    }
}
