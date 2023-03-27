extern crate alloc;
use alloc::boxed::Box;

mod planet_sprite_12;
mod planet_sprite_120;
mod planet_sprite_160;
mod planet_sprite_40;
mod planet_sprite_80;

pub enum Level {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
}

pub fn get_level(distance: f32) -> Level {
    match distance.floor() as u32 {
        0..=500 => Level::ONE,
        501..=1000 => Level::TWO,
        1001..=2000 => Level::THREE,
        2001..=3000 => Level::FOUR,
        _ => Level::FIVE,
    }
}

pub fn get_sprite(level: &Level) -> Box<[u8]> {
    match level {
        Level::ONE => Box::new(planet_sprite_160::PLANET1_160),
        Level::TWO => Box::new(planet_sprite_120::PLANET1_120),
        Level::THREE => Box::new(planet_sprite_80::PLANET1_80),
        Level::FOUR => Box::new(planet_sprite_40::PLANET1_40),
        Level::FIVE => Box::new(planet_sprite_12::PLANET1_12),
    }
}

pub fn get_width(level: &Level) -> u32 {
    match level {
        Level::ONE => planet_sprite_160::PLANET1_160_WIDTH,
        Level::TWO => planet_sprite_120::PLANET1_120_WIDTH,
        Level::THREE => planet_sprite_80::PLANET1_80_WIDTH,
        Level::FOUR => planet_sprite_40::PLANET1_40_WIDTH,
        Level::FIVE => planet_sprite_12::PLANET1_12_WIDTH,
    }
}

pub fn get_height(level: &Level) -> u32 {
    match level {
        Level::ONE => planet_sprite_160::PLANET1_160_HEIGHT,
        Level::TWO => planet_sprite_120::PLANET1_120_HEIGHT,
        Level::THREE => planet_sprite_80::PLANET1_80_HEIGHT,
        Level::FOUR => planet_sprite_40::PLANET1_40_HEIGHT,
        Level::FIVE => planet_sprite_12::PLANET1_12_HEIGHT,
    }
}

pub fn get_flags(level: &Level) -> u32 {
    match level {
        Level::ONE => planet_sprite_160::PLANET1_160_FLAGS,
        Level::TWO => planet_sprite_120::PLANET1_120_FLAGS,
        Level::THREE => planet_sprite_80::PLANET1_80_FLAGS,
        Level::FOUR => planet_sprite_40::PLANET1_40_FLAGS,
        Level::FIVE => planet_sprite_12::PLANET1_12_FLAGS,
    }
}

pub fn get_colors() -> u16 {
    0x0234
}
