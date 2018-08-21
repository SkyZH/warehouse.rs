use warehouse::command::{ Command, CommandQueue };

pub struct ParallelCommandQueue {
    queue: Vec<Box<Command>>
}

impl ParallelCommandQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new()
        }
    }
}

impl CommandQueue for ParallelCommandQueue {
    fn schedule(&mut self, command: Box<Command>) -> Result<(), &'static str> {
        let mut command = command;
        match command.initialize() {
            Ok(_) => {
                self.queue.push(command);
                Ok(())
            },
            Err(err) => Err(err)
        }
    }
    fn commands(&self) -> &Vec<Box<Command>> {
        &self.queue
    }
}

impl Command for ParallelCommandQueue {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn consume(&mut self) -> Result<bool, &'static str> {
        let mut _queue = Vec::new() as Vec<bool>;
        for command in &mut self.queue {
            match command.consume() {
                Ok(next) => _queue.push(next),
                Err(err) => return Err(err)
            }
        }
        let mut idx = 0;
        self.queue.retain(|_command| {
            idx += 1;
            _queue[idx - 1]
        });
        Ok(self.queue.len() > 0)
    }
    fn render(&self) -> Result<String, &'static str> {
        let mut error_flag: Option<&'static str> = None;
        let result = self.queue.iter()
            .map(|command: &Box<Command>| match command.render() {
                Ok(result) => result,
                Err(err) => {
                    error_flag = Some(err);
                    "".to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        match error_flag {
            Some(err) => Err(err),
            None => Ok(format!("{{ \"type\": \"parallel_queue\", \"commands\": [{}] }}", result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warehouse::command::TestCommand;
    use warehouse::command::PanicCommand;
    use warehouse::command::ConsumePanicCommand;
    use warehouse::command::TestNextCommand;

    #[test]
    fn test_new() {
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
    }

    #[test]
    fn test_schedule_command() {
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(Box::new(TestCommand::new())).unwrap();
    }

    #[test]
    fn test_schedule_command_initialize() {
        let mut queue = ParallelCommandQueue::new();
        let cmd = TestCommand::new();
        let dat = cmd.data.clone();
        queue.initialize().unwrap();
        queue.schedule(Box::new(cmd)).unwrap();
        assert!(dat.lock().unwrap().initialized);
    }

    #[test]
    #[should_panic(expected="panic command initialized")]
    fn test_schedule_panic() {
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(Box::new(PanicCommand::new())).unwrap();
    } 

    #[test]
    fn test_consume_command() {
        let mut queue = ParallelCommandQueue::new();
        let cmd = TestCommand::new();
        let dat = cmd.data.clone();
        queue.initialize().unwrap();
        queue.schedule(Box::new(cmd)).unwrap();
        queue.consume().unwrap();
        assert!(dat.lock().unwrap().consumed);
    }

    #[test]
    #[should_panic(expected="consume panic command consumed")]
    fn test_consume_panic() {
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(Box::new(ConsumePanicCommand::new())).unwrap();
        queue.consume().unwrap();
    }


    #[test]
    fn test_consume_parallel_command() {
        let mut queue = ParallelCommandQueue::new();
        let cmd1 = TestCommand::new();
        let cmd2 = TestCommand::new();
        let dat1 = cmd1.data.clone();
        let dat2 = cmd2.data.clone();
        queue.initialize().unwrap();
        queue.schedule(Box::new(cmd1)).unwrap();
        queue.schedule(Box::new(cmd2)).unwrap();
        queue.consume().unwrap();
        assert!(dat1.lock().unwrap().consumed);
        assert!(dat2.lock().unwrap().consumed);
    }

    #[test]
    fn test_remove_consumed() {
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(Box::new(TestCommand::new())).unwrap();
        queue.consume().unwrap();
        assert!(queue.queue.len() == 0);
    } 


    #[test]
    fn test_retain_next() {
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(Box::new(TestNextCommand::new())).unwrap();
        queue.consume().unwrap();
        assert!(queue.queue.len() == 1);
    }

    #[test]
    fn test_render() {
        let mut queue = ParallelCommandQueue::new();
        queue.initialize().unwrap();
        queue.schedule(Box::new(TestNextCommand::new())).unwrap();
        queue.schedule(Box::new(TestNextCommand::new())).unwrap();
        assert_eq!(queue.render().unwrap(), "{ \"type\": \"parallel_queue\", \"commands\": [{ \"type\": \"testnext\" }, { \"type\": \"testnext\" }] }");
    }
}
