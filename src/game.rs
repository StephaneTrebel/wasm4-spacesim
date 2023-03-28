use crate::{
    gamemode_flying::GameModeFlying,
    gamemode_landed::GameModeLanded,
    wasm4::{BUTTON_1, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, GAMEPAD1},
};

static mut PREVIOUS_GAMEPAD: u8 = 0;

pub struct Buttons {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub two: bool,
    pub one: bool,
}

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
}

impl Game {
    pub fn new() -> Self {
        Self {
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
            current_mode: GameMode::None,
        }
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
                let (new_mode, landingpossible_planet) =
                    mode.update(&self.button_just_pressed, self.cooldown_tick);
                self.current_mode = GameMode::Flying(new_mode);
                // Handle game mode transition
                if let Some(planet) = landingpossible_planet {
                    if self.button_just_pressed.one {
                        self.current_mode = GameMode::Landed(GameModeLanded::new(planet.clone()));
                        self.cooldown_tick = 10;
                    }
                }
            }
            GameMode::Landed(mode) => {
                let (new_mode, should_flyout) = mode.update(
                    &self.button_just_pressed,
                    &self.button_pressed_this_frame,
                    self.cooldown_tick,
                );
                self.current_mode = GameMode::Landed(new_mode);
                // Handle game mode transition
                if should_flyout {
                    // TODO Replace ::new with a factory that will create a flying mode
                    // from the current planet and with the current ship
                    self.current_mode = GameMode::Flying(GameModeFlying::new());
                    self.cooldown_tick = 10;
                }
            }
            _ => {}
        };
    }
}
