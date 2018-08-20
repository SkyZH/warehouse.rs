mod warehouse;
mod worlddata;

use worlddata::one_bot;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let (world, bots, shelves, sites) = one_bot();
    let mut file = File::create("data/data.json")?;
    file.write_all(world.render().unwrap().as_bytes())?;
    Ok(())
}
