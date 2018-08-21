use warehouse::World;
use warehouse::object::{ Bot, Shelf, Site, Location, Object };
use warehouse::command::{ Command, ParallelCommandQueue, BotMoveCommand, CommandQueue };
use std;
use std::sync::{ Arc, Mutex };
use std::io::Write;
use std::fs::File;
use super::one_bot;

pub trait Runner {
    fn scheduler(&self) -> Arc<Mutex<Command>>;
    fn world(&self) -> Arc<Mutex<World>>;
    fn task(&mut self) -> Result<(), &'static str>;
    fn tick(&mut self) -> Result<String, &'static str> {
        { self.task()?; }
        let scheduler = self.scheduler();
        let mut scheduler = scheduler.lock().unwrap();
        let data = scheduler.render().unwrap();
        scheduler.consume()?;
        Ok(data)
    }
    fn tick_start(&mut self, file: &mut File) -> std::io::Result<()> {
        file.write_all(b"{ \"data\": [")
    }
    fn tick_and_save(&mut self, file: &mut File) -> std::io::Result<()> {
        let scheduler_data = self.tick().unwrap();
        let world = self.world();
        let world = world.lock().unwrap();
        file.write_all(b"{ \"objects\": ")?;
        file.write_all(world.render().unwrap().as_bytes())?;
        file.write_all(b", \"scheduler\": ")?;
        file.write_all(scheduler_data.as_bytes())?;
        file.write_all(b"}, ")
    }
    fn tick_end(&mut self, file: &mut File) -> std::io::Result<()> {
        file.write_all(b"{}]}")
    }
}

pub struct OneRunner {
    world: Arc<Mutex<World>>,
    bots: Vec<Arc<Mutex<Bot>>>,
    shelves: Vec<Arc<Mutex<Shelf>>>,
    sites: Vec<Arc<Mutex<Site>>>,
    scheduler: Arc<Mutex<ParallelCommandQueue>>
}

impl OneRunner {
    pub fn new() -> Self {
        let data = one_bot();
        Self {
            world: Arc::new(Mutex::new(data.0)),
            bots: data.1,
            shelves: data.2,
            sites: data.3,
            scheduler: Arc::new(Mutex::new(ParallelCommandQueue::new()))
        }
    }
}

impl Runner for OneRunner {
    fn scheduler(&self) -> Arc<Mutex<Command>> {
        self.scheduler.clone() as Arc<Mutex<Command>>
    }
    fn world(&self) -> Arc<Mutex<World>> {
        self.world.clone()
    }
    fn task(&mut self) -> Result<(), &'static str> {
        let mut scheduler = self.scheduler.lock().unwrap();
        let bot = &self.bots[0];
        let target_location = {
            let bot = bot.lock().unwrap();
            bot.location().up()
        };
        scheduler.schedule(BotMoveCommand::new(bot.clone(), target_location, self.world.clone()))?;
        Ok(())
    }
}
