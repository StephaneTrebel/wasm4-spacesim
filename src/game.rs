extern crate alloc;
use alloc::string::String;
use hashbrown::HashMap;

use crate::{
    buttons::Buttons,
    gamemode_flying::GameModeFlying,
    gamemode_landed::{self, GameModeLanded},
    items::Item,
    planets::{self, Planet, PlanetItemInventory, Planets},
    player::PlayerShip,
    wasm4::{BUTTON_1, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, GAMEPAD1},
};

static mut PREVIOUS_GAMEPAD: u8 = 0;

pub enum GameMode {
    None, // Limbo zone while everything is loading
    Flying(GameModeFlying),
    Landed(GameModeLanded),
}

pub struct Game {
    cooldown_tick: i32,
    button_just_pressed: Buttons,
    button_pressed_this_frame: Buttons,
    current_mode: GameMode,
    player_ship: PlayerShip,
    planets: Planets,
}

impl Game {
    pub fn new() -> Self {
        let mut new_instance: Self = Self {
            cooldown_tick: 0,
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
            player_ship: PlayerShip::new(),
            planets: HashMap::new(),
            current_mode: GameMode::None,
        };

        new_instance.planets.insert(
            String::from("Metallia"),
            Planet::new(-300.0, -300.0, 1000.0, "Metallia", planets::Type::B, {
                let mut inventory = HashMap::new();
                inventory.insert(Item::IronIngot, PlanetItemInventory::new(1000, 10, 100));
                inventory.insert(Item::FoodCrate, PlanetItemInventory::new(100, 100, 10));
                inventory
            }),
        );

        new_instance.planets.insert(
            String::from("Farm'leh"),
            Planet::new(-200.0, -200.0, 5000.0, "Farm'leh", planets::Type::A, {
                let mut inventory = HashMap::new();
                inventory.insert(Item::IronIngot, PlanetItemInventory::new(100, 100, 10));
                inventory.insert(Item::FoodCrate, PlanetItemInventory::new(1000, 10, 100));
                inventory
            }),
        );
        new_instance
    }

    pub fn start(&mut self) {
        match &mut self.current_mode {
            GameMode::None => {
                self.current_mode = GameMode::Flying(GameModeFlying::new());
            }
            _ => {}
        }
    }

    pub fn update_pressed_buttons(&mut self) {
        let (pressed_this_frame, just_pressed) = unsafe {
            let previous = PREVIOUS_GAMEPAD;
            let gamepad = *GAMEPAD1;
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

        if just_pressed & BUTTON_UP != 0 {
            self.button_just_pressed.up = true;
        }
        if just_pressed & BUTTON_DOWN != 0 {
            self.button_just_pressed.down = true;
        }
        if just_pressed & BUTTON_LEFT != 0 {
            self.button_just_pressed.left = true;
        }
        if just_pressed & BUTTON_RIGHT != 0 {
            self.button_just_pressed.right = true;
        }
        if just_pressed & BUTTON_1 != 0 {
            self.button_just_pressed.one = true;
        }
        if just_pressed & BUTTON_2 != 0 {
            self.button_just_pressed.two = true;
        }

        if pressed_this_frame & BUTTON_UP != 0 {
            self.button_pressed_this_frame.up = true;
        }
        if pressed_this_frame & BUTTON_DOWN != 0 {
            self.button_pressed_this_frame.down = true;
        }
        if pressed_this_frame & BUTTON_LEFT != 0 {
            self.button_pressed_this_frame.left = true;
        }
        if pressed_this_frame & BUTTON_RIGHT != 0 {
            self.button_pressed_this_frame.right = true;
        }
        if pressed_this_frame & BUTTON_1 != 0 {
            self.button_pressed_this_frame.one = true;
        }
        if pressed_this_frame & BUTTON_2 != 0 {
            self.button_pressed_this_frame.two = true;
        }
    }

    pub fn update(&mut self) {
        if self.cooldown_tick > 0 {
            self.cooldown_tick -= 1;
        }
        self.update_pressed_buttons();
        match &mut self.current_mode {
            GameMode::Flying(mode) => {
                let (updated_mode, landingpossible_planet, updated_player_ship, updated_planets) =
                    mode.update(
                        &self.button_just_pressed,
                        self.cooldown_tick,
                        &self.player_ship,
                        &self.planets,
                    );
                self.current_mode = GameMode::Flying(updated_mode);
                self.player_ship = updated_player_ship.clone();
                self.planets = updated_planets.clone();

                // Handle game mode transition
                if let Some(planet) = landingpossible_planet {
                    if self.button_just_pressed.one {
                        self.current_mode =
                            GameMode::Landed(GameModeLanded::new(&planet, &self.player_ship, None));
                        self.cooldown_tick = 10;
                    }
                }
            }
            GameMode::Landed(mode) => {
                let (updated_mode, state_transition) = mode.update(
                    &self.button_just_pressed,
                    &self.button_pressed_this_frame,
                    self.cooldown_tick,
                );

                // Handle game mode transition
                match state_transition {
                    gamemode_landed::StateTransition::ChangeTo(next_state) => {
                        self.cooldown_tick = 10;
                        match next_state {
                            gamemode_landed::Action::FlyAway => {
                                // TODO Replace ::new with a factory that will create a flying mode
                                // from the current planet and with the current ship
                                self.current_mode = GameMode::Flying(GameModeFlying::new());
                            }
                            gamemode_landed::Action::Buy(planet_name, item, quantity, price) => {
                                // 2 seconds cooldown to show what's been bought
                                self.cooldown_tick = 60;
                                let mut updated_planet =
                                    self.planets.get(&planet_name).unwrap().clone();
                                updated_planet.sell(&item, quantity);
                                self.planets
                                    .insert(updated_planet.name.clone(), updated_planet);
                                self.player_ship.buy(&item, quantity, price);
                                self.current_mode = GameMode::Landed(GameModeLanded::new(
                                    &self.planets.get(&planet_name).unwrap(),
                                    &self.player_ship,
                                    Some(gamemode_landed::Action::Buy(
                                        planet_name,
                                        item,
                                        quantity,
                                        price,
                                    )),
                                ));
                            }
                            gamemode_landed::Action::Sell(planet_name, item, quantity, price) => {
                                // 2 seconds cooldown to show what's been bought
                                self.cooldown_tick = 60;
                                self.player_ship.sell(&item, quantity, price);
                                let mut updated_planet =
                                    self.planets.get(&planet_name).unwrap().clone();
                                updated_planet.buy(&item, quantity);
                                self.planets
                                    .insert(updated_planet.name.clone(), updated_planet);
                                self.current_mode = GameMode::Landed(GameModeLanded::new(
                                    &self.planets.get(&planet_name).unwrap(),
                                    &self.player_ship,
                                    Some(gamemode_landed::Action::Sell(
                                        planet_name,
                                        item,
                                        quantity,
                                        price,
                                    )),
                                ));
                            }
                            _ => self.current_mode = GameMode::Landed(updated_mode),
                        }
                    }
                    gamemode_landed::StateTransition::NoChange => {
                        self.current_mode = GameMode::Landed(updated_mode)
                    }
                }
            }
            _ => {}
        };
    }
}
