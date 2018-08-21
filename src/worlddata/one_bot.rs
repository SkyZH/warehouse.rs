use warehouse::World;
use warehouse::object::{ Bot, Shelf, Site, Location };
use worlddata::util::{ move_location, map_as_object };
use std::sync::{ Arc, Mutex };

pub fn one_bot() -> (World, 
    Vec<Arc<Mutex<Bot>>>,
    Vec<Arc<Mutex<Shelf>>>,
    Vec<Arc<Mutex<Site>>>) {
    let (SHELF_COUNT, SHELF_WIDTH, SHELF_HEIGHT) = (6, 10, 5);
    let (MARGIN_L, MARGIN_T) = (1, 1);
    let mut world = World::new();
    let mut vec_shelf : Vec<Arc<Mutex<Shelf>>> = Vec::new();
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
        let mut site = Site::new();
        move_location(site.clone(), Location::new(cnt * 3 + MARGIN_L + 1, MARGIN_T + SHELF_WIDTH + 1, 0));
        vec_site.push(site);
    }
    world.add_items(map_as_object(&vec_shelf));
    world.add_items(map_as_object(&vec_site));
    let bot = Bot::new();
    move_location(bot.clone(), Location::new(MARGIN_L, SHELF_WIDTH + 3, 0));
    world.add_items(vec![bot.clone()]);
    (world, vec![bot], vec_shelf, vec_site)
}
