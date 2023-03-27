extern crate alloc;
use core::f32::MAX;

use alloc::{borrow::ToOwned, vec::Vec};

use fastrand::Rng;

use crate::{
    graphics, hud,
    maths::{distance, Coordinates},
    palette::set_draw_color,
    planet::Planet,
    planet_hud, player,
    wasm4::{self, *},
};

use numtoa::NumToA;

static mut PREVIOUS_GAMEPAD: u8 = 0;

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

#[derive(PartialEq, Eq)]
pub enum GameMode {
    Flying,
    LandingPossible(usize),
    Landed(usize),
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum PlanetMenuOption {
    FlyOut = 0,
    Buy = 1,
    SeePlanet = 2,
}

pub type PlanetTargeting = Option<usize>;

pub struct Game {
    rng: Rng,
    theta: f32,
    player_ship: player::PlayerShip,
    current_tick: i32,
    debris: Vec<Coordinates>,
    distant_stars: Vec<Coordinates>,
    button_just_pressed: Buttons,
    button_pressed_this_frame: Buttons,
    movement: Movement,
    planets: Vec<Planet>,
    current_mode: GameMode,
    nearest_planet_distance: f32,
    targeting_planet: PlanetTargeting,
    selected_planet_menu_option: PlanetMenuOption,
}

const MAXIMUM_DISTANCE_FOR_LANDING: f32 = 100.0;
const MAXIMUM_DISTANCE_FOR_TARGETING: f32 = 5000.0;

fn is_flying(gamemode: &GameMode) -> bool {
    match gamemode {
        GameMode::Flying => true,
        GameMode::LandingPossible(_) => true,
        _ => false,
    }
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
            button_just_pressed: Buttons {
                up: false,
                down: false,
                left: false,
                right: false,
                two: false,
                one: false,
            },
            button_pressed_this_frame: Buttons {
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
            current_mode: GameMode::Flying,
            nearest_planet_distance: 0.0,
            targeting_planet: None,
            selected_planet_menu_option: PlanetMenuOption::FlyOut,
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

    pub fn draw(&self) {
        match self.current_mode {
            GameMode::Flying | GameMode::LandingPossible(_) => {
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
                if self.button_just_pressed.one {
                    text(b"\x80", 140, 150);
                }
                if self.button_just_pressed.two {
                    text(b"\x81", 150, 150);
                    let s = self.player_ship.speed.numtoa_str(10, &mut buf);
                    if self.button_just_pressed.up {
                        text("SPD+ ".to_owned() + s, 1, 150);
                    }
                    if self.button_just_pressed.down {
                        text("SPD- ".to_owned() + s, 1, 150);
                    }
                }

                set_draw_color(0x0040);
                rect(0, 0, 160, 160);
            }

            GameMode::Landed(planet_index) => {
                set_draw_color(0x0001);
                graphics::draw_planet_landed(&self.planets[planet_index]);

                set_draw_color(0x0143);
                blit(
                    &planet_hud::PLANET_HUD,
                    20,
                    20,
                    planet_hud::PLANET_HUD_WIDTH,
                    planet_hud::PLANET_HUD_HEIGHT,
                    planet_hud::PLANET_HUD_FLAGS,
                );

                set_draw_color(0x0012);
                text("Fly out", 37, 27);
                text("Buy", 37, 47);
                text("See Planet", 37, 67);
                match self.selected_planet_menu_option {
                    PlanetMenuOption::FlyOut => {
                        text(b"\x85", 27, 27);
                    }
                    PlanetMenuOption::Buy => {
                        text(b"\x85", 27, 47);
                    }
                    PlanetMenuOption::SeePlanet => {
                        text(b"\x85", 27, 67);
                    }
                }
            }
        }

        if let GameMode::LandingPossible(planet_index) = &self.current_mode {
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

    pub fn update_pressed_buttons(&mut self) {
        let (pressed_this_frame, just_pressed) = unsafe {
            let previous = PREVIOUS_GAMEPAD;
            let gamepad = *wasm4::GAMEPAD1;
            let pressed_this_frame = gamepad & (gamepad ^ previous);
            PREVIOUS_GAMEPAD = gamepad;
            (pressed_this_frame, gamepad)
        };

        self.button_just_pressed = Buttons {
            up: false,
            down: false,
            left: false,
            right: false,
            two: false,
            one: false,
        };
        self.button_pressed_this_frame = Buttons {
            up: false,
            down: false,
            left: false,
            right: false,
            two: false,
            one: false,
        };

        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.button_just_pressed.up = true;
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.button_just_pressed.down = true;
        }
        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.button_just_pressed.left = true;
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.button_just_pressed.right = true;
        }
        if just_pressed & wasm4::BUTTON_1 != 0 {
            self.button_just_pressed.one = true;
        }
        if just_pressed & wasm4::BUTTON_2 != 0 {
            self.button_just_pressed.two = true;
        }

        if pressed_this_frame & wasm4::BUTTON_UP != 0 {
            self.button_pressed_this_frame.up = true;
        }
        if pressed_this_frame & wasm4::BUTTON_DOWN != 0 {
            self.button_pressed_this_frame.down = true;
        }
        if pressed_this_frame & wasm4::BUTTON_LEFT != 0 {
            self.button_pressed_this_frame.left = true;
        }
        if pressed_this_frame & wasm4::BUTTON_RIGHT != 0 {
            self.button_pressed_this_frame.right = true;
        }
        if pressed_this_frame & wasm4::BUTTON_1 != 0 {
            self.button_pressed_this_frame.one = true;
        }
        if pressed_this_frame & wasm4::BUTTON_2 != 0 {
            self.button_pressed_this_frame.two = true;
        }
    }

    pub fn update_movement(&mut self) {
        if is_flying(&self.current_mode) {
            self.movement.delta_x = DirectionX::Center;
            self.movement.delta_y = DirectionY::Center;
            if !self.button_just_pressed.two && !self.button_just_pressed.one {
                if self.button_just_pressed.up {
                    self.movement.delta_y = DirectionY::Up;
                }
                if self.button_just_pressed.down {
                    self.movement.delta_y = DirectionY::Down;
                }
                if self.button_just_pressed.left {
                    self.movement.delta_x = DirectionX::Left;
                }
                if self.button_just_pressed.right {
                    self.movement.delta_x = DirectionX::Right;
                }
            }
        }

        if let GameMode::LandingPossible(planet_index) = &self.current_mode {
            if self.button_just_pressed.one {
                self.current_mode = GameMode::Landed(*planet_index);
            }
        }

        if let GameMode::Landed(_) = &self.current_mode {
            if self.current_tick % 10 == 0 {
                let mut tmp_select = self.selected_planet_menu_option.clone() as u8;
                if self.button_pressed_this_frame.down {
                    if tmp_select < 2 {
                        tmp_select = tmp_select + 1;
                    }
                }
                if self.button_pressed_this_frame.up {
                    if tmp_select > 0 {
                        tmp_select = tmp_select - 1;
                    }
                }
                self.selected_planet_menu_option = match tmp_select {
                    0 => PlanetMenuOption::FlyOut,
                    1 => PlanetMenuOption::Buy,
                    _ => PlanetMenuOption::SeePlanet,
                }
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
        if is_flying(&self.current_mode) {
            let mut tmp_distance: f32;
            let mut nearest_distance: f32 = MAX;
            let mut tmp_landing_possible_on_index: usize = 0;
            let mut tmp_targeting_planet_index: usize = 0;
            for (index, planet) in self.planets.iter_mut().enumerate() {
                planet.update(&self.movement, self.theta, self.player_ship.speed);
                tmp_distance = distance(planet.coordinates);
                if tmp_distance < nearest_distance {
                    tmp_landing_possible_on_index = index;
                    tmp_targeting_planet_index = index;
                    nearest_distance = tmp_distance;
                }
            }
            if nearest_distance < MAXIMUM_DISTANCE_FOR_LANDING {
                self.current_mode = GameMode::LandingPossible(tmp_landing_possible_on_index);
            }
            if nearest_distance > MAXIMUM_DISTANCE_FOR_LANDING {
                self.current_mode = GameMode::Flying;
            }
            if self.nearest_planet_distance < MAXIMUM_DISTANCE_FOR_TARGETING {
                self.targeting_planet = Some(tmp_targeting_planet_index);
            } else {
                self.targeting_planet = None;
            }
            self.nearest_planet_distance = nearest_distance;
        }
    }

    pub fn update(&mut self) {
        self.current_tick = self.current_tick + 1;
        self.update_pressed_buttons();
        self.update_movement();
        self.update_debris();
        self.update_stars();
        self.player_ship.update_speed(&self.button_just_pressed);
        self.update_planets();
        self.draw();
    }
}
