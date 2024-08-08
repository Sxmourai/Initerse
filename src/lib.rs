// #![cfg_attr(debug_assertions, allow(unused))]
// #![cfg_attr(debug_assertions, warn(unused_results))]
#![allow(unused_mut, dead_code, unused_variables, unused_imports)]
// Very usefull to initialise empty arrays
// + It's experimental because they are not 100% okay about semantics
#![feature(inline_const_pat)]
// #![warn(clippy::unused_async)]

use std::sync::{Arc, Mutex};

pub use macroquad::prelude::*;
pub use color_eyre::{Result,Report};
use tiles::{World, EMPTY_MACHINE};

pub mod tiles;
pub mod camera;
pub mod player;
pub mod hotbar;
pub mod build_mode;
pub mod tower;
use tower::{EmptyMachine, Tower};



pub async fn _main() -> Result<()> {
    tower::setup_cache_tower_textures().await?;
    let mut world = World::empty().await?;
    tiles::set_world(world);
    let mut world = get_world!();
    world.set_tower(ivec2(-1, -1), Tower::Electron.new_machine().unwrap());
    world.set_tower(ivec2(0, 0), Tower::StringCreator.new_machine().unwrap());
    world.set_tower(ivec2(1, 1), Tower::Electron.new_machine().unwrap());
    let mut hotbar = hotbar::Hotbar::new();
    let mut player = player::new();
    let mut build_mode = build_mode::BuildMode::new();
    loop {
        let dt = get_frame_time();
        
        player.update(dt);
        world.update(dt)?;
        
        world.draw(player.pos).await?;
        let on_hot = hotbar.draw(&mut build_mode).await?;
        build_mode.draw(&mut world, player.pos, on_hot).await?;
        world.interact(player.pos, &build_mode)?;
        world.control_tilesize()?;

        draw_text(&format!("X: {:.1} Y: {:.1}\nFPS: {:.1}", player.pos.x,player.pos.y, 1./dt), 20., 20., 32., WHITE);

        next_frame().await;
    }
}


pub fn vec2i(vec2f: Vec2) -> IVec2 {
    IVec2::new(vec2f.x as _, vec2f.y as _)
}
pub fn vec2i_to_f(vec2i: IVec2) -> Vec2 {
    Vec2::new(vec2i.x as _, vec2i.y as _)
}