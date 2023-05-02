extern crate alloc;
use alloc::string::String;

use crate::{
    buttons::Buttons,
    items::Item,
    maths::Quantity,
    palette::set_draw_color,
    planets::{planet_hud, Planet},
    player::PlayerShip,
    wasm4::{blit, text},
};

use self::{submenu_buy::BuyMenu, submenu_main::MainMenu, submenu_sell::SellMenu};

mod submenu_buy;
mod submenu_main;
mod submenu_sell;

#[derive(PartialEq, Clone)]
#[repr(i8)]
pub enum Mode {
    SubmenuMain(MainMenu),
    SubmenuBuy(BuyMenu),
    SubmenuSell(SellMenu),
    FlyAway,
}

#[derive(PartialEq, Clone)]
#[repr(i8)]
pub enum Action {
    MainMenu,

    BuyMenu,
    Buy(String, Item, Quantity, u32),

    SellMenu,
    Sell(String, Item, Quantity, u32),

    FlyAway,
}

#[derive(PartialEq, Clone)]
#[repr(i8)]
pub enum StateTransition {
    NoChange,
    ChangeTo(Action),
}

pub struct GameModeLanded {
    menu: Mode,
    planet: Planet,
    player_ship: PlayerShip,
}

impl GameModeLanded {
    pub fn new(planet: &Planet, player_ship: &PlayerShip, optional_action: Option<Action>) -> Self {
        Self {
            menu: match optional_action {
                None => Mode::SubmenuMain(MainMenu::new(&planet)),
                Some(action) => match action {
                    Action::Buy(_, item, quantity, _) => Mode::SubmenuBuy(BuyMenu::new(
                        &planet,
                        &player_ship,
                        Some((&item, quantity)),
                    )),
                    Action::Sell(_, item, quantity, _) => Mode::SubmenuSell(SellMenu::new(
                        &planet,
                        &player_ship,
                        Some((&item, quantity)),
                    )),
                    _ => Mode::SubmenuMain(MainMenu::new(&planet)),
                },
            },
            planet: planet.clone(),
            player_ship: player_ship.clone(),
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            menu: self.menu.clone(),
            planet: self.planet.clone(),
            player_ship: self.player_ship.clone(),
        }
    }

    pub fn draw(&self, cooldown_tick: i32) {
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
        text(&self.planet.name, 27, 5);

        match &self.menu {
            Mode::SubmenuMain(menu) => menu.draw(),
            Mode::SubmenuBuy(menu) => menu.draw(cooldown_tick),
            Mode::SubmenuSell(menu) => menu.draw(cooldown_tick),
            Mode::FlyAway => {}
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
            Mode::SubmenuBuy(menu) => {
                let (updated_menu, state_transition) =
                    menu.update_movement(just_pressed, pressed_this_frame, cooldown_tick);
                new_instance.menu = Mode::SubmenuBuy(updated_menu);
                state_transition
            }
            Mode::SubmenuSell(menu) => {
                let (updated_menu, state_transition) =
                    menu.update_movement(just_pressed, pressed_this_frame, cooldown_tick);
                new_instance.menu = Mode::SubmenuSell(updated_menu);
                state_transition
            }
            Mode::FlyAway => StateTransition::ChangeTo(Action::FlyAway),
        };

        match state_transition {
            StateTransition::ChangeTo(Action::MainMenu) => {
                new_instance.menu = Mode::SubmenuMain(MainMenu::new(&new_instance.planet));
            }
            StateTransition::ChangeTo(Action::BuyMenu) => {
                new_instance.menu =
                    Mode::SubmenuBuy(BuyMenu::new(&new_instance.planet, &self.player_ship, None));
            }
            StateTransition::ChangeTo(Action::SellMenu) => {
                new_instance.menu =
                    Mode::SubmenuSell(SellMenu::new(&new_instance.planet, &self.player_ship, None));
            }
            StateTransition::ChangeTo(Action::Buy(_, item, quantity, _)) => {
                new_instance.menu = Mode::SubmenuBuy(BuyMenu::new(
                    &new_instance.planet,
                    &self.player_ship,
                    Some((&item, quantity)),
                ));
            }
            StateTransition::ChangeTo(Action::Sell(_, item, quantity, _)) => {
                new_instance.menu = Mode::SubmenuSell(SellMenu::new(
                    &new_instance.planet,
                    &self.player_ship,
                    Some((&item, quantity)),
                ));
            }
            StateTransition::ChangeTo(Action::FlyAway) => {
                new_instance.menu = Mode::FlyAway;
            }
            StateTransition::NoChange => {}
        };

        self.draw(cooldown_tick);
        (new_instance, state_transition)
    }
}
