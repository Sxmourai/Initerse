
use std::{cell::{Cell, OnceCell, RefCell}, rc::Rc, sync::{Arc, Mutex, MutexGuard}};

use build_mode::BuildMode;
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

pub struct World {
    map: Map,
    enabled_gui: Option<IVec2>,
    tilesize: f32,
    update_radius: usize,
}
impl World {
    pub const fn tilesize(&self) -> f32 {self.tilesize}
    pub fn empty() -> Self {
        Self { 
            map: Default::default(), 
            tilesize: BASE_TILE_SIZE, 
            enabled_gui: None, 
            update_radius: BASE_UPDATE_RADIUS
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
    pub fn world_to_screen(&self, cell: IVec2, player_offset: Vec2) -> Vec2 {
        vec2i_to_f(cell)*self.tilesize()-player_offset
    }
    pub async fn draw(&mut self, player_cell: Vec2) -> Result<()> {
        let offset = (player_cell.fract_gl()*self.tilesize()).abs();
        let dest_size = Vec2::splat(self.tilesize());
        let w_tiles = (screen_width() / self.tilesize()).ceil() as i32;
        let h_tiles = (screen_height() / self.tilesize()).ceil() as i32;
        for tx in -1..=w_tiles {
            for ty in -1..=h_tiles {
                let tf = vec2(tx as _,ty as _);
                let c = IVec2::new(tx,ty) + vec2i(player_cell.floor());
                let text = self.get_tower_texture(&c).await;
                // let translated_x = cx as f32-world.tilesize()+offset.x;
                // let translated_y = cy as f32-world.tilesize()+offset.y;
                // let current_cell = (camera_pos+c/world.tilesize());
                let screen_pos = self.world_to_screen(ivec2(tx,ty), offset);
                draw_texture_ex(&text, screen_pos.x,screen_pos.y, WHITE, DrawTextureParams {
                    dest_size: Some(dest_size),
                    ..Default::default()
                });
            }
        }
        Ok(())
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

        let min_tilesize = 10.0;
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
