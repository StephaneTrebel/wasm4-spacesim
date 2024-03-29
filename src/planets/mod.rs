extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use hashbrown::HashMap;

use crate::items::Item;
use crate::maths::{distance, rotate_xz, rotate_yz, Coordinates3d};

pub mod planet_hud;

pub mod planet_a;
pub mod planet_b;

#[derive(Clone, PartialEq)]
pub enum Level {
    ONE,
    TWO,
    THREE,
    FOUR,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    A,
    B,
}

impl Type {
    pub fn get_level(&self, distance: f32) -> Level {
        match distance.floor() as u32 {
            0..=1000 => Level::ONE,
            1001..=2000 => Level::TWO,
            2001..=3000 => Level::THREE,
            _ => Level::FOUR,
        }
    }

    pub fn get_sprite(&self, level: &Level) -> Box<[u8]> {
        match self {
            Type::A => match level {
                Level::ONE => Box::new(planet_a::planet_sprite_120::PLANET_120),
                Level::TWO => Box::new(planet_a::planet_sprite_80::PLANET_80),
                Level::THREE => Box::new(planet_a::planet_sprite_40::PLANET_40),
                Level::FOUR => Box::new(planet_a::planet_sprite_12::PLANET_12),
            },
            Type::B => match level {
                Level::ONE => Box::new(planet_b::planet_sprite_120::PLANET_120),
                Level::TWO => Box::new(planet_b::planet_sprite_80::PLANET_80),
                Level::THREE => Box::new(planet_b::planet_sprite_40::PLANET_40),
                Level::FOUR => Box::new(planet_b::planet_sprite_12::PLANET_12),
            },
        }
    }

    pub fn get_width(&self, level: &Level) -> u32 {
        match self {
            Type::A => match level {
                Level::ONE => planet_a::planet_sprite_120::PLANET_120_WIDTH,
                Level::TWO => planet_a::planet_sprite_80::PLANET_80_WIDTH,
                Level::THREE => planet_a::planet_sprite_40::PLANET_40_WIDTH,
                Level::FOUR => planet_a::planet_sprite_12::PLANET_12_WIDTH,
            },
            Type::B => match level {
                Level::ONE => planet_b::planet_sprite_120::PLANET_120_WIDTH,
                Level::TWO => planet_b::planet_sprite_80::PLANET_80_WIDTH,
                Level::THREE => planet_b::planet_sprite_40::PLANET_40_WIDTH,
                Level::FOUR => planet_b::planet_sprite_12::PLANET_12_WIDTH,
            },
        }
    }

    pub fn get_height(&self, level: &Level) -> u32 {
        match self {
            Type::A => match level {
                Level::ONE => planet_a::planet_sprite_120::PLANET_120_HEIGHT,
                Level::TWO => planet_a::planet_sprite_80::PLANET_80_HEIGHT,
                Level::THREE => planet_a::planet_sprite_40::PLANET_40_HEIGHT,
                Level::FOUR => planet_a::planet_sprite_12::PLANET_12_HEIGHT,
            },
            Type::B => match level {
                Level::ONE => planet_b::planet_sprite_120::PLANET_120_HEIGHT,
                Level::TWO => planet_b::planet_sprite_80::PLANET_80_HEIGHT,
                Level::THREE => planet_b::planet_sprite_40::PLANET_40_HEIGHT,
                Level::FOUR => planet_b::planet_sprite_12::PLANET_12_HEIGHT,
            },
        }
    }

    pub fn get_flags(&self, level: &Level) -> u32 {
        match self {
            Type::A => match level {
                Level::ONE => planet_a::planet_sprite_120::PLANET_120_FLAGS,
                Level::TWO => planet_a::planet_sprite_80::PLANET_80_FLAGS,
                Level::THREE => planet_a::planet_sprite_40::PLANET_40_FLAGS,
                Level::FOUR => planet_a::planet_sprite_12::PLANET_12_FLAGS,
            },
            Type::B => match level {
                Level::ONE => planet_b::planet_sprite_120::PLANET_120_FLAGS,
                Level::TWO => planet_b::planet_sprite_80::PLANET_80_FLAGS,
                Level::THREE => planet_b::planet_sprite_40::PLANET_40_FLAGS,
                Level::FOUR => planet_b::planet_sprite_12::PLANET_12_FLAGS,
            },
        }
    }

    pub fn get_colors(&self, level: &Level) -> u16 {
        match self {
            Type::A => match level {
                _ => 0x0234,
            },
            Type::B => match level {
                _ => 0x0234,
            },
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct PlanetItemInventory {
    pub quantity: u32,
    pub buying_price: u32,
    pub selling_price: u32,
}

impl PlanetItemInventory {
    pub fn new(quantity: u32, buying_price: u32, selling_price: u32) -> Self {
        Self {
            quantity,
            buying_price,
            selling_price,
        }
    }
}

pub enum BuyingError {
    ItemCannotBeBoughtHere,
    QuantityIsZero,
}

pub enum SellingError {
    ItemNotInInventory,
    QuantityIsZero,
}

#[derive(Clone, PartialEq)]
pub struct Planet {
    pub coordinates: Coordinates3d,
    pub name: String,
    pub distance: f32,
    pub planet_type: Type,
    pub inventory: HashMap<Item, PlanetItemInventory>,
}

impl Planet {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        name: &str,
        planet_type: Type,
        inventory: HashMap<Item, PlanetItemInventory>,
    ) -> Self {
        let coords = Coordinates3d { x, y, z, w: 1.0 };
        Self {
            coordinates: coords,
            name: name.to_string(),
            distance: distance(coords),
            planet_type,
            inventory,
        }
    }

    // it's not the player that
    // rotates, it's the universe
    pub fn rotate_xz(&mut self, theta: f32) {
        self.coordinates = rotate_xz(self.coordinates, theta);
    }

    pub fn rotate_yz(&mut self, theta: f32) {
        self.coordinates = rotate_yz(self.coordinates, theta);
    }

    pub fn update(&mut self, theta_xz: f32, theta_yz: f32, player_speed: i32) {
        if theta_xz != 0.0 {
            self.rotate_xz(theta_xz);
        }

        if theta_yz != 0.0 {
            self.rotate_yz(theta_yz);
        }

        self.coordinates.z -= player_speed as f32 / 1000.0;
        self.distance = distance(self.coordinates);
    }

    /// Buy stuff (from the planet perspective)
    pub fn can_buy(&mut self, item: &Item, quantity: u32) -> Result<(), BuyingError> {
        if quantity == 0 {
            return Err(BuyingError::QuantityIsZero);
        }
        match self.inventory.get(item) {
            None => Err(BuyingError::ItemCannotBeBoughtHere),
            Some(_inventory) => Ok(()),
        }
    }
    pub fn buy(&mut self, item: &Item, quantity: u32) {
        match self.inventory.get_mut(item) {
            None => {}
            Some(inventory) => {
                let old_quantity = {
                    if inventory.quantity == 0 {
                        1
                    } else {
                        inventory.quantity
                    }
                };
                inventory.quantity += quantity;
                inventory.buying_price += inventory.buying_price * quantity / old_quantity;
                inventory.selling_price -= inventory.selling_price * quantity / old_quantity;
            }
        }
    }

    /// Sell stuff (from the planet perspective)
    pub fn can_sell(&mut self, item: &Item, quantity: u32) -> Result<(), SellingError> {
        if quantity == 0 {
            return Err(SellingError::QuantityIsZero);
        }
        match self.inventory.get(item) {
            None => Err(SellingError::ItemNotInInventory),
            Some(_) => Ok(()),
        }
    }
    pub fn sell(&mut self, item: &Item, quantity: u32) {
        match self.inventory.get_mut(item) {
            None => {}
            Some(inventory) => {
                let old_quantity = {
                    if inventory.quantity == 0 {
                        1
                    } else {
                        inventory.quantity
                    }
                };
                inventory.quantity -= quantity;
                inventory.buying_price -= inventory.buying_price * quantity / old_quantity;
                inventory.selling_price += inventory.selling_price * quantity / old_quantity;
            }
        }
    }
}

pub type Planets = HashMap<String, Planet>;
