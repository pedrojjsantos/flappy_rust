use ggez::ContextBuilder;
use ggez::event;

mod bird;
mod pipe;
mod game;

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

fn main() {
    let win_setup = ggez::conf::WindowSetup::default()
        .title("Flappy Rust")
        .vsync(true);
    let win_mode = ggez::conf::WindowMode::default()
        .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1);

    let (mut ctx, event_loop) = ContextBuilder::new("flappy_rust", "Jezzuz")
        .window_setup(win_setup)
        .window_mode(win_mode)
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = game::Game::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}