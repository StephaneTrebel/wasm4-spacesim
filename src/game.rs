extern crate alloc;
use alloc::{borrow::ToOwned, vec::Vec};

use fastrand::Rng;

use crate::{
    graphics, hud,
    maths::Coordinates,
    palette::set_draw_color,
    planet::Planet,
    player,
    wasm4::{self, *},
};

use numtoa::NumToA;

pub struct Buttons {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub two: bool,
    pub one: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum DirectionX {
    Left = -1,
    Center = 0,
    Right = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum DirectionY {
    Down = 1,
    Center = 0,
    Up = -1,
}

pub struct Movement {
    pub delta_x: DirectionX,
    pub delta_y: DirectionY,
}

pub struct Game {
    rng: Rng,
    theta: f32,
    player_ship: player::PlayerShip,
    current_tick: i32,
    debris: Vec<Coordinates>,
    distant_stars: Vec<Coordinates>,
    buttons: Buttons,
    movement: Movement,
    planets: Vec<Planet>,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(123);
        Self {
            rng,
            theta: 0.01,
            player_ship: player::PlayerShip::new(),
            current_tick: 0,
            debris: Vec::new(),
            distant_stars: Vec::new(),
            buttons: Buttons {
                up: false,
                down: false,
                left: false,
                right: false,
                two: false,
                one: false,
            },
            movement: Movement {
                delta_x: DirectionX::Center,
                delta_y: DirectionY::Center,
            },
            planets: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        for _ in 0..9 {
            self.distant_stars.push(Coordinates {
                x: self.rng.f32() * 159.0,
                y: self.rng.f32() * 159.0,
                z: 0.0,
                w: 1.0,
            });
        }

        for _ in 0..9 {
            self.debris.push(Coordinates {
                x: {
                    let x = 80.0 + self.rng.f32() * 40.0 - 20.0;
                    if x == 80.0 {
                        81.0
                    } else {
                        x
                    }
                },
                y: {
                    let y = 80.0 + self.rng.f32() * 40.0 - 20.0;
                    if y == 80.0 {
                        81.0
                    } else {
                        y
                    }
                },
                z: 0.0,
                w: 1.0,
            });
        }

        self.planets
            .push(Planet::new(-500.0, -500.0, 2000.0, "Test"));
    }

    pub fn draw(&self) {
        set_draw_color(0x0001);

        for debris in &self.debris {
            graphics::draw_debris(debris, &self.rng);
        }

        for star in &self.distant_stars {
            graphics::draw_star(star);
        }

        for planet in &self.planets {
            graphics::draw_planet(planet);
        }

        set_draw_color(0x0043);
        blit(
            &hud::HUD,
            -3 + self.movement.delta_x as i32 * 3,
            -3 + self.movement.delta_y as i32 * 3,
            hud::HUD_WIDTH,
            hud::HUD_HEIGHT,
            hud::HUD_FLAGS,
        );

        set_draw_color(0x0013);

        let mut buf = [0u8; 32];

        if self.buttons.one {
            text(b"\x80", 140, 150);
        }
        if self.buttons.two {
            text(b"\x81", 150, 150);
            let s = self.player_ship.speed.numtoa_str(10, &mut buf);
            if self.buttons.up {
                text("SPD+ ".to_owned() + s, 1, 150);
            }
            if self.buttons.down {
                text("SPD- ".to_owned() + s, 1, 150);
            }
        }

        set_draw_color(0x0040);
        rect(0, 0, 160, 160);
    }

    pub fn update_pressed_buttons(&mut self) {
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

    pub fn update_movement(&mut self) {
        self.movement.delta_x = DirectionX::Center;
        self.movement.delta_y = DirectionY::Center;

        if !self.buttons.two && !self.buttons.one {
            if self.buttons.up {
                self.movement.delta_y = DirectionY::Up;
            }
            if self.buttons.down {
                self.movement.delta_y = DirectionY::Down;
            }
            if self.buttons.left {
                self.movement.delta_x = DirectionX::Left;
            }
            if self.buttons.right {
                self.movement.delta_x = DirectionX::Right;
            }
        }
    }

    pub fn update_debris(&mut self) {
        let speed: f32 = (self.player_ship.speed as f32).log(10_f32);
        let delta_x = self.movement.delta_x as i32 as f32;
        let delta_y = self.movement.delta_y as i32 as f32;
        let rand = self.rng.f32() * 40.0 - 20.0;

        for (_, debris) in self.debris.iter_mut().enumerate() {
            debris.x = debris.x + delta_x * 2.0;
            debris.y = debris.y + delta_y * 2.0;
            if debris.x < 80.0 {
                debris.x = debris.x - (speed * self.rng.f32() * 2_f32);
                if debris.x <= 0.0 {
                    debris.x = 80.0 + rand - delta_x * 5.0;
                }
            }
            if debris.x > 80.0 {
                debris.x = debris.x + (speed * self.rng.f32() * 2_f32);
                if debris.x >= 159.0 {
                    debris.x = 80.0 + rand - delta_x * 5.0;
                }
            }
            if debris.y < 80.0 {
                debris.y = debris.y - (speed * self.rng.f32() * 2_f32);
                if debris.y <= 0.0 {
                    debris.y = 80.0 + rand - delta_y * 5.0;
                }
            }
            if debris.y > 80.0 {
                debris.y = debris.y + (speed * self.rng.f32() * 2_f32);
                if debris.y >= 159.0 {
                    debris.y = 80.0 + rand - delta_y * 5.0;
                }
            }
            if debris.x == 80.0 {
                debris.x = 81.0;
            }
            if debris.y == 80.0 {
                debris.y = 81.0;
            }
        }
    }

    pub fn update_stars(&mut self) {
        let mut remove_indexes: Vec<usize> = Vec::new();

        for (index, star) in self.distant_stars.iter_mut().enumerate() {
            if self.movement.delta_x != DirectionX::Center {
                star.x = star.x + self.movement.delta_x as i32 as f32;
            }
            if self.movement.delta_y != DirectionY::Center {
                star.y = star.y + self.movement.delta_y as i32 as f32;
            }
            if star.x < 0.0 || star.x > 159.0 || star.y < 0.0 || star.y > 159.0 {
                remove_indexes.push(index);
            }
        }
        for index in remove_indexes {
            self.distant_stars.remove(index);
            self.distant_stars.push(Coordinates {
                x: {
                    if self.movement.delta_x == DirectionX::Left
                        && self.movement.delta_y == DirectionY::Center
                    {
                        159.0
                    } else if self.movement.delta_x == DirectionX::Right
                        && self.movement.delta_y == DirectionY::Center
                    {
                        0.0
                    } else {
                        self.rng.f32() * 159.0
                    }
                },
                y: {
                    if self.movement.delta_y == DirectionY::Up
                        && self.movement.delta_x == DirectionX::Center
                    {
                        159.0
                    } else if self.movement.delta_y == DirectionY::Down
                        && self.movement.delta_x == DirectionX::Center
                    {
                        0.0
                    } else {
                        self.rng.f32() * 159.0
                    }
                },
                z: 0.0,
                w: 1.0,
            });
        }
    }

    pub fn update_planets(&mut self) {
        for planet in self.planets.iter_mut() {
            planet.update(&self.movement, self.theta, self.player_ship.speed);
        }
    }

    pub fn update(&mut self) {
        self.current_tick = self.current_tick + 1;
        self.update_pressed_buttons();
        self.update_movement();
        self.update_debris();
        self.update_stars();
        self.player_ship.update_speed(&self.buttons);
        self.update_planets();
        self.draw();
    }
}
