#![no_std]

mod game;

use lazy_static::lazy_static;
use spin::Mutex;
use wasm4::palette;

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
