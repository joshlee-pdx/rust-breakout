// This is the ball.rs file for the classic game Breakout,
// which is responsible for defining the ball and how it moves

// Import the rand crate needed to move the ball
extern crate rand;

// Use specific parts of the ggez and rand crates
use ggez::graphics::{self, set_color, DrawMode, Point2};
use ggez::{Context, GameResult};
use rand::Rng;

// Constants to help calculating ball speeds
const MAX_VEL_X: f32 = 10.0;
const MAX_VEL_Y: f32 = 10.0;

// Struct to define the state of the ball in the game
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub radius: f32,
}

impl Ball {
    // Function to setup the initial state of the ball in the game
    pub fn new(_ctx: &mut Context) -> Ball {
        let mut rng = rand::thread_rng();

        let vel_x = rng.gen::<f32>();
        let vel_y = -5.0; // Starting Speed

        Ball {
            x: ::WINDOW_W as f32 / 2.0,
            y: ::WINDOW_H as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
            radius: 8.0,
        }
    }

    // Reset the ball after loss of life
    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();

        self.x = ::WINDOW_W as f32 / 2.0;
        self.y = ::WINDOW_H as f32 / 2.0;
        self.vel_x = rng.gen::<f32>();
        self.vel_y = -5.0;
    }

    // The ball moving
    pub fn update(&mut self) {
        if self.vel_x > MAX_VEL_X {
            self.vel_x = MAX_VEL_X;
        }
        if self.vel_x < -1.0 * MAX_VEL_X {
            self.vel_x = -1.0 * MAX_VEL_X;
        }
        if self.vel_y > MAX_VEL_Y {
            self.vel_y = MAX_VEL_Y;
        }
        if self.vel_y < -1.0 * MAX_VEL_Y {
            self.vel_y = -1.0 * MAX_VEL_Y;
        }

        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    // Function to render the ball
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;

        let loc = Point2::new(self.x, self.y);
        graphics::circle(ctx, DrawMode::Fill, loc, self.radius, 1.0)?;
        Ok(())
    }
}
