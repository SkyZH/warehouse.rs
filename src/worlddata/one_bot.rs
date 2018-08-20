use warehouse::World;
use warehouse::object::{ Bot, Shelf, Site, Location, Object };
use worlddata::util::move_location;
use std::sync::{ Arc, Mutex };

pub fn one_bot() -> (World, 
    Vec<Arc<Mutex<Bot>>>,
    Vec<Arc<Mutex<Shelf>>>,
    Vec<Arc<Mutex<Site>>>) {
    let (SHELF_COUNT, SHELF_WIDTH, SHELF_HEIGHT) = (6, 10, 5);
    let (MARGIN_L, MARGIN_T) = (1, 1);
    let mut world = World::new();
    let mut vec_shelf : Vec<Arc<Mutex<Shelf>>> = Vec::new();
    let mut vec_bot : Vec<Arc<Mutex<Bot>>> = Vec::new();
    let mut vec_site : Vec<Arc<Mutex<Site>>> = Vec::new();
    for cnt in 0..SHELF_COUNT {
        for col in 0..2 {
            for row in 0..SHELF_WIDTH {
                for tal in 0..SHELF_HEIGHT {
                    let mut shelf = Shelf::new();
                    move_location(shelf.clone(), Location::new(cnt * 3 + col + MARGIN_L, MARGIN_T + row, tal));
                    vec_shelf.push(shelf);
                }
            }
        }
    }
    let mut bot = Bot::new();
    move_location(bot.clone(), Location::new(MARGIN_L, SHELF_WIDTH + 2, 0));
    world.add_items(vec_shelf.iter().cloned().map(|obj| obj as Arc<Mutex<Object>>).collect());
    world.add_items(vec![bot.clone()]);
    (world, vec![bot], vec_shelf, Vec::new())
}
