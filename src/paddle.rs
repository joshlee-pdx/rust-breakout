use ggez::graphics::{self, set_color, DrawMode};
use ggez::{Context, GameResult};

const WINDOW_W: u32 = 400;
const WINDOW_H: u32 = 600;

const PADDLE_W: f32 = 100.0;
const PADDLE_H: f32 = 10.0;

pub struct Paddle {
    pub x: f32,
    pub vel_x: f32,
    pub moving: bool,
}

impl Paddle {
    pub fn new(_ctx: &mut Context) -> Paddle {
        Paddle {
            x: WINDOW_W as f32 / 2.0 - PADDLE_W / 2.0, //Centered
            vel_x: 0.0,
            moving: false,
        }
    }

    pub fn update(&mut self) {
        if self.moving {
            self.x += self.vel_x;
        }
        if self.x <= 0.0 {
            self.x = 0.0;
        }
        if self.x + PADDLE_W >= WINDOW_W as f32 {
            self.x = WINDOW_W as f32 - PADDLE_W;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;

        let rect = graphics::Rect::new(
            self.x,
            WINDOW_H as f32 - 40.0 - PADDLE_H,
            PADDLE_W,
            PADDLE_H,
        );
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }

    pub fn move_left(&mut self) {
        self.vel_x = -10.0;
        self.moving = true;
    }

    pub fn move_right(&mut self) {
        self.vel_x = 10.0;
        self.moving = true;
    }

    pub fn stop(&mut self) {
        self.vel_x = 0.0;
        self.moving = false;
    }
}
