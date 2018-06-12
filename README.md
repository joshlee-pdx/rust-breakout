# rust-breakout (v 0.1.1)
### About
Our take on the classic game Breakout implemented in rust using [ggez](https://github.com/ggez/ggez).

The player moves a paddle across the bottom of the screen to keep the ball in play. When the ball collides with a brick the brick takes damage until it is broken. If the ball touches the bottom of the screen the player loses a life. The player wins the level by clearing all bricks from the screen.

### Authors
* Josh Lee <<joshulee@pdx.edu>>
* Geoff Maggi <<gmaggi@pdx.edu>>
* Miguel Delapaz <<delapaz@pdx.edu>>

### How to Run
1. Download the source code
2. The SDL2 libraries are required.  The best way to install them is documented [by the SDL2 crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).
3. `cargo run` to play the game
---------------------------------------

CS 410P/510 - RUST Programming Spring 2018  
*Portland State University © 2018*
