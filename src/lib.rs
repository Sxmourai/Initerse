// #![cfg_attr(debug_assertions, allow(unused))]
// #![cfg_attr(debug_assertions, warn(unused_results))]
#![allow(unused_mut, dead_code, unused_variables, unused_imports)]
// #![warn(clippy::unused_async)]

pub use macroquad::prelude::*;
pub use color_eyre::{Result,Report};
use tiles::{Tower, World, TILE_SIZE};

pub mod tiles;
pub mod camera;
pub mod player;



pub async fn _main() -> Result<()> {
    let mut world = World::empty().await?;
    world.set_tower(ivec2(5, 5), Tower::Electron);
    dbg!(world.get_tower(&ivec2(5, 5)));
    let mut player = player::new();
    loop {
        let dt = get_frame_time();
        
        player.update(dt);

        world.draw(player.pos).await?;

        draw_text(&format!("X: {:.1} Y: {:.1}", player.pos.x,player.pos.y), 20., 20., 32., WHITE);

        next_frame().await;
    }
}