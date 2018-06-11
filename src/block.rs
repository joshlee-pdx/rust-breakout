use ggez::graphics::{self, set_color, DrawMode};
use ggez::{Context, GameResult};

const BLOCK_W: f32 = 40.0;
const BLOCK_H: f32 = 20.0;

#[derive(Debug)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub life: i32,
}

impl Block {
    pub fn new(_ctx: &mut Context, xpos: f32, ypos: f32, hp: i32) -> Block {
        Block {
            x: xpos,
            y: ypos,
            life: hp,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, [0.0, 0.0, 0.0, 1.0].into())?;
        let mut rect = graphics::Rect::new(self.x, self.y, BLOCK_W, BLOCK_H);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;

        //Different shades of red, like a brick wall
        //Based on 0to255.com
        match self.life {
            1 => set_color(ctx, [0.82, 0.22, 0.27, 1.0].into())?,
            2 => set_color(ctx, [0.73, 0.17, 0.22, 1.0].into())?,
            3 => set_color(ctx, [0.57, 0.13, 0.17, 1.0].into())?,
            4 => set_color(ctx, [0.52, 0.12, 0.15, 1.0].into())?,
            5 => set_color(ctx, [0.41, 0.10, 0.12, 1.0].into())?,
            6 => set_color(ctx, [0.30, 0.07, 0.09, 1.0].into())?,
            _ => set_color(ctx, [0.19, 0.05, 0.06, 1.0].into())?,
        }

        rect = graphics::Rect::new(self.x, self.y, BLOCK_W - 1.0, BLOCK_H - 1.0);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }
}
