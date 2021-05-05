use std::ops::Range;

use ggez::{Context, GameResult, graphics};
use oorandom::Rand32;
use glam::Vec2;

use crate::{SCREEN_SIZE, game::BIRD_SPEED as SPEED};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up(f32),
    Down(f32),
}

pub struct Bird {
    pos: Vec2,
    size: f32,
    dir: Direction,
}
impl Bird {
    pub fn new(size: f32) -> Self {
        Bird {
            pos: Vec2::new(100.0, 300.0),
            size,
            dir: Direction::Down(0.0),
        }
    }

    pub fn jump(&mut self) {
        self.dir = Direction::Up(SPEED);
    }

    pub fn is_on_screen(&self) -> bool {
        self.pos.y >= 0.0 && self.pos.y + self.size <= SCREEN_SIZE.1
    }

    pub fn is_colliding(&self, pipe: &Pipe) -> bool {
        if (self.pos.x + self.size) >= pipe.pos_x() && self.pos.x <= (pipe.pos_x() + pipe.width) {
            if self.pos.y <= pipe.h_upper || self.pos.y + self.size >= pipe.lower.y {
                return true;
            }
        }

        false
    }

    /// Deal with the bird's movement
    pub fn update(&mut self, dt: std::time::Duration) {
        match self.dir {
            Direction::Up(s) => {
                self.pos.y -= s * dt.as_secs_f32() * 1.5;
                self.dir = Direction::Up(s - SPEED * dt.as_secs_f32() * 2.5);

                if s < 100.0 { self.dir = Direction::Down(0.0); } 
            }
            Direction::Down(s) => {
                self.pos.y += s * dt.as_secs_f32() * 2.0;
                self.dir = Direction::Down(s + SPEED * dt.as_secs_f32() * 2.0);
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(
            self.pos.x,
            self.pos.y,
            self.size,
            self.size
        );
        
        let rectangle = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            rect,
            [0.7, 0.7,0.0,1.0].into()
        )?;

        graphics::draw(ctx, &rectangle, ([0.0, 0.0],))?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pipe {
    upper: Vec2,
    lower: Vec2,
    width: f32,
    h_upper: f32,
    h_lower: f32,
}
impl From<&Pipe> for (graphics::Rect, graphics::Rect) {
    fn from(pipe: &Pipe) -> Self {
        let upper = graphics::Rect::new(
            pipe.upper.x,
            pipe.upper.y,
            pipe.width,
            pipe.h_upper
        );
        let lower = graphics::Rect::new(
            pipe.lower.x,
            pipe.lower.y,
            pipe.width,
            pipe.h_lower
        );

        (upper, lower)
    }
}
impl Pipe {
    pub fn new(width: f32, gap: f32, rng: &mut Rand32, height_range: Range<u32>) -> Self {
        let y_lower = rng.rand_range(height_range) as f32;
        let y_upper = 0.0;

        let h_lower = SCREEN_SIZE.1 - y_lower; 
        let h_upper = y_lower - gap;

        Pipe {
            upper: Vec2::new(SCREEN_SIZE.0, y_upper),
            lower: Vec2::new(SCREEN_SIZE.0, y_lower),
            width,
            h_upper,
            h_lower,
        }
    }

    pub fn pos_x(&self) -> f32 { self.upper.x }
    
    pub fn move_foward(&mut self, dt: std::time::Duration) {
        self.upper.x -= 300.0 * dt.as_secs_f32();
        self.lower.x -= 300.0 * dt.as_secs_f32();
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let (upper, lower) = self.into();

        let rect_upper = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            upper, 
            [0.0, 1.0, 0.0, 1.0].into()
        )?;
        
        let rect_lower = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            lower, 
            [0.0, 1.0, 0.0, 1.0].into()
        )?;
        
        graphics::draw(ctx, &rect_upper, ([0.0,0.0],))?;
        graphics::draw(ctx, &rect_lower, ([0.0,0.0],))?;

        Ok(())
    }
}