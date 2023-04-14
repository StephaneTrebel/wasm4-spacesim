use crate::{buttons::Buttons, planets::Planet, wasm4::text};

use super::{Action, StateTransition};

#[derive(PartialEq, Clone)]
#[repr(i8)]
pub enum MainMenuOption {
    FlyAway,
    Buy,
    Sell,
}

#[derive(PartialEq, Clone)]
pub struct MainMenu {
    selected_option: MainMenuOption,
    planet: Planet,
}

impl MainMenu {
    pub fn new(planet: &Planet) -> Self {
        Self {
            selected_option: MainMenuOption::FlyAway,
            planet: planet.clone(),
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            selected_option: self.selected_option.clone(),
            planet: self.planet.clone(),
        }
    }

    pub fn update_movement(
        &self,
        just_pressed: &Buttons,
        pressed_this_frame: &Buttons,
        cooldown_tick: i32,
    ) -> (Self, StateTransition) {
        let mut new_instance = self.copy();

        let state_transition: StateTransition = {
            let mut tmp_select = new_instance.selected_option.clone() as u8;
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
            new_instance.selected_option = match tmp_select {
                0 => MainMenuOption::FlyAway,
                1 => MainMenuOption::Buy,
                _ => MainMenuOption::Sell,
            };
            if just_pressed.one && cooldown_tick == 0 {
                match self.selected_option {
                    MainMenuOption::FlyAway => StateTransition::ChangeTo(Action::FlyAway),
                    // MainMenuOption::Buy => StateTransition::ChangeTo(Action::Buy),
                    // MainMenuOption::Sell => StateTransition::ChangeTo(Action::Sell),
                    _ => StateTransition::NoChange,
                };
            }
            StateTransition::NoChange
        };
        (new_instance, state_transition)
    }

    pub fn draw(&self) {
        text("Fly Away", 37, 27);
        text("Buy", 37, 47);
        text("Sell", 37, 67);
        text(
            b"\x85",
            27,
            match self.selected_option {
                MainMenuOption::FlyAway => 27,
                MainMenuOption::Buy => 47,
                MainMenuOption::Sell => 67,
            },
        );
    }
}
