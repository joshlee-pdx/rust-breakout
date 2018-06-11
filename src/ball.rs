extern crate rand;

use ggez::graphics::{self, set_color, DrawMode, Point2};
use ggez::{Context, GameResult};
use rand::Rng;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new(_ctx: &mut Context) -> Ball {
        let mut rng = rand::thread_rng();

        let vel_x = rng.gen::<f32>();
        let vel_y = -5.0; //Starting Speed

        Ball {
            x: ::WINDOW_W as f32 / 2.0,
            y: ::WINDOW_H as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
            radius: 8.0,
        }
    }

    // reset the ball after loss of life
    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();

        self.x = ::WINDOW_W as f32 / 2.0;
        self.y = ::WINDOW_H as f32 / 2.0;
        self.vel_x = rng.gen::<f32>();
        self.vel_y = -5.0;
    }

    //The ball moving
    pub fn update(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;

        let loc = Point2::new(self.x, self.y);
        graphics::circle(ctx, DrawMode::Fill, loc, self.radius, 1.0)?;
        Ok(())
    }
}
