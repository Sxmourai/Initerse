use color_eyre::eyre::ContextCompat;
use macroquad::ui::root_ui;
use strum::{EnumCount, IntoEnumIterator};
use tiles::{DynMachine, WORLD};

use super::*;

use std::{borrow::Borrow, cell::RefCell, sync::RwLock};

pub static TOWER_TEXTURES: [RwLock<Option<Texture2D>>; Tower::COUNT] = [const{RwLock::new(None)}; Tower::COUNT];

pub async fn setup_cache_tower_textures() -> Result<()> {
    if let Err(e) = EMPTY_MACHINE.with(|cell| cell.set(Arc::new(Mutex::new(EmptyMachine {})))) {
        todo!()
    }
    for (i,tower) in Tower::iter().enumerate() {
        TOWER_TEXTURES[i].write().unwrap().replace(tower.load_texture().await?);
    }
    
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, strum_macros::EnumCount)]
pub enum Tower {
    Empty,
    Electron,
    StringCreator,
    Water,
    Grass,
    Ice,
}
impl Default for Tower {fn default() -> Self {Self::Empty}}
impl Tower {
    /// loads a new texture, expensive
    pub async fn load_texture(self) -> Result<Texture2D> {
        let texture = match self {
            Tower::Empty => load_texture("assets/empty.png").await?,
            Tower::Water => load_texture("assets/water.png").await?,
            Tower::Grass => load_texture("assets/grass.png").await?,
            Tower::Ice   => load_texture("assets/ice.png").await?,
            Tower::Electron => load_texture("assets/electron.png").await?,
            Tower::StringCreator => load_texture("assets/string creator.png").await?,
        };
        Ok(texture)
    }
    pub fn try_loaded_texture(self) -> Option<Texture2D> {
        Some(TOWER_TEXTURES[self as usize].read().ok()?.clone()?)
    }
    pub async fn loaded_texture(self) -> Texture2D {
        if let Some(texture) = self.try_loaded_texture() {
            texture
        } else {
            miniquad::warn!("Loading new texture");
            self.load_texture().await.unwrap()
        }
    }
    pub fn new_machine(self) -> Option<DynMachine> {
        Some(match self {
            Tower::Empty    => Arc::new(Mutex::new(EmptyMachine {})),
            Tower::Electron => Arc::new(Mutex::new(Electron::new())),
            Tower::StringCreator => Arc::new(Mutex::new(StringCreator::new())),
            Tower::Water    => return None,
            Tower::Grass    => return None,
            Tower::Ice      => return None,
        })
    }
}

pub trait Machine {
    fn draw_gui(&mut self) -> Result<Rect>;
    fn update_gui(&mut self) -> Result<()>;
    fn update(&mut self, dt: f32) -> Result<()>;
    fn ty(&self) -> Tower;
    #[track_caller]
    fn texture(&self) -> Texture2D {
        self.ty().try_loaded_texture().context(format!("Can't get texture of {:?}", self.ty())).unwrap()
    }
}
pub struct Electron {
    buffer: f32,
    collect_speed: f32,
    name: String,
}
impl Electron {
    pub fn new() -> Self {
        Self {
            buffer: 0.,
            collect_speed: 1.,
            name: "Electron".to_string(),
        }
    }
}
impl Machine for Electron {
    fn draw_gui(&mut self) -> Result<Rect> {
        let (x,y) = (100.,50.);
        let (w,h) = (screen_width()-x*2., screen_height()-y*2.);
        draw_rectangle(x, y, w, h, DARKGRAY);
        draw_text(&self.name, x+10., y+32., 32., WHITE);
        draw_line(x+w-20., y+10., x+w-10., y+20., 2., WHITE);
        draw_line(x+w-20., y+20., x+w-10., y+10., 2., WHITE);
        let collect_str = format!("Collect ({:.0} strings)", self.buffer);
        let collect_rect = Rect::new(x+5., y+80.0-28., collect_str.len() as f32*15., 40.);
        draw_rectangle(collect_rect.x, collect_rect.y, collect_rect.w, collect_rect.h, Color::from_rgba(255,255,255,30));
        draw_text(&collect_str, x+10., y+80., 32., WHITE);
        if is_mouse_button_down(MouseButton::Left) {
            let mp = mouse_position();
            if Rect::new(x+w-30., y, 30., 30.).contains(mp.into()) {
                unsafe { WORLD.as_mut().unwrap().remove_gui().unwrap(); }
            }
        }
        if is_clicked(collect_rect) {
            dbg!();
        }
        Ok(Rect::new(x, y, w, h))
    }

    fn update_gui(&mut self) -> Result<()> {
        
        Ok(())
    }

    fn update(&mut self, dt: f32) -> Result<()> {
        self.buffer += self.collect_speed * dt;
        Ok(())
    }

    fn ty(&self) -> Tower {
        Tower::Electron
    }
}


pub struct StringCreator {
    buffer: f32,
    collect_speed: f32,
    name: String,
}
impl StringCreator {
    pub fn new() -> Self {
        Self {
            buffer: 0.,
            collect_speed: 1.,
            name: "String creator".to_string(),
        }
    }
}
impl Machine for StringCreator {
    fn draw_gui(&mut self) -> Result<Rect> {todo!()
    }

    fn update_gui(&mut self) -> Result<()> {
        
        Ok(())
    }

    fn update(&mut self, dt: f32) -> Result<()> {
        self.buffer += self.collect_speed * dt;
        Ok(())
    }

    fn ty(&self) -> Tower {
        Tower::StringCreator
    }
}
pub fn is_clicked(r: Rect) -> bool {
    let mp = mouse_position();
    let in_bounds = r.contains(mp.into());
    is_mouse_button_released(MouseButton::Left) && in_bounds
}

#[derive(Clone, Copy)]
pub struct EmptyMachine {

}
impl Machine for EmptyMachine {
    fn draw_gui(&mut self) -> Result<Rect> {
        todo!()
    }

    fn update_gui(&mut self) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, dt: f32) -> Result<()> {
        Ok(())
    }

    fn ty(&self) -> Tower {
        Tower::Empty
    }
}