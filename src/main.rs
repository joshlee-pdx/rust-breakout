//Bouncing ball

extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, set_color, DrawMode, Point2};
use ggez::{Context, ContextBuilder, GameResult};
use std::{env, path};

use rand::Rng;

const WINDOW_W: u32 = 400;
const WINDOW_H: u32 = 600;
const PADDLE_W: f32 = 80.0;
const PADDLE_H: f32 = 10.0;

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    radius: f32,
}

impl Ball {
    fn new(_ctx: &mut Context) -> Ball {
        let mut rng = rand::thread_rng();

        let mut vel_x = rng.gen::<f32>(); //This will change based on paddle velocity
        let vel_y = 5.0; //Starting Speed

        Ball {
            x: WINDOW_W as f32 / 2.0,
            y: WINDOW_H as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
            radius: 8.0,
        }
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

struct Paddle {
    x: f32,
    vel_x: f32,
    moving: bool,
}

impl Paddle {
    fn new(_ctx: &mut Context) -> Paddle {
        Paddle {
            x: 300.0,
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
            WINDOW_H as f32 - 10.0 - PADDLE_H,
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

struct MainState {
    ball: Ball,
    paddle: Paddle,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            ball: Ball::new(_ctx),
            paddle: Paddle::new(_ctx),
        };
        Ok(s)
    }

    pub fn collision(&mut self) {
        //Top
        if self.ball.y - self.ball.radius <= 0.0 {
            self.ball.vel_y *= -1.0;
        }
        //Bottom
        if self.ball.y + self.ball.radius >= WINDOW_H as f32 {
            self.ball.vel_y *= -1.0;
        }
        //Left
        if self.ball.x - self.ball.radius < 0.0 {
            self.ball.vel_x *= -1.0;
        }
        //Right
        if self.ball.x + self.ball.radius > WINDOW_W as f32 {
            self.ball.vel_x *= -1.0;
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.paddle.update();
        self.ball.update();
        self.collision();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx);

        self.paddle.draw(_ctx)?;
        self.ball.draw(_ctx)?;

        graphics::present(_ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::Left | Keycode::A => {
                self.paddle.move_left();
            }
            Keycode::Right | Keycode::D => {
                self.paddle.move_right();
            }
            Keycode::Escape => _ctx.quit().unwrap(),
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left | Keycode::A | Keycode::Right | Keycode::D => {
                self.paddle.stop();
            }
            _ => {}
        }
    }
}

pub fn main() {
    let mut cb = ContextBuilder::new("Rust Breakout!", "ggez")
        .window_setup(conf::WindowSetup::default().title("Breakout!"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_W, WINDOW_H));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources/");
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
