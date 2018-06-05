extern crate ggez;
extern crate rand;

use ggez::{GameResult, Context, ContextBuilder};
use ggez::graphics::{self, DrawMode, Point2, set_color};
use ggez::conf;
use ggez::event;
use std::{env, path};

use rand::Rng;

const WINDOW_W: u32 = 400;
const WINDOW_H: u32 = 600;

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
        let vel_y = 1.0; //Starting Speed

        Ball {
            x: WINDOW_W as f32 / 2.0,
            y: WINDOW_H as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
            radius: 10.0,
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

struct MainState {
    ball: Ball,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
          ball: Ball::new(_ctx),
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
        self.ball.update();
        self.collision();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.ball.draw(ctx)?;
        
        graphics::present(ctx);
        Ok(())
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
