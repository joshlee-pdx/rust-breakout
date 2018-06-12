extern crate ggez;
extern crate rand;

mod ball;
mod block;
mod levels;
mod paddle;

use ggez::conf;
use ggez::event;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{clear, draw, present, set_color, Font, Point2, Text};
use ggez::{Context, ContextBuilder, GameResult};
use std::{env, path};

const WINDOW_W: u32 = 400;
const WINDOW_H: u32 = 600;
const PADDLE_W: f32 = 100.0;
const PADDLE_H: f32 = 10.0;
const PADDLE_PADDING: f32 = 40.0; //The paddle's spacing from the bottom
const BLOCK_W: f32 = WINDOW_W as f32 / 10.0;
const BLOCK_H: f32 = 20.0;
const NUMBER_OF_LEVELS: i32 = 3;

use ball::Ball;
use block::Block;
use paddle::Paddle;

struct MainState {
    ball: Ball,
    paddle: Paddle,
    blocks: Vec<Block>,
    level: i32,
    score: i32,
    lives: i32,
    level_text: Text,
    score_text: Text,
    lives_text: Text,
    game_over: bool,
    font: Font,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let font_init = Font::new(_ctx, "/DejaVuSerif.ttf", 18)?;
        let level_out = Text::new(_ctx, "level", &font_init)?;
        let score_out = Text::new(_ctx, "score", &font_init)?;
        let lives_out = Text::new(_ctx, "lives", &font_init)?;
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
            game_over: false,
            font: font_init,
        };
        Ok(s)
    }

    pub fn set_blocks(&mut self, _ctx: &mut Context) {
        self.blocks = Vec::new();
        let mut x = 0.0;
        let mut y = 80.0;

        for i in 0..6 {
            for j in 0..10 {
                self.blocks
                    .push(Block::new(_ctx, x, y, levels::LEVELS[self.level as usize][i][j]));
                x = x + BLOCK_W;
            }
            x = 0.0;
            y = y + BLOCK_H;
        }
    }

    pub fn collision(&mut self) {
        let err_x = 1.5 * self.ball.vel_x.abs();
        let err_y = 1.5 * self.ball.vel_y.abs();

        let ball_top = self.ball.y - self.ball.radius;
        let ball_bottom = self.ball.y + self.ball.radius;
        let ball_left = self.ball.x - self.ball.radius;
        let ball_right = self.ball.x + self.ball.radius;

        /***** BLOCK COLLISION *****/
        for b in &mut self.blocks {
            if b.life > 0 {
                /* Top and Bottom*/
                if ((ball_top <= b.y + BLOCK_H && ball_top + err_y > b.y + BLOCK_H)
                    || (ball_bottom >= b.y && ball_bottom - err_y < b.y))
                    && (ball_right - err_x > b.x && ball_left + err_y < b.x + BLOCK_W)
                {
                    b.life -= 1;
                    if b.life == 0 {
                        self.score += 1;
                    }
                    self.ball.vel_y *= -1.0;
                }
                /* Left and Right */
                else if ((ball_right >= b.x && ball_right - err_x < b.x)
                    || (ball_left <= b.x + BLOCK_W && ball_left + err_x > b.x + BLOCK_W))
                    && (ball_bottom + err_y > b.y && ball_top - err_y < b.y + BLOCK_H)
                {
                    b.life -= 1;
                    if b.life == 0 {
                        self.score += 1;
                    }
                    self.ball.vel_x *= -1.0;
                }
            }
        }

        /***** PADDLE COLLISION *****/
        if (ball_bottom <= WINDOW_H as f32 - PADDLE_PADDING
            && ball_bottom + err_y >= WINDOW_H as f32 - PADDLE_PADDING)
            && ((ball_left <= self.paddle.x + PADDLE_W && ball_left >= self.paddle.x)
                || (ball_right <= self.paddle.x + PADDLE_W && ball_right >= self.paddle.x))
        {
            if self.paddle.moving {
                self.ball.vel_x += self.paddle.vel_x / (2.0 as f32).sqrt();
            }
            self.ball.vel_y *= -1.05; //Every hit makes it 5% faster
        }

        /***** EDGE COLLISION *****/
        //Top
        if ball_top <= 0.0 {
            self.ball.vel_y *= -1.0;
        }
        //Left
        if ball_left <= 0.0 {
            self.ball.vel_x = self.ball.vel_x.abs();
        }
        //Right
        if ball_right >= WINDOW_W as f32 {
            self.ball.vel_x = self.ball.vel_x.abs() * -1.0;
        }
        //Bottom
        if ball_bottom >= WINDOW_H as f32 {
            self.ball.reset();
            self.lives -= 1;
        }
    }

    pub fn update_ui(&mut self, _ctx: &mut Context) {
        let new_score = format!("Score: {}", self.score);
        let new_level = format!("Level: {}", self.level);
        let new_lives = format!("Lives: {}", self.lives);

        self.score_text = Text::new(_ctx, &new_score, &self.font).unwrap();
        self.level_text = Text::new(_ctx, &new_level, &self.font).unwrap();
        self.lives_text = Text::new(_ctx, &new_lives, &self.font).unwrap();
    }

    pub fn check_end_conditions(&mut self, _ctx: &mut Context){
        // Out of lives?
        if self.lives == 0 {
            self.game_over = true;
            return;
        }

        // Level Done?
        let mut blocks_gone = true;
        for b in &mut self.blocks {
            if b.life > 0 {
                blocks_gone = false;
                break;
            }
        }
        if blocks_gone {
          self.level += 1;
          if self.level > NUMBER_OF_LEVELS {
              self.game_over = true;
          }
          else {
              self.ball.reset();
              self.set_blocks(_ctx);
          }
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.game_over {
            self.paddle.update();
            self.ball.update();
            self.collision();
            self.check_end_conditions(_ctx);
            self.update_ui(_ctx);
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        clear(_ctx);

        if self.game_over {
            set_color(_ctx, [1.0, 1.0, 1.0, 1.0].into())?; //White
            let text_pos = Point2::new(WINDOW_W as f32 / 3.0, WINDOW_H as f32 / 2.0);
            let go_text = Text::new(_ctx, "GAME OVER", &self.font).unwrap();
            draw(_ctx, &go_text, text_pos, 0.0)?;
            let score_pos = Point2::new(WINDOW_W as f32 / 3.0, (WINDOW_H as f32 / 2.0) + 40.0);
            draw(_ctx, &self.score_text, score_pos, 0.0)?;
            present(_ctx);
        }
        else {
            self.paddle.draw(_ctx)?;
            self.ball.draw(_ctx)?;

            for b in &mut self.blocks {
                if b.life > 0 {
                    b.draw(_ctx)?;
                }
            }

            // Draw UI elements
            set_color(_ctx, [1.0, 1.0, 1.0, 1.0].into())?; //White
            let score_pos = Point2::new(WINDOW_W as f32 - 150.0, 10.0);
            draw(_ctx, &self.score_text, score_pos, 0.0)?;

            let level_pos = Point2::new(10.0, 10.0);
            draw(_ctx, &self.level_text, level_pos, 0.0)?;

            let lives_pos = Point2::new(10.0, WINDOW_H as f32 - 30.0);
            draw(_ctx, &self.lives_text, lives_pos, 0.0)?;

            present(_ctx);
        }
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
