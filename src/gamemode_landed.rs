use crate::{game::Buttons, graphics, palette::set_draw_color, planet::Planet, wasm4::*};

#[derive(PartialEq, Eq, Clone)]
#[repr(i8)]
pub enum PlanetMenuOption {
    FlyOut = 0,
    Buy = 1,
    SeePlanet = 2,
}

fn update_movement(mut mode: &mut GameModeLanded, buttons: &Buttons) {
    let mut tmp_select = mode.selected_planet_menu_option.clone() as u8;
    if buttons.down {
        if tmp_select < 2 {
            tmp_select = tmp_select + 1;
        }
    }
    if buttons.up {
        if tmp_select > 0 {
            tmp_select = tmp_select - 1;
        }
    }
    mode.selected_planet_menu_option = match tmp_select {
        0 => PlanetMenuOption::FlyOut,
        1 => PlanetMenuOption::Buy,
        _ => PlanetMenuOption::SeePlanet,
    };

    // if buttons.one && mode.cooldown_tick == 0 {
    // match mode.selected_planet_menu_option {
    // PlanetMenuOption::FlyOut => {
    // mode.current_mode = GameMode::Flying;
    // mode.cooldown_tick = 10;
    // }
    // _ => {}
    // }
    // }
}

fn draw(mode: &GameModeLanded) {
    set_draw_color(0x0001);
    graphics::draw_planet_landed(&mode.planet);

    set_draw_color(0x0012);
    text("Fly out", 37, 27);
    text("Buy", 37, 47);
    text("See Planet", 37, 67);
    match mode.selected_planet_menu_option {
        PlanetMenuOption::FlyOut => {
            text(b"\x85", 27, 27);
        }
        PlanetMenuOption::Buy => {
            text(b"\x85", 27, 47);
        }
        PlanetMenuOption::SeePlanet => {
            text(b"\x85", 27, 67);
        }
    }
}

pub struct GameModeLanded {
    selected_planet_menu_option: PlanetMenuOption,
    planet: Planet,
}

impl GameModeLanded {
    pub fn new(planet: Planet) -> Self {
        Self {
            selected_planet_menu_option: PlanetMenuOption::FlyOut,
            planet: planet.clone(),
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            selected_planet_menu_option: self.selected_planet_menu_option.clone(),
            planet: self.planet.clone(),
        }
    }

    pub fn update(&self, buttons: &Buttons) -> Self {
        let mut new_instance = self.copy();
        update_movement(&mut new_instance, buttons);
        draw(&new_instance);
        new_instance
    }
}
