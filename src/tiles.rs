use std::{borrow::Borrow, cell::RefCell, sync::RwLock};

use super::*;

pub const TILE_SIZEU: u32 = 64;
pub const TILE_SIZEI: i32 = TILE_SIZEU as _;
pub const TILE_SIZE: f32 = TILE_SIZEU as _;

pub static TOWER_TEXTURES: [RwLock<Option<Texture2D>>; 3] = [
    RwLock::new(None),
    RwLock::new(None),
    RwLock::new(None),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tower {
    Empty = 0,
    Electron = 1,
    StringCreator = 2,
}
impl Default for Tower {fn default() -> Self {Self::Empty}}
impl Tower {
    /// LOADS A NEW TEXTURE
    pub async fn load_texture(self) -> Result<Texture2D> {
        let texture = match self {
            Tower::Empty => load_texture("assets/empty.png").await?,
            Tower::Electron => load_texture("assets/electron.png").await?,
            Tower::StringCreator => load_texture("assets/string creator.png").await?,
        };
        let mut cache = TOWER_TEXTURES[self as usize].write().unwrap();
        if cache.is_none() {
            cache.replace(texture.clone());
        }
        Ok(texture)
    }
    pub fn try_loaded_texture(self) -> Option<Texture2D> {
        Some(TOWER_TEXTURES[self as usize].read().ok()?.clone()?)
    }
    pub async fn loaded_texture(self) -> Texture2D {
        if let Some(texture) = self.try_loaded_texture() {
            texture
        } else {
            self.load_texture().await.unwrap()
        }
    }
}

pub struct World {
    map: hashbrown::HashMap<IVec2, Tower>,
}
impl World {
    pub async fn empty() -> Result<Self> {
        // setup_cache_tower_textures(); // Lazily done for now
        Ok(Self { map: Default::default() })
    }
    pub fn set_tower(&mut self, coords: IVec2, tower: Tower) -> Option<Tower> {
        self.map.insert(coords, tower)
    }
    pub fn get_tower(&self, coords: &IVec2) -> Tower {
        self.map.get(coords).cloned().unwrap_or_default()
    }
    pub async fn get_tower_texture(&self, coords: &IVec2) -> Texture2D {
        self.get_tower(coords).loaded_texture().await
    }
    pub async fn draw(&mut self, player_cell: Vec2) -> Result<()> {
        let mut offset = (player_cell - player_cell.floor())*TILE_SIZE;
        let dest_size = Vec2::splat(TILE_SIZE);
        let w_tiles = (screen_width() / TILE_SIZE).ceil() as i32;
        let h_tiles = (screen_height() / TILE_SIZE).ceil() as i32;
        for tx in -1..=w_tiles {
            for ty in -1..=h_tiles {
                let tf = vec2(tx as _,ty as _)*TILE_SIZE;
                let c = IVec2::new(tx,ty) + vec2i(&player_cell);
                let text = self.get_tower_texture(&c).await;
                // let translated_x = cx as f32-TILE_SIZE+offset.x;
                // let translated_y = cy as f32-TILE_SIZE+offset.y;
                // let current_cell = (camera_pos+c/TILE_SIZE);
                let screen_pos = vec2(tf.x - offset.x, tf.y - offset.y);
                draw_texture_ex(&text, screen_pos.x,screen_pos.y, WHITE, DrawTextureParams {
                    dest_size: Some(dest_size),
                    ..Default::default()
                });
            }
        }
        Ok(())
    }
}

pub fn vec2i(vec2f: &Vec2) -> IVec2 {
    IVec2::new(vec2f.x as _,vec2f.y as _)
}