// This is the block.rs file for the classic game Breakout,
// which is responsible for defining the blocks that the player
// gets to break.

// Use specific parts of the ggez crate
use ggez::graphics::{self, set_color, DrawMode};
use ggez::{Context, GameResult};

#[derive(Debug)]
// Struct to define the state of blocks in the game
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub life: i32,
}

impl Block {
    // Function to setup the initial state of the blocks in the game
    pub fn new(_ctx: &mut Context, xpos: f32, ypos: f32, hp: i32) -> Block {
        Block {
            x: xpos,
            y: ypos,
            life: hp,
        }
    }

    // Function to render the blocks in the game
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, [0.0, 0.0, 0.0, 1.0].into())?;
        let mut rect = graphics::Rect::new(self.x, self.y, ::BLOCK_W, ::BLOCK_H);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;

        // Different shades of red, like a brick wall
        // Based on 0to255.com
        match self.life {
            1 => set_color(ctx, [0.82, 0.22, 0.27, 1.0].into())?,
            2 => set_color(ctx, [0.73, 0.17, 0.22, 1.0].into())?,
            3 => set_color(ctx, [0.57, 0.13, 0.17, 1.0].into())?,
            4 => set_color(ctx, [0.52, 0.12, 0.15, 1.0].into())?,
            5 => set_color(ctx, [0.41, 0.10, 0.12, 1.0].into())?,
            6 => set_color(ctx, [0.30, 0.07, 0.09, 1.0].into())?,
            _ => set_color(ctx, [0.19, 0.05, 0.06, 1.0].into())?,
        }

        rect = graphics::Rect::new(self.x, self.y, ::BLOCK_W - 1.0, ::BLOCK_H - 1.0);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }
}
