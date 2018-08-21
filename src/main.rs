mod warehouse;
mod worlddata;

use worlddata::{ Runner, OneRunner };

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut runner = OneRunner::new();
    let mut file = File::create("data/data.json")?;
    runner.tick_start(&mut file)?;
    runner.tick_and_save(&mut file)?;
    runner.tick_and_save(&mut file)?;
    runner.tick_and_save(&mut file)?;
    runner.tick_and_save(&mut file)?;
    runner.tick_end(&mut file)?;
    Ok(())
}
