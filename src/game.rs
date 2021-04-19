use std::{collections::LinkedList, time};
use ggez::{graphics, Context, event::{EventHandler, KeyCode, KeyMods}};
use oorandom::Rand32;

use crate::{SCREEN_SIZE, game_objects::{Bird, Pipe}};

pub const BIRD_SPEED: f32 = 400.0;

pub const BIRD_WIDTH: f32 = 30.0;

pub const PIPE_WIDTH: f32 = 50.0;
// Distance between pipes
pub const PIPE_DISTANCE: f32 = 200.0;
// Vertical Gap between two pipes 
pub const PIPE_GAP: f32 = 200.0;


struct Menu {
    background: [f32;4],
    foreground: [f32;4],

    color_offset: f32,

    text: graphics::Text,
}
impl Menu {
    fn new() -> Self{
        let frag = graphics::TextFragment::new("Press Space to start!").scale(40.0.into());
        let text = graphics::Text::new(frag);

        Menu {
            background: [0.0, 0.0, 0.0, 0.0],
            foreground: [0.8, 0.8, 0.8, 1.0],
            color_offset: 0.01,
            text,
        }
    }

    fn change_foreground_color(&mut self) {
        if self.foreground[0] >= 0.8 || self.foreground[0] <= 0.4 {
            self.color_offset *= -1.0;
        }

        for i in self.foreground.iter_mut() {
            *i += self.color_offset;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, self.background.into());

        let text_x = (SCREEN_SIZE.0 - self.text.width(ctx)) / 2.0;

        let param = graphics::DrawParam::default()
            .dest([text_x, SCREEN_SIZE.1 / 2.0])
            .color(self.foreground.into());

        graphics::draw(
            ctx,
            &self.text,
            param
            )?;
        graphics::present(ctx)?;
        
        Ok(())
    }
}

enum GameOption {
    StartScreen,
    GameOver(time::Instant),
    Running,
    Paused,
}

pub struct GameState {
    menu: Menu,
    bird: Bird,
    pipes: LinkedList<Pipe>,
    
    rng: Rand32,

    state: GameOption,
}
impl GameState {
    pub fn new() -> Self {
        let rng = Rand32::new(12345);
        
        GameState {
            menu: Menu::new(),
            bird: Bird::new(BIRD_WIDTH),
            pipes: LinkedList::new(),
            rng,
            state: GameOption::StartScreen,
        }
    }
}



impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let GameOption::StartScreen = self.state {
            self.menu.change_foreground_color();
            return Ok(());
        }
        else if let GameOption::GameOver(t) = self.state {
            
        }
        // Checks if the first pipe is outside of the screen and deletes it
        if let Some(p) = self.pipes.front() {
            if p.pos_x() < -PIPE_WIDTH {
                self.pipes.pop_front();
            }
        }
        // if there is no pipe creates one offscreen
        else {
            let pipe = Pipe::new(
                PIPE_WIDTH,
                PIPE_GAP,
                &mut self.rng,
                100 + PIPE_GAP as u32 .. 500
            );

            self.pipes.push_back(pipe);
        }

        // Checks if the last pipe has moved enough to spawn a new one
        if let Some(p) = self.pipes.back() {
            if p.pos_x() < SCREEN_SIZE.0 - PIPE_DISTANCE - PIPE_WIDTH {
                let pipe = Pipe::new(
                    PIPE_WIDTH,
                    PIPE_GAP,
                    &mut self.rng,
                    100 + PIPE_GAP as u32 .. 500
                );

                self.pipes.push_back(pipe);
            }
        }
        
        self.bird.update(ggez::timer::delta(ctx));

        // Moving the pipes to the left
        for i in self.pipes.iter_mut() {
            i.move_foward();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let GameOption::StartScreen = self.state {
            self.menu.draw(ctx)?;
            return Ok(());
        }
        graphics::clear(ctx, [0.0, 1.0, 1.0, 1.0].into());

        self.bird.draw(ctx)?;

        for i in self.pipes.iter() {
            i.draw(ctx)?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => {
                match self.state {
                    GameOption::Running     => self.bird.jump(),
                    GameOption::StartScreen => self.state = GameOption::Running,
                    _ => (),
                }
            },

            KeyCode::Escape => ggez::event::quit(ctx),
            
            _ => ()
        }
    }
}