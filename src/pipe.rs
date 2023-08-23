use ggez::{mint::Point2, context::Has, GameResult, graphics::{GraphicsContext, Mesh, DrawMode, Rect}};
use oorandom::Rand32;

use crate::SCREEN_SIZE;

const OFFSCREEN_X: f32 = SCREEN_SIZE.0 + PIPE_WIDTH;

const PIPE_COLOR: [f32;4] = [0.0, 0.8, 0.0, 1.0];
const PIPE_WIDTH: f32 = 100.;
const HEIGHT_GAP: f32 = 200.;
const PIPE_DISTANCE: f32 = 250. + PIPE_WIDTH;
const SPEED: f32 = 300.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pipe {
    pos: Point2<f32>,
    height: f32,
}

impl Pipe {
    pub fn new_pair(rng: &mut Rand32) -> (Self, Self) {
        let heigth_upper = rng.rand_range(100..300) as f32;

        let y_lower = heigth_upper + HEIGHT_GAP;
        let height_lower = SCREEN_SIZE.1 - y_lower;

        let upper = Pipe {
            pos: [OFFSCREEN_X, 0.0].into(),
            height: heigth_upper,
        };

        let lower = Pipe {
            pos: [OFFSCREEN_X, y_lower].into(),
            height: height_lower,
        };

        (upper, lower)
    }

    pub fn get_mesh(&self, gfx: &impl ggez::context::Has<GraphicsContext>) -> GameResult<Mesh> {
        Mesh::new_rectangle(gfx, DrawMode::fill(), self.position(), PIPE_COLOR.into())
    }

    pub fn position(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, PIPE_WIDTH, self.height)
    }

    pub fn get_x(&self) -> f32 {
        self.pos.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.pos.x = x;
    }
}

pub struct Pipes {
    head: usize,
    rng: Rand32,
    pipes: Vec<Pair>,
}

pub struct Pair {
    up: Pipe,
    down: Pipe,
} 
impl Pair {
    fn move_forward(&mut self, ctx: &mut ggez::Context) {
        let new_x = self.get_x() - (SPEED * ctx.time.delta().as_secs_f32());

        self.move_to(new_x)
    }

    fn get_x(&self) -> f32 {
        self.up.get_x()
    }

    fn move_to(&mut self, new_x: f32) {
        self.up.set_x(new_x);
        self.down.set_x(new_x);
    }

    fn is_offscreen(&self) -> bool {
        self.get_x() < -PIPE_WIDTH * 1.5
    }

    fn respawn(&mut self, rng: &mut Rand32, x: f32) {
        (self.up, self.down) = Pipe::new_pair(rng);
        self.move_to(x);
    }
} 
impl From<(Pipe, Pipe)> for Pair {
    fn from(value: (Pipe, Pipe)) -> Self {
        Pair { up: value.0, down: value.1 }
    }
}

impl Pipes {
    pub fn new(mut rng: Rand32) -> Self {
        let pipes = Pipes::create_pipes(&mut rng);

        Pipes {
            head: 0,
            rng,
            pipes,
        }
    }

    fn create_pipes(rng: &mut Rand32) -> Vec<Pair> {
        let pipes: Vec<Pair> = (1..=4)
            .map(|x| (x as f32) * PIPE_DISTANCE + OFFSCREEN_X)
            .map(|x| {
                let mut pair: Pair = Pipe::new_pair(rng).into();
                pair.move_to(x);
                pair
            })
            .collect();
        pipes
    }

    pub fn clear(&mut self) {
        let xs = (1..=self.len()).map(|x| (x as f32) * PIPE_DISTANCE + OFFSCREEN_X);
        
        self.pipes.iter_mut()
            .zip(xs)
            .for_each(|(p, x)| p.respawn(&mut self.rng, x));

        self.head = 0;
    }

    fn get(&self, i: usize) -> &Pair {
        let i = (self.head + i) % self.len();

        &self.pipes[i]
    }

    pub fn front(&self) -> &Pair {
        self.get(0)
    }

    pub fn back(&self) -> &Pair {
        self.get(self.len() - 1)
    }

    pub fn len(&self) -> usize {
        self.pipes.len()
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        self.pipes.iter_mut().for_each(|p| p.move_forward(ctx));

        if self.front().is_offscreen() {
            let offscreen = self.back().get_x() + PIPE_DISTANCE;

            self.pipes[self.head].respawn(&mut self.rng, offscreen);

            self.head = (self.head + 1) % self.len();
        }
    }

    pub fn get_meshes(&self, gfx: &impl Has<GraphicsContext>) -> GameResult<Vec<Mesh>> {
        self.pipes.iter()
            .flat_map(|pair| [pair.up, pair.down])
            .map(|p| p.get_mesh(gfx))
            .collect()
    }

    pub fn is_colliding(&self, rect: Rect) -> bool {
        self.pipes.iter()
            .flat_map(|p| [p.up, p.down])
            .map(|p| p.position())
            .any(|p| p.overlaps(&rect))
    }
}
