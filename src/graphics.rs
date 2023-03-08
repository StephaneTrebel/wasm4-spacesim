use fastrand::Rng;

use crate::{maths::Coordinates, wasm4::*};

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

pub fn draw_star(coords: &Coordinates) {
    pixel(coords.x as i32, coords.y as i32, 1);
}

pub fn draw_debris(coords: &Coordinates, rng: &Rng) {
    let delta_x = rng.i8(-1..1) as i32;
    let delta_y = rng.i8(-1..1) as i32;
    pixel(coords.x as i32 + delta_x, coords.y as i32 + delta_y, 1);
}
