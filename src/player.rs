use crate::{maths::Coordinates3d, utils::clamp};

#[derive(Default, Clone, Copy)]
pub struct PlayerShip {
    pub position: Coordinates3d,
    pub speed: i32,
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
        }
    }

    pub fn increment_speed(&mut self) {
        self.speed = clamp(0, self.speed + 1, MAX_SPEED);
    }
    pub fn decrement_speed(&mut self) {
        self.speed = clamp(0, self.speed - 1, MAX_SPEED);
    }
}
