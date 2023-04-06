extern crate alloc;
use alloc::boxed::Box;

mod planet_landscape;
mod planet_sprite_12;
mod planet_sprite_120;
mod planet_sprite_40;
mod planet_sprite_80;

pub enum Level {
    ONE,
    TWO,
    THREE,
    FOUR,
    LANDSCAPE = 99,
}

pub fn get_level(distance: f32) -> Level {
    match distance.floor() as u32 {
        0..=500 => Level::ONE,
        501..=1000 => Level::ONE,
        1001..=2000 => Level::TWO,
        2001..=3000 => Level::THREE,
        _ => Level::FOUR,
    }
}

pub fn get_sprite(level: &Level) -> Box<[u8]> {
    match level {
        Level::ONE => Box::new(planet_sprite_120::PLANET_120),
        Level::TWO => Box::new(planet_sprite_80::PLANET_80),
        Level::THREE => Box::new(planet_sprite_40::PLANET_40),
        Level::FOUR => Box::new(planet_sprite_12::PLANET_12),
        Level::LANDSCAPE => Box::new(planet_landscape::PLANET_LANDSCAPE),
    }
}

pub fn get_width(level: &Level) -> u32 {
    match level {
        Level::ONE => planet_sprite_120::PLANET_120_WIDTH,
        Level::TWO => planet_sprite_80::PLANET_80_WIDTH,
        Level::THREE => planet_sprite_40::PLANET_40_WIDTH,
        Level::FOUR => planet_sprite_12::PLANET_12_WIDTH,
        Level::LANDSCAPE => planet_landscape::PLANET_LANDSCAPE_WIDTH,
    }
}

pub fn get_height(level: &Level) -> u32 {
    match level {
        Level::ONE => planet_sprite_120::PLANET_120_HEIGHT,
        Level::TWO => planet_sprite_80::PLANET_80_HEIGHT,
        Level::THREE => planet_sprite_40::PLANET_40_HEIGHT,
        Level::FOUR => planet_sprite_12::PLANET_12_HEIGHT,
        Level::LANDSCAPE => planet_landscape::PLANET_LANDSCAPE_HEIGHT,
    }
}

pub fn get_flags(level: &Level) -> u32 {
    match level {
        Level::ONE => planet_sprite_120::PLANET_120_FLAGS,
        Level::TWO => planet_sprite_80::PLANET_80_FLAGS,
        Level::THREE => planet_sprite_40::PLANET_40_FLAGS,
        Level::FOUR => planet_sprite_12::PLANET_12_FLAGS,
        Level::LANDSCAPE => planet_landscape::PLANET_LANDSCAPE_FLAGS,
    }
}

pub fn get_colors(level: &Level) -> u16 {
    match level {
        Level::LANDSCAPE => 0x0432,
        _ => 0x0234,
    }
}
