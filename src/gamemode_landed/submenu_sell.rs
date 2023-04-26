extern crate alloc;
use alloc::borrow::ToOwned;

use numtoa::NumToA;

use crate::{buttons::Buttons, items::Item, planets::Planet, player::PlayerShip, wasm4::text};

use super::{Action, StateTransition};

#[derive(PartialEq, Clone)]
pub struct SellMenu {
    selected_option: u8,
    planet: Planet,
    player_ship: PlayerShip,
    bought_stuff: Option<(Item, u32)>,
}

const QUANTITY: u32 = 10;

impl SellMenu {
    pub fn new(
        planet: &Planet,
        player_ship: &PlayerShip,
        bought_stuff: Option<(&Item, u32)>,
    ) -> Self {
        Self {
            selected_option: 0,
            planet: planet.clone(),
            player_ship: player_ship.clone(),
            bought_stuff: match bought_stuff {
                None => None,
                Some((item, quantity)) => Some((item.clone(), quantity)),
            },
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            selected_option: self.selected_option.clone(),
            planet: self.planet.clone(),
            player_ship: self.player_ship.clone(),
            bought_stuff: self.bought_stuff.clone(),
        }
    }

    pub fn update_movement(
        &self,
        just_pressed: &Buttons,
        pressed_this_frame: &Buttons,
        cooldown_tick: i32,
    ) -> (Self, StateTransition) {
        let mut new_instance = self.copy();
        let inventory_count = self.planet.inventory.len() as u8;

        let mut tmp_select = new_instance.selected_option.clone() as u8;
        if pressed_this_frame.down {
            if tmp_select < inventory_count {
                tmp_select = tmp_select + 1;
            }
        }
        if pressed_this_frame.up {
            if tmp_select > 0 {
                tmp_select = tmp_select - 1;
            }
        }

        let mut state_transition: StateTransition = StateTransition::NoChange;
        new_instance.selected_option = tmp_select;
        if just_pressed.one && cooldown_tick == 0 {
            match new_instance.selected_option {
                0 => state_transition = StateTransition::ChangeTo(Action::MainMenu),
                index if index < (inventory_count + 1) => {
                    if let Some((item, inventory)) = new_instance
                        .planet
                        .inventory
                        .clone()
                        .into_iter()
                        .nth((new_instance.selected_option - 1) as usize)
                    {
                        if let Ok(_) = new_instance.planet.can_buy(&item, QUANTITY) {
                            if let Ok(_) = new_instance.player_ship.can_sell(&item, QUANTITY) {
                                state_transition = StateTransition::ChangeTo(Action::Sell(
                                    new_instance.planet.name.clone(),
                                    item,
                                    QUANTITY,
                                    inventory.selling_price,
                                ));
                            }
                        }
                    }
                }
                _ => {}
            };
        }
        (new_instance, state_transition)
    }

    pub fn draw(&self, cooldown_tick: i32) {
        text("Back", 37, 27);
        for (index, item) in self.player_ship.inventory.clone().into_iter().enumerate() {
            text(item.0.get_name(), 37, 47 + index as i32 * 20);
        }
        text(b"\x85", 27, 27 + self.selected_option as i32 * 20);
        if cooldown_tick > 0 {
            match self.bought_stuff {
                Some((item, quantity)) => {
                    let mut buf = [0u8; 32];
                    text(
                        "- ".to_owned() + item.get_name() + " " + quantity.numtoa_str(10, &mut buf),
                        27,
                        147,
                    );
                }
                None => {}
            }
        }
    }
}
