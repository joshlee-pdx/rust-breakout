/*
Main file for Breakout game using the ggez engine.

NOTE: Initial main file is mostly a copy of the Hello_World example
from the ggez site.
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

// Structure to contain the game's state
struct MainState {
    enemies: u32,
    //lives: i32,
    //text: graphics::Text,
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
            enemies: 0,
            //lives: 1,
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
                let font = graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 22)?;
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
