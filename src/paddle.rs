// This is the paddle.rs file for the classic game Breakout,
// which is responsible for defining the paddle the player controls

// Use specific parts of the ggez crate
use ggez::graphics::{self, set_color, DrawMode};
use ggez::{Context, GameResult};

// Struct to define the state of the paddle in the game
pub struct Paddle {
    pub x: f32,
    pub vel_x: f32,
    pub moving: bool,
}

impl Paddle {
    // Function to setup the initial state of the paddle in the game
    pub fn new(_ctx: &mut Context) -> Paddle {
        Paddle {
            x: ::WINDOW_W as f32 / 2.0 - ::PADDLE_W / 2.0, //Centered
            vel_x: 0.0,
            moving: false,
        }
    }

    // Function to update the paddle which is called every time we update the main state
    pub fn update(&mut self) {
        if self.moving {
            self.x += self.vel_x;
        }
        if self.x <= 0.0 {
            self.x = 0.0;
        }
        if self.x + ::PADDLE_W >= ::WINDOW_W as f32 {
            self.x = ::WINDOW_W as f32 - ::PADDLE_W;
        }
    }

    // Function to render the paddle in the game
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;

        let rect = graphics::Rect::new(
            self.x,
            ::WINDOW_H as f32 - ::PADDLE_PADDING - ::PADDLE_H,
            ::PADDLE_W,
            ::PADDLE_H,
        );
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }

    // Functions to move or stop the paddle
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
