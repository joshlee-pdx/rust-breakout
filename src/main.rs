/*
Main file for Breakout game using the ggez engine.

TODO: Create Paddle for player, create blocks for enemies, place blocks and paddle
*/

extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
//use ggez::timer;

use std::env;
use std::path;

use std::time::Duration;

const BLOCK: f32 = 32.0;

const WIDTH: u32 = BLOCK as u32 * 25;
const HEIGHT: u32 = BLOCK as u32 * 20;

/* ======================== WORK IN PROGRESS ======================== */
// Defining the player, blocks, and ball
struct Player{
    x: f32,
    y: f32,
    vel_x: f32,
    moving: bool,
}

impl Player {
    fn new(/*_ctx: &mut Context*/) -> Player {
        Player {
            x: 400.0
            y: 10.0,
            vel_x: 0.0,
            moving: false,
        }
    }

    pub fn draw(/*&mut self, ctx: &mut Context*/) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x, self.y, 32.0, 100.0);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }
}

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
        let mut vel_x = rng.gen::<f32>();
        vel_y -= 2.0;
        let vel_x = rng.gen::<f32>();

        Ball {
            x: WINDOW_W as f32 / 2.0,
            y: WINDOW_H as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
            radius: 10.0,
        }
    }

    pub fn update(&mut self) {
        // called every frame
        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dst = Point2::new(self.x, self.y);
        graphics::circle(ctx, DrawMode::Fill, dst, self.radius, 1.0)?;
        Ok(())
    }
}

struct Block {
    x: f32,
    y: f32,
    visible: bool,
}

impl Block {

}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(key: Keycode) -> Option<Direction> {
        match key {
            Keycode::Left => Some(Direction::Left),
            Keycode::Right => Some(Direction::Right),
            _ => None
        }
    }
}
/* ================================================================== */ 

// Structure to contain the game's state
struct MainState {
    //enemies: u32,
    //lives: i32,
    //text: graphics::Text,

    // First we need the player (paddle)
    player: Player,
    // Next need the blocks
    block: Block,
    // And lastly the ball
    ball: Ball,

    gameover: bool,

    score:u32,
    score_changed: bool,
    score_display: graphics::Text,
    frames: usize,
}

impl MainState {
    fn new (ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory
        // Which will be mounted later, so no need to path it here

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf",48)?;
        let text = graphics::Text::new(ctx, &"Start", &font)?;

        let s = MainState {
            //enemies: 0,
            //lives: 1,

            player: Player::new(player_pos),
            block: Block::new(block_pos),
            ball: Ball::new(ball_pos),

            gameover: false,

            score:0,
            score_changed: true,
            score_display: text,
            frames:0,
            };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
        fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
            // Check if victory

            //Check Collisions

            //Run update for objects

            // Update Score
            if self.score_changed {
                let font = graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 18)?;
                //let text_to_display = format!("Score: {} Lives: {}", self.score, self.lives);
                let text_to_display = format!("Score: {}", self.score);
                let text = graphics::Text::new(_ctx, &text_to_display, &font)?;

                self.score_display = text;
                self.score_changed = false;
                }

        Ok(())
    }

    // Draw current mainstate
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //Clear the screen first
        graphics::clear(ctx);

        // Display score
        graphics::set_color(ctx, graphics::WHITE)?;
        let dest_point = graphics::Point2::new(50.0, 20.0);
        graphics::draw(ctx, &self.score_display, dest_point, 0.0)?;
        //Tell the paddle and blocks where to draw themselves

        // Check if dead?

        graphics::present(ctx);

        ggez::timer::yield_now();

        Ok(())
    }
}
    // Now our main function, which does three things:
    //
    // * First, create a new `ggez::conf::Conf`
    // object which contains configuration info on things such
    // as screen resolution and window title.
    // * Second, create a `ggez::game::Game` object which will
    // do the work of creating our MainState and running our game.
    // * Then, just call `game.run()` which runs the `Game` mainloop.
fn main() {
    // Window settings
    // Game settings
    // Configuration Settings
    let c = conf::Conf::new();
    let ctx = &mut ggez::ContextBuilder::new("breakout", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Breakout!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(HEIGHT, WIDTH))
        .build().expect("Failed to build ggez context");

    graphics::set_background_color(ctx, [0.0, 0.0, 0.0, 0.0].into());

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.

    /*if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
           let mut path = path::PathBuf::from(manifest_dir);
           path.push("resources");
           ctx.filesystem.mount(&path, true);
       }
    */

       let state = &mut MainState::new(ctx).unwrap();

       if let Err(e) = event::run(ctx, state) {
           println!("Error encountered: {}", e);
       } else {
           println!("Game exited cleanly.");
       }
}
