use std::{collections::LinkedList};
use ggez::{
        graphics::{self, TextFragment},
        event::{EventHandler, KeyCode, KeyMods},
        Context,
        timer
};
use oorandom::Rand32;

use crate::{SCREEN_SIZE, game_objects::{Bird, Pipe}};


pub const BIRD_SPEED: f32 = 400.0;
pub const BIRD_SIZE: f32 = 30.0;
pub const PIPE_WIDTH: f32 = 50.0;

/// Distance between pipes
pub const PIPE_DISTANCE: f32 = 200.0;

/// Vertical Gap between two pipes 
pub const PIPE_GAP: f32 = 200.0;

/// Generic struct that keeps the state of a text menu
struct Menu {
    background: [f32;4],
    foreground: [f32;4],

    color_offset: f32,

    text: graphics::Text,
}
impl Menu {
    fn new(frag: TextFragment) -> Self{
        let text = graphics::Text::new(frag);

        Menu {
            background: [0.0, 0.0, 0.0, 0.0],
            foreground: [0.9, 0.9, 0.9, 1.0],
            color_offset: 0.4,
            text,
        }
    }

    /// Function to change the color of the text, making it blink
    fn change_foreground_color(&mut self, dt: std::time::Duration) {
        if self.foreground[0] >= 0.9 || self.foreground[0] <= 0.6 {
            self.color_offset *= -1.0;

            let x = if self.foreground[0] >= 0.9 {0.9} else {0.6};

            self.foreground[0] = x;
            self.foreground[1] = x;
            self.foreground[2] = x;
        }

        self.foreground[0] += self.color_offset * dt.as_secs_f32();
        self.foreground[1] += self.color_offset * dt.as_secs_f32();
        self.foreground[2] += self.color_offset * dt.as_secs_f32();
    }

    /// Draw the prompt at the menu
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        let text_pos = [
            (SCREEN_SIZE.0 - self.text.width(ctx)) / 2.0,
            (SCREEN_SIZE.1 - self.text.height(ctx)) / 2.0
        ];

        let param = graphics::DrawParam::default()
            .dest(text_pos)
            .color(self.foreground.into());

        graphics::draw(
            ctx,
            &self.text,
            param
            )?;
        
        Ok(())
    }
}

enum GameOption {
    StartScreen(Menu),
    Running,
    Paused(Menu),
    GameOver(f32, Menu),
}

pub struct GameState {
    bird: Bird,
    pipes: LinkedList<Pipe>,
    
    rng: Rand32,

    state: GameOption,
}
impl GameState {
    pub fn new() -> Self {
        let mut seed = [0;8];
        getrandom::getrandom(&mut seed).expect("Could not generate RNG seed");
        
        let rng = Rand32::new(u64::from_ne_bytes(seed));

        let frag = TextFragment::new("Press Space to start!").scale(40.0.into());
        let menu = Menu::new(frag);
        
        GameState {
            bird: Bird::new(BIRD_SIZE),
            pipes: LinkedList::new(),
            rng,
            state: GameOption::StartScreen(menu),
        }
    }

    fn restart(&mut self) {
        let mut seed = [0;8];
        getrandom::getrandom(&mut seed).expect("Could not generate RNG seed");

        let rng = Rand32::new(u64::from_ne_bytes(seed));

        self.rng   = rng;
        self.bird  = Bird::new(BIRD_SIZE);
        self.pipes = LinkedList::new();
        self.state = GameOption::Running;
    }
}



impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let GameOption::StartScreen(menu) = &mut self.state {
            menu.change_foreground_color(timer::delta(ctx));
            return Ok(());
        }
        if let GameOption::GameOver(sec, menu) = &mut self.state {
            if *sec > 0.0 {
                *sec -= timer::delta(ctx).as_secs_f32();
            } else {
                menu.change_foreground_color(timer::delta(ctx));
            }

            return Ok(());
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

        
        let dt = timer::delta(ctx);
        
        self.bird.update(dt);

        // Moving the pipes to the left
        for i in self.pipes.iter_mut() {
            i.move_foward(dt);
        }
        
        for p in self.pipes.iter() {
            if self.bird.is_colliding(p) || !self.bird.is_on_screen() {
                let frag = TextFragment::new("").scale(60.0.into());
                let menu = Menu::new(frag);

                self.state = GameOption::GameOver(5.0, menu);
                break;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let GameOption::StartScreen(menu) = &self.state {
            graphics::clear(ctx, menu.background.into());
            menu.draw(ctx)?;
            graphics::present(ctx)?;

            return Ok(());
        }

        graphics::clear(ctx, [0.0, 1.0, 1.0, 1.0].into());

        self.bird.draw(ctx)?;

        for i in self.pipes.iter() {
            i.draw(ctx)?;
        }

        if let GameOption::GameOver(sec, ref mut menu) = self.state {
            let screen = graphics::Rect::new(0.0, 0.0, SCREEN_SIZE.0, SCREEN_SIZE.1);
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                screen,
                [1.0, 0.0, 0.0, 0.7].into()
            )?;

            
            graphics::draw(ctx, &rectangle, ([0.0,0.0],))?;
            
            let sec = sec.ceil() as i8;
            let frag = menu.text.fragments_mut();
            let f;

            if sec > 0 {
                f = TextFragment::new(format!("{}", sec)).scale(120.0.into());
            } else {
                f = TextFragment::new("Press Space!").scale(60.0.into());
            }

            frag[0] = f;
            
            menu.draw(ctx)?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => {
                match self.state {
                    GameOption::Running     => self.bird.jump(),
                    GameOption::GameOver(s, _) => if s <= 0.0 { self.restart(); },
                    GameOption::StartScreen(_) => self.state = GameOption::Running,
                    _ => (),
                }
            },

            KeyCode::Escape => ggez::event::quit(ctx),
            
            _ => ()
        }
    }
}