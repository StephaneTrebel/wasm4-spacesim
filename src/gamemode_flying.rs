extern crate alloc;
use core::f32::MAX;

use alloc::{borrow::ToOwned, vec::Vec};

use fastrand::Rng;

use crate::{
    game::Buttons,
    graphics, hud,
    maths::{distance, Coordinates},
    palette::set_draw_color,
    planet::Planet,
    player,
    wasm4::*,
};

use numtoa::NumToA;

const MAXIMUM_DISTANCE_FOR_LANDING: f32 = 100.0;
const MAXIMUM_DISTANCE_FOR_TARGETING: f32 = 5000.0;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum DirectionX {
    Left = -1,
    #[default]
    Center = 0,
    Right = 1,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum DirectionY {
    Down = 1,
    #[default]
    Center = 0,
    Up = -1,
}

#[derive(Default)]
pub struct Movement {
    pub delta_x: DirectionX,
    pub delta_y: DirectionY,
}

#[derive(Default, PartialEq, Eq)]
pub enum FlyingMode {
    #[default]
    Flying,
    LandingPossible(usize),
}

pub type PlanetTargeting = Option<usize>;

#[derive(Default)]
pub struct GameModeFlying {
    current_flying_mode: FlyingMode,
    debris: Vec<Coordinates>,
    distant_stars: Vec<Coordinates>,
    movement: Movement,
    nearest_planet_distance: f32,
    planets: Vec<Planet>,
    player_ship: player::PlayerShip,
    rng: Rng,
    targeting_planet: PlanetTargeting,
    theta: f32,
}

impl GameModeFlying {
    pub fn new() -> Self {
        let rng = Rng::with_seed(123);
        Self {
            current_flying_mode: FlyingMode::Flying,
            debris: Vec::new(),
            distant_stars: Vec::new(),
            movement: Movement {
                delta_x: DirectionX::Center,
                delta_y: DirectionY::Center,
            },
            nearest_planet_distance: 0.0,
            planets: Vec::new(),
            player_ship: player::PlayerShip::new(),
            rng,
            targeting_planet: None,
            theta: 0.01,
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
            .push(Planet::new(-300.0, -300.0, 1000.0, "Test"));
    }

    pub fn draw(&self, buttons: &Buttons) {
        set_draw_color(0x0001);

        for debris in &self.debris {
            graphics::draw_debris(debris, &self.rng);
        }

        for star in &self.distant_stars {
            graphics::draw_star(star);
        }

        for (index, planet) in self.planets.iter().enumerate() {
            graphics::draw_planet(&planet, self.targeting_planet == Some(index));
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
        if buttons.one {
            text(b"\x80", 140, 150);
        }
        if buttons.two {
            text(b"\x81", 150, 150);
            let s = self.player_ship.speed.numtoa_str(10, &mut buf);
            if buttons.up {
                text("SPD+ ".to_owned() + s, 1, 150);
            }
            if buttons.down {
                text("SPD- ".to_owned() + s, 1, 150);
            }
        }

        set_draw_color(0x0040);
        rect(0, 0, 160, 160);

        if let FlyingMode::LandingPossible(planet_index) = &self.current_flying_mode {
            let name = self.planets[*planet_index].name.clone();
            set_draw_color(0x0012);
            text(
                "LAND ON ".to_owned() + &name + " ?",
                30 - name.len() as i32,
                80,
            );
            text(b"Press \x80 to do so", 17, 90);
        }
    }

    pub fn update_movement(&mut self, buttons: &Buttons) {
        self.movement.delta_x = DirectionX::Center;
        self.movement.delta_y = DirectionY::Center;
        if !buttons.two && !buttons.one {
            if buttons.up {
                self.movement.delta_y = DirectionY::Up;
            }
            if buttons.down {
                self.movement.delta_y = DirectionY::Down;
            }
            if buttons.left {
                self.movement.delta_x = DirectionX::Left;
            }
            if buttons.right {
                self.movement.delta_x = DirectionX::Right;
            }
        }

        // if let FlyingMode::LandingPossible(planet_index) = &self.current_flying_mode {
        // if buttons.one {
        // //TODO self.current_mode = GameMode::Landed(*planet_index);
        // //TODO self.cooldown_tick = 10;
        // }
        // }
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
        let mut tmp_distance: f32;
        let mut nearest_distance: f32 = MAX;
        // let mut tmp_landing_possible_on_index: usize = 0;
        let mut tmp_targeting_planet_index: usize = 0;
        for (index, planet) in self.planets.iter_mut().enumerate() {
            planet.update(&self.movement, self.theta, self.player_ship.speed);
            tmp_distance = distance(planet.coordinates);
            if tmp_distance < nearest_distance {
                // tmp_landing_possible_on_index = index;
                tmp_targeting_planet_index = index;
                nearest_distance = tmp_distance;
            }
        }

        // TODO
        // if nearest_distance < MAXIMUM_DISTANCE_FOR_LANDING && self.cooldown_tick == 0 {
        // self.current_mode = GameMode::LandingPossible(tmp_landing_possible_on_index);
        // }
        // if nearest_distance > MAXIMUM_DISTANCE_FOR_LANDING {
        // self.current_mode = GameMode::Flying;
        // }

        if self.nearest_planet_distance < MAXIMUM_DISTANCE_FOR_TARGETING {
            self.targeting_planet = Some(tmp_targeting_planet_index);
        } else {
            self.targeting_planet = None;
        }
        self.nearest_planet_distance = nearest_distance;
    }

    pub fn update(&mut self, buttons: &Buttons) {
        self.update_movement(buttons);
        self.update_debris();
        self.update_stars();
        self.player_ship.update_speed(buttons);
        self.update_planets();
        self.draw(buttons);
    }
}
