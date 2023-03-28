use crate::{
    gamemode_flying::GameModeFlying,
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
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum PlanetMenuOption {
    FlyOut = 0,
    Buy = 1,
    SeePlanet = 2,
}

pub struct Game {
    current_tick: i32,
    cooldown_tick: i32,
    button_just_pressed: Buttons,
    button_pressed_this_frame: Buttons,
    current_mode: GameMode,
    selected_planet_menu_option: PlanetMenuOption,
}

impl Game {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
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
            selected_planet_menu_option: PlanetMenuOption::FlyOut,
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

    pub fn draw(&mut self) {
        // GameMode::Landed(planet_index) => {
        // set_draw_color(0x0001);
        // graphics::draw_planet_landed(&self.planets[planet_index]);

        // set_draw_color(0x0012);
        // text("Fly out", 37, 27);
        // text("Buy", 37, 47);
        // text("See Planet", 37, 67);
        // match self.selected_planet_menu_option {
        // PlanetMenuOption::FlyOut => {
        // text(b"\x85", 27, 27);
        // }
        // PlanetMenuOption::Buy => {
        // text(b"\x85", 27, 47);
        // }
        // PlanetMenuOption::SeePlanet => {
        // text(b"\x85", 27, 67);
        // }
        // }
        // }
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

    pub fn update_movement(&mut self) {

        // if let GameMode::Landed(_) = &self.current_mode {
        // let mut tmp_select = self.selected_planet_menu_option.clone() as u8;
        // if self.button_pressed_this_frame.down {
        // if tmp_select < 2 {
        // tmp_select = tmp_select + 1;
        // }
        // }
        // if self.button_pressed_this_frame.up {
        // if tmp_select > 0 {
        // tmp_select = tmp_select - 1;
        // }
        // }
        // self.selected_planet_menu_option = match tmp_select {
        // 0 => PlanetMenuOption::FlyOut,
        // 1 => PlanetMenuOption::Buy,
        // _ => PlanetMenuOption::SeePlanet,
        // };

        // if self.button_just_pressed.one && self.cooldown_tick == 0 {
        // match self.selected_planet_menu_option {
        // PlanetMenuOption::FlyOut => {
        // self.current_mode = GameMode::Flying;
        // self.cooldown_tick = 10;
        // }
        // _ => {}
        // }
        // }
        // }
    }

    pub fn update(&mut self) {
        self.update_pressed_buttons();
        match &mut self.current_mode {
            GameMode::Flying(mode) => {
                self.current_mode = GameMode::Flying(mode.update(&self.button_just_pressed));
            }
            _ => {}
        }
    }
}
