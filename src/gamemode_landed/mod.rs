use crate::{
    buttons::Buttons,
    palette::set_draw_color,
    planets::{planet_hud, Planet},
    wasm4::{blit, text},
};

use self::submenu_main::MainMenu;

mod submenu_main;

#[derive(PartialEq, Clone)]
#[repr(i8)]
pub enum Mode {
    SubmenuMain(MainMenu),
    // SubmenuBuy(BuyMenu),
    // SubmenuSell(SellMenu),
    FlyAway,
}

#[derive(PartialEq, Clone)]
#[repr(i8)]
pub enum Action {
    MainMenu,
    // BuyMenu,
    // SellMenu,
    FlyAway,
}

#[derive(PartialEq, Clone)]
#[repr(i8)]
pub enum StateTransition {
    NoChange,
    ChangeTo(Action),
}

fn draw(mode: &GameModeLanded) {
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
    text(&mode.planet.name, 27, 5);

    match &mode.menu {
        Mode::SubmenuMain(menu) => menu.draw(),
        Mode::FlyAway => {}
    }
}

pub struct GameModeLanded {
    menu: Mode,
    planet: Planet,
}

impl GameModeLanded {
    pub fn new(planet: &Planet) -> Self {
        Self {
            menu: Mode::SubmenuMain(MainMenu::new(&planet)),
            planet: planet.clone(),
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            menu: self.menu.clone(),
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

        let state_transition = match new_instance.menu {
            Mode::SubmenuMain(menu) => {
                let (updated_menu, state_transition) =
                    menu.update_movement(just_pressed, pressed_this_frame, cooldown_tick);
                new_instance.menu = Mode::SubmenuMain(updated_menu);
                state_transition
            }
            Mode::FlyAway => StateTransition::NoChange,
        };

        match state_transition {
            StateTransition::ChangeTo(Action::MainMenu) => {
                new_instance.menu = Mode::SubmenuMain(MainMenu::new(&new_instance.planet));
            }
            _ => {}
        }

        draw(&new_instance);
        (new_instance, state_transition)
    }
}
