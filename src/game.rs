use fastrand::Rng;

use crate::{
    graphics, hud,
    maths::Coordinates,
    palette::set_draw_color,
    player,
    wasm4::{self, *},
};

pub struct Game {
    rng: Rng,
    player_ship: player::PlayerShip,
    current_tick: i32,
    debris: Vec<Coordinates>,
    distant_stars: Vec<Coordinates>,
    prev_gamepad: u8,
}

fn generate_distant_stars(rng: &Rng) -> Vec<Coordinates> {
    let mut distant_stars: Vec<Coordinates> = vec![];
    for _ in 0..9 {
        distant_stars.push(Coordinates {
            x: rng.u8(0..159) as i32,
            y: rng.u8(0..159) as i32,
        });
    }
    distant_stars
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(123);
        Self {
            rng,
            player_ship: player::PlayerShip::new(),
            current_tick: 0,
            debris: vec![],
            distant_stars: vec![],
            prev_gamepad: 0,
        }
    }

    pub fn start(&mut self) {
        self.distant_stars = generate_distant_stars(&self.rng)
    }

    pub fn draw(&self) {
        set_draw_color(0x1204);

        for debris in &self.debris {
            graphics::draw_debris(debris, &self.rng);
        }

        for star in &self.distant_stars {
            graphics::draw_star(star)
        }

        blit(
            &hud::HUD,
            0,
            0,
            hud::HUD_WIDTH,
            hud::HUD_HEIGHT,
            hud::HUD_FLAGS,
        );

        text(
            format!("SPD:{}", self.player_ship.speed.to_string()),
            0,
            150,
        );
    }

    pub fn update_debris(&mut self) {
        let mut remove_indexes: Vec<usize> = vec![];

        for (index, debris) in self.debris.iter_mut().enumerate() {
            if debris.x < 80 {
                debris.x = debris.x - 1
            }
            if debris.x > 80 {
                debris.x = debris.x + 1
            }
            if debris.y < 80 {
                debris.y = debris.y - 1
            }
            if debris.y > 80 {
                debris.y = debris.y + 1
            }
            if debris.x < 0 || debris.x > 159 || debris.y < 0 || debris.y > 159 {
                remove_indexes.push(index);
            }
        }
        for index in remove_indexes {
            self.debris.remove(index);
        }

        if self.debris.len() < 10 && self.current_tick % 10 == 0 {
            self.debris.push(Coordinates {
                x: 80 + self.rng.i32(-10..10),
                y: 80 + self.rng.i32(-10..10),
            });
        }
    }

    pub fn update_stars(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        let mut move_x = 0;
        let mut move_y = 0;
        if just_pressed & wasm4::BUTTON_UP != 0 {
            move_y = 1;
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            move_y = -1;
        }
        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            move_x = -1;
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            move_x = 1;
        }

        let mut remove_indexes: Vec<usize> = vec![];

        for (index, star) in self.distant_stars.iter_mut().enumerate() {
            if move_x != 0 {
                star.x = star.x + move_x;
            }
            if move_y != 0 {
                star.y = star.y + move_y;
            }
            if star.x < 0 || star.x > 159 || star.y < 0 || star.y > 159 {
                remove_indexes.push(index);
            }
        }
        for index in remove_indexes {
            self.distant_stars.remove(index);
        }

        if self.distant_stars.len() < 10 && self.current_tick % 10 == 0 {
            self.distant_stars.push(Coordinates {
                x: self.rng.u8(0..159) as i32,
                y: self.rng.u8(0..159) as i32,
            });
        }
    }

    pub fn update(&mut self) {
        self.current_tick = self.current_tick + 1;
        self.update_debris();
        self.update_stars();
        self.draw();
    }
}
