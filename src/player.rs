use core::i32::MAX;

use hashbrown::HashMap;

use crate::{items::Item, maths::Coordinates3d, utils::clamp};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Inventory {
    quantity: u32,
}

#[derive(Default, Clone)]
pub struct PlayerShip {
    pub position: Coordinates3d,
    pub speed: i32,
    pub inventory: HashMap<Item, Inventory>,
    pub money: i32,
}

const MAX_SPEED: i32 = 500;

impl PlayerShip {
    pub fn new() -> Self {
        Self {
            position: Coordinates3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            speed: 100,
            inventory: HashMap::new(),
            money: 1000,
        }
    }

    pub fn increment_speed(&mut self) {
        self.speed = clamp(0, self.speed + 1, MAX_SPEED);
    }

    pub fn decrement_speed(&mut self) {
        self.speed = clamp(0, self.speed - 1, MAX_SPEED);
    }

    pub fn increment_money(&mut self) {
        self.money = clamp(0, self.money + 1, MAX);
    }

    pub fn decrement_money(&mut self) {
        self.money = clamp(0, self.money - 1, MAX);
    }

    /// Add item to player ship inventory
    /// If the item already exist it is overwritten and the previous value is returned
    pub fn add_item_to_inventory(&mut self, item: &Item, quantity: u32) -> Option<Inventory> {
        self.inventory.insert(item.clone(), Inventory { quantity })
    }

    /// Remove item from player ship inventory
    /// If the item exist it is removed and the previous value is returned
    pub fn remove_item_from_inventory(&mut self, item: &Item) -> Option<Inventory> {
        self.inventory.remove(item)
    }
}
