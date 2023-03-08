extern crate alloc;
use alloc::string::{ToString, String};

use crate::maths::Coordinates;

pub struct Planet {
    coordinates: Coordinates,
    name: String,
}

impl Planet {
    pub fn new(x: i32, y: i32, z: i32, name: &str) -> Self {
        Self {
            coordinates: Coordinates { x, y, z },
            name: name.to_string(),
        }
    }
}
