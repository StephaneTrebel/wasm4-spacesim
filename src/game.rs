use fastrand::Rng;

use crate::{
    graphics, hud,
    maths::Coordinates,
    palette::set_draw_color,
    player,
    utils::clamp,
    wasm4::{self, *},
};

pub struct Buttons {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    two: bool,
    one: bool,
}

pub struct Game {
    rng: Rng,
    player_ship: player::PlayerShip,
    current_tick: i32,
    debris: Vec<Coordinates>,
    distant_stars: Vec<Coordinates>,
    buttons: Buttons,
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
            buttons: Buttons {
                up: false,
                down: false,
                left: false,
                right: false,
                two: false,
                one: false,
            },
        }
    }

    pub fn start(&mut self) {
        self.distant_stars = vec![];
        for _ in 0..9 {
            self.distant_stars.push(Coordinates {
                x: self.rng.u8(0..159) as i32,
                y: self.rng.u8(0..159) as i32,
            });
        }
    }

    pub fn draw(&self) {
        set_draw_color(0x0001);

        for debris in &self.debris {
            graphics::draw_debris(debris, &self.rng);
        }

        for star in &self.distant_stars {
            graphics::draw_star(star)
        }

        set_draw_color(0x0043);
        blit(
            &hud::HUD,
            0,
            0,
            hud::HUD_WIDTH,
            hud::HUD_HEIGHT,
            hud::HUD_FLAGS,
        );

        set_draw_color(0x0013);
        text(
            format!("SPD:{}", self.player_ship.speed.to_string()),
            0,
            150,
        );
    }

    pub fn get_pressed_buttons(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad;
        self.buttons = Buttons {
            up: false,
            down: false,
            left: false,
            right: false,
            two: false,
            one: false,
        };
        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.buttons.up = true;
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.buttons.down = true;
        }
        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.buttons.left = true;
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.buttons.right = true;
        }
        if just_pressed & wasm4::BUTTON_1 != 0 {
            self.buttons.one = true;
        }
        if just_pressed & wasm4::BUTTON_2 != 0 {
            self.buttons.two = true;
        }
    }

    pub fn update_debris(&mut self) {
        let mut remove_indexes: Vec<usize> = vec![];
        let mut move_x = 0;
        let mut move_y = 0;

        if !self.buttons.two && !self.buttons.one && self.buttons.up {
            move_y = -1
        }
        if !self.buttons.two && !self.buttons.one && self.buttons.down {
            move_y = 1
        }
        if !self.buttons.two && !self.buttons.one && self.buttons.left {
            move_x = -1
        }
        if !self.buttons.two && !self.buttons.one && self.buttons.right {
            move_x = 1
        }

        for (index, debris) in self.debris.iter_mut().enumerate() {
            debris.x = debris.x + move_x;
            debris.y = debris.y + move_y;
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
                x: 80 + self.rng.i32(-20..20) + 1,
                y: 80 + self.rng.i32(-20..20) + 1,
            });
        }
    }

    pub fn update_stars(&mut self) {
        let mut move_x = 0;
        let mut move_y = 0;

        if !self.buttons.two && !self.buttons.one && self.buttons.up {
            move_y = -1
        }
        if !self.buttons.two && !self.buttons.one && self.buttons.down {
            move_y = 1
        }
        if !self.buttons.two && !self.buttons.one && self.buttons.left {
            move_x = -1
        }
        if !self.buttons.two && !self.buttons.one && self.buttons.right {
            move_x = 1
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
            self.distant_stars.push(Coordinates {
                x: {
                    if move_x == -1 && move_y == 0 {
                        159
                    } else if move_x == 1 && move_y == 0 {
                        0
                    } else {
                        self.rng.u8(0..159)
                    }
                } as i32,
                y: {
                    if move_y == -1 && move_x == 0 {
                        159
                    } else if move_y == 1 && move_x == 0 {
                        0
                    } else {
                        self.rng.u8(0..159)
                    }
                } as i32,
            });
        }
    }

    pub fn update_speed(&mut self) {
        if self.buttons.two && self.buttons.up {
            self.player_ship.speed = clamp(0, self.player_ship.speed + 1, 150);
        }
        if self.buttons.two && self.buttons.down {
            self.player_ship.speed = clamp(0, self.player_ship.speed - 1, 150);
        }
    }

    pub fn update(&mut self) {
        self.current_tick = self.current_tick + 1;
        self.get_pressed_buttons();
        self.update_debris();
        self.update_stars();
        self.update_speed();
        self.draw();
    }
}
