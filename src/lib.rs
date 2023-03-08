#![no_std]

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod game;
mod graphics;
mod hud;
mod maths;
mod palette;
mod player;
mod planet;
mod utils;
mod wasm4;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([0x000, 0xf9a875, 0xeb6b6f, 0x7c3f58]);
    GAME.lock().start();
}

#[no_mangle]
fn update() {
    GAME.lock().update();
}
