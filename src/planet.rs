extern crate alloc;
use alloc::string::{String, ToString};

use crate::maths::Coordinates;

pub struct Planet {
    pub coordinates: Coordinates,
    pub name: String,
}

impl Planet {
    pub fn new(x: f32, y: f32, z: f32, name: &str) -> Self {
        Self {
            coordinates: Coordinates { x, y, z, w: 1.0 },
            name: name.to_string(),
        }
    }
}

