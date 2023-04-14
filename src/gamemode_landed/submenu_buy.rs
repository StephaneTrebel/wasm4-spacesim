use crate::{buttons::Buttons, planets::Planet, wasm4::text};

use super::{Action, StateTransition};

#[derive(PartialEq, Clone)]
pub struct BuyMenu {
    selected_option: u8,
    planet: Planet,
}

impl BuyMenu {
    pub fn new(planet: &Planet) -> Self {
        Self {
            selected_option: 0,
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
            new_instance.selected_option = tmp_select;
            if just_pressed.one && cooldown_tick == 0 {
                match new_instance.selected_option {
                    0 => StateTransition::ChangeTo(Action::MainMenu),
                    // TODO BUY ITEMS HERE
                    _ => StateTransition::NoChange,
                };
            }
            StateTransition::NoChange
        };
        (new_instance, state_transition)
    }

    pub fn draw(&self) {
        text("Back", 37, 27);
        text(b"\x85", 27, 27 + self.selected_option as i32 * 20);
    }
}
