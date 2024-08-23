// #![cfg_attr(debug_assertions, allow(unused))]
// #![cfg_attr(debug_assertions, warn(unused_results))]
#![allow(unused_mut, dead_code, unused_variables, unused_imports)]
// Very usefull to initialise empty arrays
// + It's experimental because they are not 100% okay about semantics
#![feature(inline_const_pat)]
#![feature(get_mut_unchecked)]
// #![warn(clippy::unused_async)]

use std::{fs::read_to_string, path::PathBuf, sync::{Arc, Mutex}};

use config::Config;
pub use macroquad::prelude::*;
pub use color_eyre::{Result,Report};
use miniquad::window::order_quit;
use tiles::{World, EMPTY_MACHINE};

pub mod tiles;
pub mod camera;
pub mod player;
pub mod hotbar;
pub mod build_mode;
pub mod tower;
pub mod celestial;
pub mod gui;
pub mod config;

use tower::{EmptyMachine, Tower};
use gui::*;


pub async fn _main() -> Result<()> {
    tower::setup_cache_tower_textures().await?;
    unsafe { config::CONFIG.set(Config::get()).unwrap() }
    loop {
        if button(Rect::new(screen_width()/2.0-100., screen_height()/2.0-100., 200., 50.), "New world", 32., DARKGRAY) {
            new_world_scene().await?;
        }
        if button(Rect::new(screen_width()/2.0-100., screen_height()/2.0, 200., 50.), "Load world", 32., DARKGRAY) {
            load_world_scene().await?;
        }
        if button(Rect::new(screen_width()/2.0-100., screen_height()/2.0+100., 200., 50.), "Options", 32., DARKGRAY) {
            options_scene().await?;
        }
        if button(Rect::new(screen_width()/2.0-100., screen_height()-100., 200., 50.), "Quit", 32., DARKGRAY) {
            break
        }
        next_frame().await;
    }
    Ok(())
}
async fn new_world_scene() -> Result<()> {
    let seed = format!("{}", ::rand::random::<u64>());
    let mut seed_inp = TextBox::new(seed, Rect::new(screen_width()/2.0-100., screen_height()/2.0-100., 200., 50.), DARKBLUE);
    loop {
        seed_inp.update();
        if button(Rect::new(screen_width()/2.0-100., screen_height()/2.0+100., 200., 50.), "Play", 32., DARKGRAY) {
            let mut seed_n = 0;
            for b in seed_inp.text.bytes() {
                seed_n += b as u64;
            }
            println!("Generating world with seed: {}", seed_n);
            rand::srand(seed_n);
            let mut world = World::new(seed_n).await;
            world.set_tower(ivec2(-1, -1), Tower::Electron.new_machine().unwrap());
            // world.set_tower(ivec2(0, 0), Tower::StringCreator.new_machine().unwrap());
            world.set_tower(ivec2(1, 1), Tower::Electron.new_machine().unwrap());
            return game_loop(world).await
        }
        seed_inp.draw();

        next_frame().await;
    }
}
fn get_saves() -> Vec<PathBuf> {
    let dir = match std::fs::read_dir("saves") {
        Ok(dir) => dir,
        Err(e) => {
            miniquad::warn!("Can't open saves folder ! {:?}", e);
            return vec![]
        },
    };
    let mut saves = vec![]; // Vec::with_capacity(dir.count())
    for save in dir {
        if let Ok(save) = save {
            saves.push(save.path())
        }
    }
    saves
}
async fn load_world_scene() -> Result<()> {
    let saves = get_saves();
    loop {
        for (i,save) in saves.iter().enumerate() {
            if button(Rect::new(screen_width()/2.0-100., screen_height()/2.0-200.+75.*i as f32, 200., 50.), &format!("{:?}", save), 32., DARKGRAY) {
                let mut world = World::load(read_to_string(save)?).await?;
                return game_loop(world).await
            }
        }

        next_frame().await;
    }
}
async fn options_scene() -> Result<()> {
    let mut config_menu = config::ConfigMenu::new();
    loop {
        if config_menu.update() {
            return Ok(())
        }

        next_frame().await;
    }
}
async fn game_options_scene() -> Result<()> {
    let mut config_menu = config::ConfigMenu::new();
    loop {
        if config_menu.update() {
            return Ok(())
        }
        if button(Rect::new(10., screen_height()-100., 200., 50.), "Save & Quit", 32., DARKGRAY) {
            get_world!().save().unwrap();
            miniquad::window::order_quit();
        }
        if button(Rect::new(screen_width()-200.0-10., screen_height()-100., 100., 50.), "Force Quit", 32., DARKGRAY) {
            miniquad::window::order_quit();
        }

        next_frame().await;
    }
}
async fn game_loop(world: World) -> Result<()> {
    prevent_quit();
    tiles::set_world(world);
    let mut world = get_world!();
    let mut hotbar = hotbar::Hotbar::new();
    let mut player = player::new();
    let mut build_mode = build_mode::BuildMode::new();
    loop {
        if is_quit_requested() {
            unsafe { config::CONFIG.get().unwrap().write().unwrap() }
            world.save().unwrap();
            order_quit();
            return Ok(())
        }
        let dt = get_frame_time();
        if is_key_down(KeyCode::Escape) {
            game_options_scene().await?;
        }

        player.update(dt);
        world.update(player.pos, dt)?;
        
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




// Based on quad_rand, but we can't get there seed
/// returns a pseudo-random number in the range of 0 to u32::MAX and the new seed
pub fn rand_with_seed(seed: u64) -> (u32, u64) {
    let xorshifted: u32 = (((seed >> 18) ^ seed) >> 27) as u32;
    let rot: u32 = (seed >> 59) as u32;
    (xorshifted.rotate_right(rot), seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407))
}