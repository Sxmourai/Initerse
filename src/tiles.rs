
use std::{cell::{Cell, OnceCell, RefCell}, rc::Rc, sync::{Arc, Mutex, MutexGuard}};

use build_mode::BuildMode;
use celestial::{parse_celestials, Celestial};
use color_eyre::eyre::ContextCompat;
use tower::{EmptyMachine, Machine};

use super::*;

pub const BASE_TILE_SIZE: f32 = 48.;
pub const BASE_UPDATE_RADIUS: usize = 50;

pub type DynMachine = Rc<dyn Machine>;
pub fn new_machine(machine: impl Machine + 'static) -> DynMachine {
    Rc::new(machine)
}
thread_local! {
    pub static EMPTY_MACHINE: OnceCell<DynMachine> = OnceCell::new();
}
pub static mut WORLD: Option<World> = None;
#[macro_export]
macro_rules! get_world {
    () => {{
        unsafe { tiles::WORLD.as_mut().unwrap() }
    }};
}
pub fn set_world(world: World) {
    unsafe { tiles::WORLD.replace(world) };
}
pub type Map = hashbrown::HashMap<IVec2, DynMachine>;
pub const STAR_PARTICLE_MAX_LIFETIME: f32 = 10.;
pub const STAR_PARTICLE_MAX_AMOUNT: i32 = 200;
pub struct World {
    map: Map,
    seed: u64,
    enabled_gui: Option<IVec2>,
    tilesize: f32,
    update_radius: usize,
    celestials: Vec<(IVec2, Celestial)>,
    star_particle: Texture2D,
    star_particles: Vec<(Vec2,Vec2, Vec2, f32)>,
}
impl World {
    pub const fn tilesize(&self) -> f32 {self.tilesize}
    pub fn tiles_in_screen(&self) -> Vec2 {
        vec2(screen_width(), screen_height())/self.tilesize
    }
    pub async fn new(seed: u64) -> Self {
        let celestials = parse_celestials().await;
        Self { 
            seed,
            tilesize: BASE_TILE_SIZE, 
            update_radius: BASE_UPDATE_RADIUS,
            celestials: {
                let mut c = vec![];
                for celest in celestials {
                    c.push((ivec2(20, 0), celest));
                }
                c
            },
            map: std::default::Default::default(),
            enabled_gui: std::default::Default::default(),
            star_particle: load_texture("assets/star_particle.png").await.unwrap(),
            star_particles: {
                let mut star_particles = vec![];
                for n in 0..STAR_PARTICLE_MAX_AMOUNT {
                    let x = rand::gen_range(-20., 20.);
                    let y = rand::gen_range(-20., 20.);
                    let vx = rand::gen_range(-0.5, 0.5);
                    let vy = rand::gen_range(-0.5, 0.5);
                    let rot = macroquad::rand::gen_range(-360., 360.);
                    let rot_vel = macroquad::rand::gen_range(-1.0, 1.);
                    let lifetime = macroquad::rand::gen_range(0., 10.);
                    star_particles.push((vec2(x,y),vec2(vx,vy),vec2(rot,rot_vel), lifetime))
                }
                star_particles
            },
        }
    }
    pub fn set_tower(&mut self, coords: IVec2, machine: impl Into<DynMachine>) -> Option<DynMachine> {
        let machine = machine.into();
        if machine.ty() == Tower::Empty {
            self.map.remove(&coords)
        } else {
            self.map.insert(coords, machine)
        }
    }
    #[track_caller]
    pub fn get_tower(&self, coords: &IVec2) -> DynMachine {
        if let Some(machine) = self.map.get(coords) {
            machine.clone()
        } else {
            EMPTY_MACHINE.with(|cell| {
                cell.get().unwrap().clone()
            })
        }
    }
    #[track_caller]
    pub fn try_get_tower(&self, coords: &IVec2) -> Option<DynMachine> {
        self.map.get(coords).cloned()
    }
    pub async fn get_tower_texture(&self, coords: &IVec2) -> Texture2D {
        self.get_tower(coords).texture()
    }
    pub fn screen_to_world(&self, scr: Vec2, player_cell: Vec2) -> IVec2 {
        vec2i((scr/self.tilesize()+player_cell).floor())
    }
    pub fn world_to_screen_offset(&self, cell: IVec2, player_offset: Vec2) -> Vec2 {
        vec2i_to_f(cell)*self.tilesize()-player_offset
    }
    pub fn world_to_screen(&self, cell: IVec2, player_cell: Vec2) -> Vec2 {
        let player_offset = (player_cell.fract_gl()*self.tilesize()).abs();
        self.world_to_screen_offset(cell, player_offset)-player_cell.floor()*self.tilesize()
    }
    
    pub async fn draw(&mut self, player_cell: Vec2) -> Result<()> {
        let player_offset = (player_cell.fract_gl()*self.tilesize()).abs();
        let dest_size = Vec2::splat(self.tilesize());
        let w_tiles = (screen_width() / self.tilesize()).ceil() as i32;
        let h_tiles = (screen_height() / self.tilesize()).ceil() as i32;
        self.draw_background_stars(player_cell);
        if ((w_tiles*h_tiles) as usize) < self.map.len() {
            for tx in -1..=w_tiles {
                for ty in -1..=h_tiles {
                    self.draw_tile(ivec2(tx,ty), player_cell, dest_size, player_offset)?;
                }
            }
        } else {
            for tile in self.map.keys().cloned().collect::<Vec<IVec2>>() {
                self.draw_tile(tile-vec2i(player_cell.floor()), player_cell, dest_size, player_offset)?;
            }
        }
        for (coords, celest) in &self.celestials {
            let coords = self.world_to_screen(*coords, player_cell);
            draw_texture_ex(celest.texture(), coords.x, coords.y, WHITE, DrawTextureParams {
                dest_size: Some(vec2i_to_f(celest.size)*self.tilesize()),
                ..Default::default()
            });
        }
        Ok(())
    }
    fn draw_tile(&mut self, tile: IVec2, player_cell: Vec2, dest_size: Vec2, player_offset: Vec2) -> Result<()> {
        let c = tile + vec2i(player_cell.floor());
        let (tx,ty) = tile.into();
        let screen_pos = self.world_to_screen_offset(tile, player_offset);
        // let perlin = noise::Perlin::new(self.seed as _);
        // let density = noise::NoiseFn::get(&perlin, [tile.x as f64/10., tile.y as f64/10.]).abs()*200.;
        // if density != 0. {dbg!(density);};
        // let alpha = Color::from_rgba(255,255,255, density as u8);
        // draw_rectangle(screen_pos.x,screen_pos.y, dest_size.x, dest_size.y, alpha);
        
        let text = if let Some(machine) = self.map.get(&c) {
            machine.texture()
        } else {return Ok(())};
        // let translated_x = cx as f32-world.tilesize()+offset.x;
        // let translated_y = cy as f32-world.tilesize()+offset.y;
        // let current_cell = (camera_pos+c/world.tilesize());
        draw_texture_ex(&text, screen_pos.x,screen_pos.y, WHITE, DrawTextureParams {
            dest_size: Some(dest_size),
            ..Default::default()
        });
        Ok(())
    }
    fn draw_background_stars(&mut self, player_cell: Vec2) {
        rand_with_seed(self.seed);
        self.star_particles.retain(|(p, v,r, lifetime)| *lifetime <= 10.);

        for n in 0..STAR_PARTICLE_MAX_AMOUNT-self.star_particles.len() as i32 {
            let x = rand::gen_range(player_cell.x-1., player_cell.x+self.tiles_in_screen().x+1.);
            let y = rand::gen_range(player_cell.y-1., player_cell.y+self.tiles_in_screen().y+1.);
            let vx = rand::gen_range(-0.5, 0.5);
            let vy = rand::gen_range(-0.5, 0.5);
            let rot = macroquad::rand::gen_range(-360., 360.);
            let rot_vel = macroquad::rand::gen_range(-1.0, 1.);
            let lifetime = 0.;
            self.star_particles.push((vec2(x,y),vec2(vx,vy),vec2(rot,rot_vel), lifetime))
        }
        for (pos, vel, rot, lifetime) in &mut self.star_particles {
            *pos += *vel*get_frame_time();
            if !Rect::new(player_cell.x-1.,player_cell.y-1., screen_width()/self.tilesize+2., screen_height()/self.tilesize+2.).contains(*pos) {
                let (x,y) = player_cell.into();
                let (w,h) = (screen_width()/self.tilesize, screen_height()/self.tilesize);
                let p = *pos;

                pos.x = if p.x < x-1. {
                    rand::gen_range(player_cell.x+w-1., player_cell.x+w)
                } else if p.x > x-1.+w+2. {
                    rand::gen_range(player_cell.x-1., player_cell.x)
                } else {pos.x};
                pos.y = if p.y < y-1. {
                    rand::gen_range(player_cell.y+h-1., player_cell.y+h)
                } else if p.y > y-1.+h+2. {
                    rand::gen_range(player_cell.y-1., player_cell.y)
                } else {pos.y};

            }
            let pos = (*pos-player_cell)*self.tilesize;
            let alpha = (255.0 * (1.0 - ((*lifetime - 5.0) / 5.0).powi(2))) as u8;
            draw_texture_ex(&self.star_particle, pos.x, pos.y, Color::from_rgba(255, 255, 255, alpha), DrawTextureParams { dest_size: Some(vec2(0.25,0.25)*self.tilesize), rotation: rot.x,..Default::default() });
            *lifetime += get_frame_time();
            rot.x += rot.y*get_frame_time();
        }
    }
    pub fn interact(&mut self, player_cell: Vec2, build_mode: &BuildMode) -> Result<()> {
        if is_key_released(KeyCode::Escape) {
            self.enabled_gui = None;return Ok(())
        }
        let rect = if let Some(coords) = self.enabled_gui {
            let mut m = self.get_tower(&coords);
            let mut machine = unsafe {Rc::get_mut_unchecked(&mut m)};
            machine.update_gui()?;
            machine.draw_gui()?
        } else {Rect::default()};
        let mp = mouse_position().into();
        if is_mouse_button_released(MouseButton::Left) && build_mode.current == Tower::Empty && !rect.contains(mp) {
            let cell = self.screen_to_world(mp, player_cell);
            if let Some(machine_arc) = self.try_get_tower(&cell) {
                if self.enabled_gui.is_some() && self.enabled_gui.unwrap() == cell {
                    self.enabled_gui = None;
                } else {
                    self.enabled_gui.replace(cell);
                }
            }
        }
        
        Ok(())
    }
    pub fn control_tilesize(&mut self) -> Result<()> {
        let (wx, mut wy) = mouse_wheel();
        let zoom_factor = 1.1; 

        if wy > 0.0 { // Scroll up (zoom in)
            self.tilesize *= zoom_factor;
        } else if wy < 0.0 { // Scroll down (zoom out)
            self.tilesize /= zoom_factor;
        }

        let min_tilesize = 3.0;
        let max_tilesize = 256.0;
        self.tilesize = self.tilesize.clamp(min_tilesize, max_tilesize);
        Ok(())
    }
    pub fn remove_gui(&mut self) -> Option<IVec2> {
        self.enabled_gui.take()
    }

    pub fn update(&mut self, player_cell: Vec2, dt: f32) -> Result<()> {
        let keys = {
            let mut keys = Vec::new();
            for x in 0..self.update_radius {
                for y in 0..self.update_radius {
                    let coords = ivec2(x as i32+player_cell.x as i32-(self.update_radius as i32/2), y as i32+player_cell.y as i32-(self.update_radius as i32/2));
                    if self.map.contains_key(&coords) {
                        keys.push(coords);
                    }
                }
            }
            keys
        };
        for coords in keys {
            let mut machine_rc = self.map.get_mut(&coords).unwrap().clone();
            // The update function just operates on the tiles, but not on the whole hashmap
            let mut machine = unsafe {Rc::get_mut_unchecked(&mut machine_rc)};
            machine.update(&mut self.map, dt)?;
        }
        Ok(())
    }
}
