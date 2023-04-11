use buttons::Buttons;
use planets::{planet_hud, Planet};
use wasm4::palette::set_draw_color;
use wasm4::wasm4::{blit, text};

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum Mode {
    Menu,
    ShowPlanet,
    ExitMode,
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum MenuOption {
    FlyOut,
    Buy,
    SeePlanet,
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum StateTransition {
    NoChange,
    ChangeTo(Mode),
}

fn update_movement(
    mut gamemode: &mut GameModeLanded,
    just_pressed: &Buttons,
    pressed_this_frame: &Buttons,
    cooldown_tick: i32,
) -> StateTransition {
    match gamemode.submode {
        Mode::Menu => {
            let mut tmp_select = gamemode.selected_planet_menu_option.clone() as u8;
            if pressed_this_frame.down {
                if tmp_select < 2 {
                    tmp_select = tmp_select + 1;
                }
            }
            if pressed_this_frame.up {
                if tmp_select > 0 {
                    tmp_select = tmp_select - 1;
                }
            }
            gamemode.selected_planet_menu_option = match tmp_select {
                0 => MenuOption::FlyOut,
                1 => MenuOption::Buy,
                _ => MenuOption::SeePlanet,
            };

            if just_pressed.one && cooldown_tick == 0 {
                match gamemode.selected_planet_menu_option {
                    MenuOption::FlyOut => return StateTransition::ChangeTo(Mode::ExitMode),
                    MenuOption::SeePlanet => return StateTransition::ChangeTo(Mode::ShowPlanet),
                    _ => return StateTransition::NoChange,
                }
            }
        }
        Mode::ShowPlanet => {
            if (just_pressed.up
                || just_pressed.down
                || just_pressed.left
                || just_pressed.right
                || just_pressed.one
                || just_pressed.two)
                && cooldown_tick == 0
            {
                return StateTransition::ChangeTo(Mode::Menu);
            }
        }
        _ => {}
    }
    StateTransition::NoChange
}

fn draw(gamemode: &GameModeLanded) {
    graphics::draw_planet_landed(&gamemode.planet);

    if gamemode.submode != Mode::ShowPlanet {
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
        match gamemode.selected_planet_menu_option {
            MenuOption::FlyOut => {
                text(b"\x85", 27, 27);
            }
            MenuOption::Buy => {
                text(b"\x85", 27, 47);
            }
            MenuOption::SeePlanet => {
                text(b"\x85", 27, 67);
            }
        }
    }
}

pub struct GameModeLanded {
    submode: Mode,
    selected_planet_menu_option: MenuOption,
    planet: Planet,
}

impl GameModeLanded {
    pub fn new(planet: Planet) -> Self {
        Self {
            submode: Mode::Menu,
            selected_planet_menu_option: MenuOption::FlyOut,
            planet: planet.clone(),
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            submode: self.submode.clone(),
            selected_planet_menu_option: self.selected_planet_menu_option.clone(),
            planet: self.planet.clone(),
        }
    }

    pub fn update(
        &self,
        just_pressed: &Buttons,
        pressed_this_frame: &Buttons,
        cooldown_tick: i32,
    ) -> (Self, StateTransition) {
        let mut new_instance = self.copy();
        let state_transition = update_movement(
            &mut new_instance,
            just_pressed,
            pressed_this_frame,
            cooldown_tick,
        );
        match state_transition {
            StateTransition::ChangeTo(Mode::ShowPlanet) => {
                new_instance.submode = Mode::ShowPlanet;
            }
            StateTransition::ChangeTo(Mode::Menu) => {
                new_instance.submode = Mode::Menu;
            }
            _ => {}
        }
        draw(&new_instance);
        (new_instance, state_transition)
    }
}
