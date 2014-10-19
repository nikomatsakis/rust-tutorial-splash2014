// Theme: Shared ownership and mutation.

use std::rc::Rc;

struct Configuration {
    installed_packages: Vec<Rc<Package>>
}

struct Room {
    players: Vec<Rc<Player>>,
}

pub fn main() { }
