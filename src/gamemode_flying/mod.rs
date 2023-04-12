use core::f32::MAX;

extern crate alloc;
use alloc::{borrow::ToOwned, vec::Vec};

use fastrand::Rng;

use crate::{
    buttons::Buttons,
    graphics::{draw_debris, draw_planet, draw_star, draw_targeting},
    maths::{distance, Coordinates3d},
    palette::set_draw_color,
    planets::{self, Planet},
    player::PlayerShip,
    wasm4::*,
};

use numtoa::NumToA;

mod hud;

const MAXIMUM_DISTANCE_FOR_LANDING: f32 = 1000.0;
const MAXIMUM_DISTANCE_FOR_TARGETING: f32 = 5000.0;

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

#[derive(Clone, PartialEq)]
pub enum FlyingMode {
    Flying,
    LandingPossible(Planet),
}

pub type PlanetTargeting = Option<usize>;

fn update_movement(mode: &mut GameModeFlying, buttons: &Buttons) -> Option<Planet> {
    mode.movement.delta_x = DirectionX::Center;
    mode.movement.delta_y = DirectionY::Center;
    if !buttons.two && !buttons.one {
        if buttons.up {
            mode.movement.delta_y = DirectionY::Up;
        }
        if buttons.down {
            mode.movement.delta_y = DirectionY::Down;
        }
        if buttons.left {
            mode.movement.delta_x = DirectionX::Left;
        }
        if buttons.right {
            mode.movement.delta_x = DirectionX::Right;
        }
    }
    if let FlyingMode::LandingPossible(planet) = &mode.current_flying_mode {
        Some(planet.clone())
    } else {
        None
    }
}

fn update_debris(mode: &mut GameModeFlying) {
    let speed: f32 = (mode.player_ship.speed as f32).log(10_f32);
    let delta_x = mode.movement.delta_x as i32 as f32;
    let delta_y = mode.movement.delta_y as i32 as f32;
    let rand = mode.rng.f32() * 40.0 - 20.0;

    for (_, debris) in mode.debris.iter_mut().enumerate() {
        debris.x = debris.x + delta_x * 2.0;
        debris.y = debris.y + delta_y * 2.0;
        if debris.x < 80.0 {
            debris.x = debris.x - (speed * mode.rng.f32() * 2_f32);
            if debris.x <= 0.0 {
                debris.x = 80.0 + rand - delta_x * 5.0;
            }
        }
        if debris.x > 80.0 {
            debris.x = debris.x + (speed * mode.rng.f32() * 2_f32);
            if debris.x >= 159.0 {
                debris.x = 80.0 + rand - delta_x * 5.0;
            }
        }
        if debris.y < 80.0 {
            debris.y = debris.y - (speed * mode.rng.f32() * 2_f32);
            if debris.y <= 0.0 {
                debris.y = 80.0 + rand - delta_y * 5.0;
            }
        }
        if debris.y > 80.0 {
            debris.y = debris.y + (speed * mode.rng.f32() * 2_f32);
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

fn update_stars(mode: &mut GameModeFlying) {
    let mut remove_indexes: Vec<usize> = Vec::new();

    for (index, star) in mode.distant_stars.iter_mut().enumerate() {
        if mode.movement.delta_x != DirectionX::Center {
            star.x = star.x + mode.movement.delta_x as i32 as f32;
        }
        if mode.movement.delta_y != DirectionY::Center {
            star.y = star.y + mode.movement.delta_y as i32 as f32;
        }
        if star.x < 0.0 || star.x > 159.0 || star.y < 0.0 || star.y > 159.0 {
            remove_indexes.push(index);
        }
    }
    for index in remove_indexes {
        mode.distant_stars.remove(index);
        mode.distant_stars.push(Coordinates3d {
            x: {
                if mode.movement.delta_x == DirectionX::Left
                    && mode.movement.delta_y == DirectionY::Center
                {
                    159.0
                } else if mode.movement.delta_x == DirectionX::Right
                    && mode.movement.delta_y == DirectionY::Center
                {
                    0.0
                } else {
                    mode.rng.f32() * 159.0
                }
            },
            y: {
                if mode.movement.delta_y == DirectionY::Up
                    && mode.movement.delta_x == DirectionX::Center
                {
                    159.0
                } else if mode.movement.delta_y == DirectionY::Down
                    && mode.movement.delta_x == DirectionX::Center
                {
                    0.0
                } else {
                    mode.rng.f32() * 159.0
                }
            },
            z: 0.0,
            w: 1.0,
        });
    }
}

fn draw(mode: &GameModeFlying, buttons: &Buttons) {
    set_draw_color(0x0001);

    for star in &mode.distant_stars {
        draw_star(star);
    }

    for (index, planet) in mode.planets.iter().enumerate() {
        draw_planet(&planet);
        if mode.targeting_planet == Some(index) {
            draw_targeting(planet);
        }
    }

    set_draw_color(0x0043);
    blit(
        &hud::HUD,
        -3 + mode.movement.delta_x as i32 * 3,
        -3 + mode.movement.delta_y as i32 * 3,
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
        let s = mode.player_ship.speed.numtoa_str(10, &mut buf);
        if buttons.up {
            text("SPD+ ".to_owned() + s, 1, 150);
        }
        if buttons.down {
            text("SPD- ".to_owned() + s, 1, 150);
        }
    }

    set_draw_color(0x0001);
    for debris in &mode.debris {
        draw_debris(debris, &mode.rng);
    }

    set_draw_color(0x0040);
    rect(0, 0, 160, 160);

    if let FlyingMode::LandingPossible(planet) = &mode.current_flying_mode {
        let name = planet.name.clone();
        set_draw_color(0x0012);
        text(
            "LAND ON ".to_owned() + &name + " ?",
            20 - name.len() as i32,
            80,
        );
        text(b"Press \x80 to do so", 17, 90);
    }
}

fn update_planets(mode: &mut GameModeFlying, cooldown_tick: i32) {
    let mut tmp_distance: f32;
    let mut nearest_distance: f32 = MAX;
    let mut tmp_planet_landing_possible: Option<&Planet> = None;
    let mut tmp_targeting_planet_index: usize = 0;

    let theta_xz = {
        if mode.movement.delta_x == DirectionX::Left {
            -mode.theta
        } else if mode.movement.delta_x == DirectionX::Right {
            mode.theta
        } else {
            0.0
        }
    };

    let theta_yz = {
        if mode.movement.delta_y == DirectionY::Up {
            mode.theta
        } else if mode.movement.delta_y == DirectionY::Down {
            -mode.theta
        } else {
            0.0
        }
    };

    for (index, planet) in mode.planets.iter_mut().enumerate() {
        planet.update(theta_xz, theta_yz, mode.player_ship.speed);
        tmp_distance = distance(planet.coordinates);
        if tmp_distance < nearest_distance {
            tmp_planet_landing_possible = Some(planet);
            tmp_targeting_planet_index = index;
            nearest_distance = tmp_distance;
        }
    }

    // TODO
    if nearest_distance < MAXIMUM_DISTANCE_FOR_LANDING && cooldown_tick == 0 {
        if let Some(planet) = tmp_planet_landing_possible {
            mode.current_flying_mode = FlyingMode::LandingPossible(planet.clone());
        }
    }
    if nearest_distance > MAXIMUM_DISTANCE_FOR_LANDING {
        mode.current_flying_mode = FlyingMode::Flying;
    }

    if mode.nearest_planet_distance < MAXIMUM_DISTANCE_FOR_TARGETING {
        mode.targeting_planet = Some(tmp_targeting_planet_index);
    } else {
        mode.targeting_planet = None;
    }
    mode.nearest_planet_distance = nearest_distance;
}

pub struct GameModeFlying {
    current_flying_mode: FlyingMode,
    debris: Vec<Coordinates3d>,
    distant_stars: Vec<Coordinates3d>,
    movement: Movement,
    nearest_planet_distance: f32,
    planets: Vec<Planet>,
    player_ship: PlayerShip,
    rng: Rng,
    targeting_planet: PlanetTargeting,
    theta: f32,
}

impl GameModeFlying {
    pub fn new() -> Self {
        let rng = Rng::with_seed(123);
        let mut new_instance = Self {
            current_flying_mode: FlyingMode::Flying,
            debris: Vec::new(),
            distant_stars: Vec::new(),
            movement: Movement {
                delta_x: DirectionX::Center,
                delta_y: DirectionY::Center,
            },
            nearest_planet_distance: 0.0,
            planets: Vec::new(),
            player_ship: PlayerShip::new(),
            rng,
            targeting_planet: None,
            theta: 0.01,
        };

        for _ in 0..9 {
            new_instance.distant_stars.push(Coordinates3d {
                x: new_instance.rng.f32() * 159.0,
                y: new_instance.rng.f32() * 159.0,
                z: 0.0,
                w: 1.0,
            });
        }

        for _ in 0..9 {
            new_instance.debris.push(Coordinates3d {
                x: {
                    let x = 80.0 + new_instance.rng.f32() * 40.0 - 20.0;
                    if x == 80.0 {
                        81.0
                    } else {
                        x
                    }
                },
                y: {
                    let y = 80.0 + new_instance.rng.f32() * 40.0 - 20.0;
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

        new_instance.planets.push(Planet::new(
            -300.0,
            -300.0,
            1000.0,
            "Metallia",
            planets::Type::B,
        ));

        new_instance.planets.push(Planet::new(
            -200.0,
            -200.0,
            5000.0,
            "Farm'leh",
            planets::Type::A,
        ));
        new_instance
    }

    pub fn copy(&self) -> Self {
        Self {
            current_flying_mode: self.current_flying_mode.clone(),
            debris: self.debris.clone(),
            distant_stars: self.distant_stars.clone(),
            movement: Movement {
                delta_x: self.movement.delta_x,
                delta_y: self.movement.delta_y,
            },
            nearest_planet_distance: self.nearest_planet_distance,
            planets: self.planets.clone(),
            player_ship: self.player_ship.clone(),
            rng: self.rng.clone(),
            targeting_planet: self.targeting_planet,
            theta: self.theta,
        }
    }

    pub fn update(&self, buttons: &Buttons, cooldown_tick: i32) -> (Self, Option<Planet>) {
        let mut new_instance = self.copy();
        let should_land = update_movement(&mut new_instance, buttons);
        update_debris(&mut new_instance);
        update_stars(&mut new_instance);
        new_instance.player_ship.update_speed(buttons);
        update_planets(&mut new_instance, cooldown_tick);
        draw(&new_instance, buttons);
        (new_instance, should_land)
    }
}
