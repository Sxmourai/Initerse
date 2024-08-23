use color_eyre::eyre::ContextCompat;
use macroquad::ui::root_ui;
use strum::{EnumCount, EnumProperty, IntoEnumIterator};
use tiles::{new_machine, DynMachine, Map, WORLD};

use super::*;

pub mod electron;
pub mod string_creator;
pub mod antimatter_collector;

use std::{borrow::Borrow, cell::RefCell, sync::RwLock};

pub static TOWER_TEXTURES: [RwLock<Option<Texture2D>>; Tower::COUNT] = [const{RwLock::new(None)}; Tower::COUNT];

pub async fn setup_cache_tower_textures() -> Result<()> {
    if let Err(e) = EMPTY_MACHINE.with(|cell| cell.set(new_machine(EmptyMachine {}))) {
        todo!()
    }
    for (i,tower) in Tower::iter().enumerate() {
        TOWER_TEXTURES[i].write().unwrap().replace(tower.load_texture().await?);
    }
    
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, strum_macros::EnumCount, strum_macros::EnumProperty, strum_macros::EnumString)]
pub enum Tower {
    #[strum(props(asset_path = "empty.png"))]
    Empty,
    #[strum(props(asset_path = "electron.png", buildable = "true"))]
    Electron,
    // #[strum(props(asset_path = "string creator.png", buildable = "true"))]
    // StringCreator,
    #[strum(props(asset_path = "antimatter_collector.png", buildable = "true"))]
    AntimatterCollector,
}
impl Default for Tower {fn default() -> Self {Self::Empty}}
impl Tower {
    pub fn texture_path(self) -> &'static str {
        self.get_str("asset_path").unwrap_or(Self::default().get_str("asset_path").unwrap())
    } 
    /// loads a new texture, expensive
    pub async fn load_texture(self) -> Result<Texture2D> {
        let texture = match load_texture(&format!("assets/{}", self.texture_path())).await {
            Ok(t) => t,
            Err(e) => {
                miniquad::warn!("Couldn't load image: {} - Err: {:?}", &format!("assets/{}", self.texture_path()), e);
                load_texture(&format!("assets/{}", Self::Empty.texture_path())).await?
            },
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
            Tower::Empty    => new_machine(EmptyMachine {}),
            Tower::Electron => new_machine(electron::Electron::new()),
            // Tower::StringCreator => new_machine(string_creator::StringCreator::new()),
            Tower::AntimatterCollector => new_machine(antimatter_collector::new()),
        })
    }
    pub fn deserialize_machine(self, raw: &str) -> Result<DynMachine> {
        Ok(match self {
            Tower::Empty    => new_machine(EmptyMachine {}),
            Tower::Electron => new_machine(electron::Electron::deserialize(raw)?),
            // Tower::StringCreator => new_machine(string_creator::StringCreator::deserialize(raw)),
            Tower::AntimatterCollector => new_machine(antimatter_collector::deserialize(raw)?),
        })
    }
}

pub trait Machine {
    fn draw_gui(&mut self) -> Result<Rect>;
    fn update(&mut self, map: &mut Map, dt: f32) -> Result<()>;
    fn ty(&self) -> Tower;
    fn serialize(&self) -> String;
    #[track_caller]
    fn texture(&self) -> Texture2D {
        self.ty().try_loaded_texture().context(format!("Can't get texture of {:?}", self.ty())).unwrap()
    }
}




#[derive(Clone, Copy)]
pub struct EmptyMachine {

}
impl Machine for EmptyMachine {
    fn draw_gui(&mut self) -> Result<Rect> {
        unreachable!()
    }

    fn update(&mut self, map: &mut Map, dt: f32) -> Result<()> {
        Ok(())
    }

    fn ty(&self) -> Tower {
        Tower::Empty
    }
    
    fn serialize(&self) -> String {
        unreachable!()
    }
}
