use ggez::{
    graphics::{DrawMode, GraphicsContext, Mesh, Rect},
    mint::Point2,
    GameResult,
};

use crate::SCREEN_SIZE;

const SIZE: f32 = 50.0;
const BIRD_COLOR: [f32;4] = [0.7, 0.7, 0.0, 1.0];
const JUMP_SPEED: f32 = 700.0;
const GRAVITY: f32 = JUMP_SPEED * 3.;

pub struct Bird {
    pos: Point2<f32>,
    size: f32,
    speed: f32,
}

impl Bird {
    pub fn new() -> Self {
        Bird {
            pos: [100., 300.].into(),
            size: SIZE,
            speed: 0.,
        }
    }

    pub fn get_mesh(&self, gfx: &impl ggez::context::Has<GraphicsContext>) -> GameResult<Mesh> {
        Mesh::new_rectangle(gfx, DrawMode::fill(), self.position(), BIRD_COLOR.into())
    }

    pub fn position(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.size, self.size)
    }

    pub fn jump(&mut self) {
        self.speed = JUMP_SPEED;
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {        
        let delta = ctx.time.delta().as_secs_f32();

        self.pos.y -= self.speed * delta;
        self.speed -= GRAVITY * delta;
    }

    pub fn is_on_screen(&self) -> bool {
        let (_ , heigth) = SCREEN_SIZE;
        let bird_pos = self.position();

        bird_pos.top() > 0. && bird_pos.bottom() < heigth
    }
}
