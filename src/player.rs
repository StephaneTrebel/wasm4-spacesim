use core::i32::MAX;

use hashbrown::HashMap;

use crate::{items::Item, maths::Coordinates3d, utils::clamp};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PlayerInventory {
    quantity: u32,
}

#[derive(PartialEq, Default, Clone)]
pub struct PlayerShip {
    pub position: Coordinates3d,
    pub speed: u32,
    pub inventory: HashMap<Item, PlayerInventory>,
    pub money: u32,
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
        self.speed = clamp(0, (self.speed + 1) as i32, MAX_SPEED) as u32;
    }

    pub fn decrement_speed(&mut self) {
        self.speed = clamp(0, (self.speed - 1) as i32, MAX_SPEED) as u32;
    }

    pub fn increment_money(&mut self, amount: u32) {
        self.money = clamp(0, (self.money + amount) as i32, MAX) as u32;
    }

    pub fn decrement_money(&mut self, amount: u32) {
        self.money = clamp(0, (self.money - amount) as i32, MAX) as u32;
    }
}

pub enum BuyingError {
    QuantityIsZero,
    NotEnoughMoney,
}

impl PlayerShip {
    /// Buy stuff (from the player perspective)
    pub fn can_buy(&mut self, quantity: u32, buying_price: u32) -> Result<(), BuyingError> {
        if quantity == 0 {
            return Err(BuyingError::QuantityIsZero);
        }

        let total = quantity * buying_price;
        if self.money < total {
            return Err(BuyingError::NotEnoughMoney);
        }
        Ok(())
    }
    pub fn buy(&mut self, item: &Item, quantity: u32, buying_price: u32) {
        let total = quantity * buying_price;
        let previous_inventory = self.inventory.get(item);

        let new_quantity = match previous_inventory {
            None => quantity,
            Some(inventory) => inventory.quantity + quantity,
        };

        self.decrement_money(total);
        self.inventory.insert(
            item.clone(),
            PlayerInventory {
                quantity: new_quantity,
            },
        );
    }
}

pub enum SellingError {
    QuantityIsZero,
    NotEnoughToSell,
}

impl PlayerShip {
    /// Sell stuff (from the player perspective)
    pub fn can_sell(&mut self, item: &Item, quantity: u32) -> Result<(), SellingError> {
        if quantity == 0 {
            return Err(SellingError::QuantityIsZero);
        }
        let previous_inventory = self.inventory.get(item);

        let new_quantity: i32 = match previous_inventory {
            None => quantity as i32,
            Some(inventory) => inventory.quantity as i32 - quantity as i32,
        };

        if new_quantity < 0 {
            return Err(SellingError::NotEnoughToSell);
        }
        Ok(())
    }
    pub fn sell(&mut self, item: &Item, quantity: u32, selling_price: u32) {
        let previous_inventory = self.inventory.get(item);

        let new_quantity: i32 = match previous_inventory {
            // Should not happen, TODO force an error
            None => 0,
            Some(inventory) => inventory.quantity as i32 - quantity as i32,
        };

        self.increment_money(quantity * selling_price);
        // TODO Remove entry if new_quantity=0
        self.inventory.insert(
            item.clone(),
            PlayerInventory {
                quantity: new_quantity as u32,
            },
        );
    }
}
