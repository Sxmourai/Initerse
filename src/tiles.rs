
use std::{cell::{Cell, OnceCell, RefCell}, sync::{Arc, Mutex, MutexGuard}};

use build_mode::BuildMode;
use tower::{EmptyMachine, Machine};

use super::*;

pub const BASE_TILE_SIZE: f32 = 48.;

pub type DynMachine = Arc<Mutex<dyn Machine>>;
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

pub struct World {
    map: hashbrown::HashMap<IVec2, DynMachine>,
    tilesize: f32,
    enabled_gui: Option<(IVec2, DynMachine)>,
}
impl World {
    pub const fn tilesize(&self) -> f32 {self.tilesize}
    pub async fn empty() -> Result<Self> {
        Ok(Self { map: Default::default(), tilesize: BASE_TILE_SIZE, enabled_gui: None })
    }
    pub fn set_tower(&mut self, coords: IVec2, machine: impl Into<DynMachine>) -> Option<DynMachine> {
        let machine = machine.into();
        if machine.lock().unwrap().ty() == Tower::Empty {
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
        self.get_tower(coords).lock().unwrap().texture()
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
        let rect = if let Some((coords, machine)) = &mut self.enabled_gui {
            let mut machine = machine.lock().unwrap();
            machine.update_gui()?;
            machine.draw_gui()?
        } else {Rect::default()};
        let mp = mouse_position().into();
        if is_mouse_button_released(MouseButton::Left) && build_mode.current == Tower::Empty && !rect.contains(mp) {
            let cell = self.screen_to_world(mp, player_cell);
            if let Some(machine_arc) = self.try_get_tower(&cell) {
                if self.enabled_gui.is_some() && self.enabled_gui.as_ref().unwrap().0 == cell {
                    self.enabled_gui = None;
                } else {
                    self.enabled_gui.replace((cell, machine_arc));
                }
            }
        }
        
        Ok(())
    }
    pub fn control_tilesize(&mut self) -> Result<()> {
        let (wx, mut wy) = mouse_wheel();
        // dbg!(wy);
        // wy *= -1.;
        // if wy < 0. {wy = 1./wy}
        // let new = (self.tilesize-BASE_TILE_SIZE).log2()+wy;
        // self.tilesize = 10.0f32.max(new+BASE_TILE_SIZE);
        Ok(())
    }
    pub fn remove_gui(&mut self) -> Option<(IVec2, DynMachine)> {
        self.enabled_gui.take()
    }

    pub fn update(&mut self, dt: f32) -> Result<()> {
        for (coords, machine) in self.map.iter_mut() {
            let mut machine = machine.lock().unwrap();
            machine.update(dt)?;
        }
        Ok(())
    }
}
