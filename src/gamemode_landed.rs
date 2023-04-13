use crate::{
    buttons::Buttons,
    palette::set_draw_color,
    planets::{planet_hud, Planet},
    wasm4::{blit, text},
};

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum Mode {
    MainMenu,
    BuyMenu,
    SellMenu,
    FlyAway,
}

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum MenuOption {
    FlyAway,
    Buy,
    Sell,
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
        Mode::MainMenu => {
            let mut tmp_select = gamemode.selected_planet_menu_option.clone() as u8;
            if pressed_this_frame.down {
                if tmp_select < 3 {
                    tmp_select = tmp_select + 1;
                }
            }
            if pressed_this_frame.up {
                if tmp_select > 0 {
                    tmp_select = tmp_select - 1;
                }
            }
            gamemode.selected_planet_menu_option = match tmp_select {
                0 => MenuOption::FlyAway,
                1 => MenuOption::Buy,
                _ => MenuOption::Sell,
            };

            if just_pressed.one && cooldown_tick == 0 {
                match gamemode.selected_planet_menu_option {
                    MenuOption::FlyAway => return StateTransition::ChangeTo(Mode::FlyAway),
                    _ => return StateTransition::NoChange,
                }
            }
        }
        Mode::BuyMenu => {}
        Mode::SellMenu => {}
        Mode::FlyAway => {}
    }
    StateTransition::NoChange
}

fn draw(gamemode: &GameModeLanded) {
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
    text(&gamemode.planet.name, 27, 5);

    match gamemode.submode {
        Mode::MainMenu => {
            text("Fly out", 37, 27);
            text("Buy", 37, 47);
            text("Sell", 37, 67);
            text(
                b"\x85",
                27,
                match gamemode.selected_planet_menu_option {
                    MenuOption::FlyAway => 27,
                    MenuOption::Buy => 47,
                    MenuOption::Sell => 67,
                },
            );
        }
        Mode::BuyMenu => {}
        Mode::SellMenu => {}
        Mode::FlyAway => {}
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
            submode: Mode::MainMenu,
            selected_planet_menu_option: MenuOption::FlyAway,
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
            StateTransition::ChangeTo(Mode::MainMenu) => {
                new_instance.submode = Mode::MainMenu;
            }
            _ => {}
        }
        draw(&new_instance);
        (new_instance, state_transition)
    }
}
