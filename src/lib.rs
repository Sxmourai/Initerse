// #![cfg_attr(debug_assertions, allow(unused))]
// #![cfg_attr(debug_assertions, warn(unused_results))]
#![allow(unused_mut, dead_code, unused_variables, unused_imports)]
// #![warn(clippy::unused_async)]

pub use macroquad::prelude::*;
pub use color_eyre::{Result,Report};
use tiles::{World, TILE_SIZE};

pub mod tiles;
pub mod camera;
pub mod player;



pub async fn _main() -> Result<()> {
    let mut world = World::empty().await?;
    let mut player = player::new();
    loop {
        let dt = get_frame_time();
        // clear_background(BLACK);
        // set_camera(&camera);
        player.update(dt);
        world.draw(player.pos)?;
        // set_default_camera();
        draw_text(&format!("X: {:.1} Y: {:.1}", player.pos.x,player.pos.y), 20., 20., 32., WHITE);

        next_frame().await;
    }
}