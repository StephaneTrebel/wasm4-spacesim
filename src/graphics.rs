use fastrand::Rng;
use numtoa::NumToA;

use crate::{
    maths::{project, Coordinates3d},
    palette::set_draw_color,
    planet::Planet,
    planet_sprite::{get_colors, get_flags, get_height, get_level, get_sprite, get_width, Level},
    wasm4::*,
};

pub fn pixel(x: i32, y: i32, color: u8) {
    if x < 0 || x > 159 {
        return;
    };
    if y < 0 || y > 159 {
        return;
    };
    // The byte index into the framebuffer that contains (x, y)
    let idx = (y as usize * 160 + x as usize) >> 2;

    // Calculate the bits within the byte that corresponds to our position
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;

    unsafe {
        let palette_color: u8 = (*DRAW_COLORS & 0xf) as u8;
        if palette_color == 0 {
            // Transparent
            return;
        }
        // let color = (palette_color - 1) & 0b11;

        let framebuffer = &mut *FRAMEBUFFER;

        framebuffer[idx] = (color << shift) | (framebuffer[idx] & !mask);
    }
}

pub fn draw_star(coords: &Coordinates3d) {
    pixel(coords.x as i32, coords.y as i32, 1);
}

pub fn draw_debris(coords: &Coordinates3d, rng: &Rng) {
    let delta_x = rng.i8(-1..1) as i32;
    let delta_y = rng.i8(-1..1) as i32;
    pixel(coords.x as i32 + delta_x, coords.y as i32 + delta_y, 1);
}

pub fn draw_planet(planet: &Planet) {
    if planet.coordinates.z >= 0.0 {
        let coordinates = project(planet.coordinates);
        let level = get_level(planet.distance);

        set_draw_color(get_colors(&level));
        let x = (coordinates.x + 80.0 - get_width(&level) as f32 / 2.0) as i32;
        let y = (coordinates.y + 80.0 - get_width(&level) as f32 / 2.0) as i32;
        blit(
            &get_sprite(&level),
            x,
            y,
            get_width(&level),
            get_height(&level),
            get_flags(&level),
        );
    }
}

pub fn draw_targeting(planet: &Planet) {
    let coordinates = project(planet.coordinates);
    let level = get_level(planet.distance);
    let x = (coordinates.x + 80.0 - get_width(&level) as f32 / 2.0) as i32;
    let y = (coordinates.y + 80.0 - get_height(&level) as f32 / 2.0) as i32;
    let center_x = x + get_width(&level) as i32 / 2;
    let center_y = y + get_height(&level) as i32 / 2;

    set_draw_color(0x0013);

    // Draw proper targeting reticle around planet
    if center_x > 0 && center_y > 0 && planet.coordinates.z > 0.0 {
        let mut buf = [0u8; 32];
        let distance = (planet.distance.floor() as i32).numtoa_str(10, &mut buf);
        text(
            distance,
            center_x -
             // distance string length is used to «center» the text above the planet
             (distance.len() as i32) * 4,
            y - 10,
        );
        set_draw_color(0x0002);
        line(x, y, x + get_width(&level) as i32 / 3, y);
        line(x, y, x, y + get_height(&level) as i32 / 3);
        line(
            x + get_width(&level) as i32,
            y + get_height(&level) as i32,
            x + get_width(&level) as i32 * 2 / 3,
            y + get_height(&level) as i32,
        );
        line(
            x + get_width(&level) as i32,
            y + get_height(&level) as i32,
            x + get_width(&level) as i32,
            y + get_height(&level) as i32 * 2 / 3,
        );
    }

    // Draw targeting indicator on screen edges
    set_draw_color(0x0002);
    if center_x < 0 && center_y > 0 {
        text(b"\x84", 2, center_y);
    }
    if center_x > 0 && center_y < 0 {
        text(b"\x86", center_x, 2);
    }
    if center_x > 160 && center_y > 0 {
        text(b"\x85", 151, center_y);
    }
    if center_x > 0 && center_y > 160 {
        text(b"\x87", center_x, 151);
    }
}

pub fn draw_planet_landed(_planet: &Planet) {
    let level = Level::LANDSCAPE;
    set_draw_color(get_colors(&level));
    blit(
        &get_sprite(&level),
        0,
        0,
        get_width(&level),
        get_height(&level),
        get_flags(&level),
    );
}
