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
const PADDLE_W: f32 = 100.0;
const PADDLE_H: f32 = 10.0;
const BLOCK_W: f32 = WINDOW_W as f32 / 10.0;
const BLOCK_H: f32 = 20.0;

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

        let vel_x = rng.gen::<f32>();
        let vel_y = -5.0; //Starting Speed

        Ball {
            x: WINDOW_W as f32 / 2.0,
            y: WINDOW_H as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
            radius: 8.0,
        }
    }

    // reset the ball after loss of life
    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();

        self.x = WINDOW_W as f32 / 2.0;
        self.y = WINDOW_H as f32 / 2.0;
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

struct Paddle {
    x: f32,
    vel_x: f32,
    moving: bool,
}

impl Paddle {
    fn new(_ctx: &mut Context) -> Paddle {
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

#[derive(Debug)]
struct Block {
    x: f32,
    y: f32,
    life: i32,
}

impl Block {
    fn new(_ctx: &mut Context, xpos: f32, ypos: f32, hp: i32) -> Block {
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

struct MainState {
    ball: Ball,
    paddle: Paddle,
    blocks: Vec<Block>,
    level: i32,
    score: i32,
    lives: i32,
    level_text: graphics::Text,
    score_text: graphics::Text,
    lives_text: graphics::Text,
    font: graphics::Font,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let font_init = graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 18)?;
        let level_out = graphics::Text::new(_ctx, "level", &font_init)?;
        let score_out = graphics::Text::new(_ctx, "score", &font_init)?;
        let lives_out = graphics::Text::new(_ctx, "lives", &font_init)?;
        let s = MainState {
            ball: Ball::new(_ctx),
            paddle: Paddle::new(_ctx),
            blocks: Vec::new(),
            level: 1,
            score: 0,
            lives: 3,
            level_text: level_out,
            score_text: score_out,
            lives_text: lives_out,
            font: font_init,
        };
        Ok(s)
    }

    pub fn set_blocks(&mut self, _ctx: &mut Context) {
      self.blocks = Vec::new();
      let mut x = 0.0;
      let mut y = 80.0;

      for _ in 1..6 {
          for _ in 0..10 {
              self.blocks.push(Block::new(_ctx, x, y, self.level));
              x = x + BLOCK_W;
          }
          x = 0.0;
          y = y + BLOCK_H;
      }
    }

    pub fn collision(&mut self) {
        let ball_top = self.ball.y - self.ball.radius;
        let ball_bottom = self.ball.y + self.ball.radius;
        let ball_left = self.ball.x - self.ball.radius;
        let ball_right = self.ball.x + self.ball.radius;

        for b in &mut self.blocks {
            if b.life > 0 &&
               ((ball_top <= b.y + BLOCK_H &&
                 ball_top >= b.y) ||
                (ball_bottom >= b.y &&
                 ball_bottom <= b.y + BLOCK_H)) &&
               ((ball_left >= b.x &&
                 ball_left <= b.x + BLOCK_W) ||
                (ball_right >= b.x &&
                 ball_right <= b.x + BLOCK_W)) {

                b.life -= 1;
                if b.life == 0 {
                    self.score += 1;
                }
                self.ball.vel_y *= -1.0;

            }
        }

        if ((ball_top <= WINDOW_H as f32 - 40.0 &&
             ball_top >= WINDOW_H as f32 - 40.0 - PADDLE_H) ||
            (ball_bottom <= WINDOW_H as f32 - 40.0 &&
             ball_bottom >= WINDOW_H as f32 - 40.0 - PADDLE_H)) &&
           ((ball_left <= self.paddle.x + PADDLE_W &&
             ball_left >= self.paddle.x) ||
            (ball_right <= self.paddle.x + PADDLE_W &&
             ball_right >= self.paddle.x))     
        {
            if self.paddle.moving {
                self.ball.vel_x = self.paddle.vel_x / (2.0 as f32).sqrt();
            }
            self.ball.vel_y *= -1.0;
        }

        //Top
        if self.ball.y - self.ball.radius <= 0.0 {
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
        //Bottom
        if self.ball.y + self.ball.radius >= WINDOW_H as f32 {
            self.ball.reset();
            self.lives -= 1;
        }
    }

    pub fn update_ui(&mut self, _ctx: &mut Context) {
        let new_score = format!("Score: {}", self.score);
        let new_level = format!("Level: {}", self.level);
        let new_lives = format!("Lives: {}", self.lives);

        self.score_text = graphics::Text::new(_ctx, &new_score, &self.font).unwrap();
        self.level_text = graphics::Text::new(_ctx, &new_level, &self.font).unwrap();
        self.lives_text = graphics::Text::new(_ctx, &new_lives, &self.font).unwrap();
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.paddle.update();
        self.ball.update();
        self.collision();
        self.update_ui(_ctx);
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx);

        self.paddle.draw(_ctx)?;
        self.ball.draw(_ctx)?;

        for b in &mut self.blocks {
            if b.life > 0 {
                b.draw(_ctx)?;
            }
        }

        // Draw UI elements
        let score_pos = graphics::Point2::new(WINDOW_W as f32 - 150.0, 10.0);
        graphics::draw(_ctx, &self.score_text, score_pos, 0.0)?;

        let level_pos = graphics::Point2::new(10.0, 10.0);
        graphics::draw(_ctx, &self.level_text, level_pos, 0.0)?;

        let lives_pos = graphics::Point2::new(10.0, WINDOW_H as f32 - 30.0);
        graphics::draw(_ctx, &self.lives_text, lives_pos, 0.0)?;

        graphics::present(_ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Left | Keycode::A => {
                self.paddle.move_left();
            }
            Keycode::Right | Keycode::D => {
                self.paddle.move_right();
            }
            Keycode::Escape => {
                _ctx.quit().unwrap();
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
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
    state.set_blocks(ctx);
    event::run(ctx, state).unwrap();
}
