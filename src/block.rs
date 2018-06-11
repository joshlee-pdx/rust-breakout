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
        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;

        let rect = graphics::Rect::new(
            self.x,
            self.y,
            BLOCK_W,
            BLOCK_H,
        );
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }

}
