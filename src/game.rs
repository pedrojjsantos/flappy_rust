use ggez::input::keyboard::{KeyInput, KeyCode};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect, Canvas};
use ggez::event::EventHandler;
use oorandom::Rand32;

use crate::SCREEN_SIZE;
use crate::pipe::Pipes;
use crate::bird::Bird;

const GAMEOVER_COLOR: [f32;4] = [1., 0., 0., 0.5];

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Starting,
    Running,
    Gameover,
}

pub struct Game {
    state: GameState,
    bird: Bird,
    pipes: Pipes
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let mut seed = [0;8];
        getrandom::getrandom(&mut seed).expect("Could not generate RNG seed");
        
        let rng = Rand32::new(u64::from_ne_bytes(seed));

        // Load/create resources such as images here.
        Game {
            state: GameState::Starting,
            bird: Bird::new(),
            pipes: Pipes::new(rng),
        }
    }

    fn change_state_to(&mut self, new_state: GameState) {
        self.state = new_state;
    }

    fn jump(&mut self) {
        self.bird.jump()
    }

    fn restart(&mut self) {
        self.change_state_to(GameState::Starting);
        self.bird = Bird::new();
        self.pipes.clear();
    }

    fn is_not_colliding(&self) -> bool {
        !self.pipes.is_colliding(self.bird.position())
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.state == GameState::Running {
            
            if self.bird.is_on_screen() && self.is_not_colliding() {
                self.bird.update(ctx);
                self.pipes.update(ctx);
            } else {
                self.change_state_to(GameState::Gameover);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        use GameState::*;

        let mut canvas: Canvas;

        if self.state == Starting {
            canvas = Canvas::from_frame(ctx, Color::BLACK);
            let text = graphics::Text::new("Press Space to start!");
            canvas.draw(&text, DrawParam::default().z(1));
            return canvas.finish(ctx);
        }

        canvas = Canvas::from_frame(ctx, Color::CYAN);
        canvas.draw(&self.bird.get_mesh(ctx)?, DrawParam::default().z(1));

        if let Ok(meshs) = self.pipes.get_meshes(ctx) {
            for mesh in meshs {
                canvas.draw(&mesh, DrawParam::default().z(1))
            } 
        }

        if self.state == Gameover {
            let screen = Rect::new(0., 0., SCREEN_SIZE.0, SCREEN_SIZE.1);
            let gameover_box = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), screen, GAMEOVER_COLOR.into())?;

            canvas.draw(&gameover_box, DrawParam::default().z(2));
        }

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool,) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            match (keycode, &mut self.state) {
                (KeyCode::Space, GameState::Starting) => self.change_state_to(GameState::Running),
                (KeyCode::Space, GameState::Running)  => self.jump(),
                (KeyCode::Space, GameState::Gameover) => self.restart(),

                (KeyCode::Escape, _) => ctx.request_quit(),
                _ => {},
            }
        }

        Ok(())
    }
}