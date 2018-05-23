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

// Structure to contain the game's state
struct MainState {
    text: graphics::Text,
    frames: usize,
}

impl MainState {
    fn new (ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory
        // Which will be mounted later, so no need to path it here

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf",48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;

        let s = MainState {
            text,
            frames:0,
            };
    };
        Ok(s)
}

impl event::EventHandler for MainState {
            fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
                Ok(())
            }
            
    }

fn main(){
    // Configuration Settings
    let c = conf::Conf::new();

    // Window settings

    // Game settings




}
