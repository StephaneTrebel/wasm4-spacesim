extern crate alloc;
use alloc::string::{String, ToString};

use crate::{
    gamemode_flying::{DirectionX, DirectionY, Movement},
    maths::{distance, rotate_xz, rotate_yz, Coordinates},
};

#[derive(Clone)]
pub struct Planet {
    pub coordinates: Coordinates,
    pub name: String,
    pub distance: f32,
}

impl Planet {
    pub fn new(x: f32, y: f32, z: f32, name: &str) -> Self {
        let coords = Coordinates { x, y, z, w: 1.0 };
        Self {
            coordinates: coords,
            name: name.to_string(),
            distance: distance(coords),
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

    pub fn update(&mut self, movement: &Movement, theta: f32, player_speed: i32) {
        if movement.delta_y == DirectionY::Up {
            self.rotate_yz(theta);
        }
        if movement.delta_y == DirectionY::Down {
            self.rotate_yz(-theta);
        }
        if movement.delta_x == DirectionX::Left {
            self.rotate_xz(-theta);
        }
        if movement.delta_x == DirectionX::Right {
            self.rotate_xz(theta);
        }
        self.coordinates.z -= player_speed as f32 / 1000.0;
        self.distance = distance(self.coordinates);
    }
}
