use crate::{game::Buttons, maths::Coordinates, utils::clamp};

#[derive(Default, Clone, Copy)]
pub struct PlayerShip {
    pub position: Coordinates,
    pub speed: i32,
}

impl PlayerShip {
    pub fn new() -> Self {
        Self {
            position: Coordinates {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            speed: 100,
        }
    }

    pub fn update_speed(&mut self, buttons: &Buttons) {
        if buttons.two && buttons.up {
            self.speed = clamp(0, self.speed + 1, 200);
        }
        if buttons.two && buttons.down {
            self.speed = clamp(0, self.speed - 1, 200);
        }
    }
}
