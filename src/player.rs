use crate::maths::Coordinates;

pub struct PlayerShip {
    pub position: Coordinates,
    pub speed: i32,
}

impl PlayerShip {
    pub fn new() -> Self {
        Self {
            position: Coordinates { x: 0, y: 0, z: 0 },
            speed: 100,
        }
    }
}
