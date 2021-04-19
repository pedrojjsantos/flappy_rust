extern crate ggez;
extern crate oorandom;
extern crate glam;

mod game;
mod game_objects;

use ggez::{ContextBuilder, GameResult};

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

fn main() -> GameResult {
    let win_setup = ggez::conf::WindowSetup::default()
        .title("Flappy Rust")
        .vsync(true);
    let win_mode = ggez::conf::WindowMode::default()
        .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1);

    let (ctx, event_loop) = ContextBuilder::new("flappy_rust", "jezzuz")
        .window_setup(win_setup)
        .window_mode(win_mode)
        .build()?;

    let state = game::GameState::new();

    ggez::event::run(ctx, event_loop, state);
}