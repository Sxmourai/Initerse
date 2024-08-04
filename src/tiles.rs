use super::*;

pub const TILE_SIZEU: u32 = 64;
pub const TILE_SIZEI: i32 = TILE_SIZEU as _;
pub const TILE_SIZE: f32 = TILE_SIZEU as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tower {
    Empty,
    Electron,
    StringCreator,
}
impl Tower {
    pub async fn load_texture(self) -> Result<Texture2D> {
        Ok(match self {
            Tower::Empty => load_texture("assets/empty.png").await?,
            Tower::Electron => load_texture("assets/electron.png").await?,
            Tower::StringCreator => load_texture("assets/string creator.png").await?,
        })
    }
}
pub struct World {
    map: hashbrown::HashMap<IVec2, (Tower, Texture2D)>,
    empty: Texture2D,
}
impl World {
    pub fn get_tower_texture(&self, coords: &IVec2) -> &Texture2D {
        if let Some((tower, texture)) = self.map.get(coords) {
            texture
        } else {&self.empty}
    }
    pub fn draw(&mut self, player_cell: Vec2) -> Result<()> {
        let offset = (player_cell - player_cell.floor())*TILE_SIZE;
        let dest_size = Vec2::splat(TILE_SIZE);
        for cx in (0..=(screen_width() as i32+TILE_SIZEI)).step_by(TILE_SIZEU as _) {
            for cy in (0..=(screen_height() as i32+TILE_SIZEI)).step_by(TILE_SIZEU as _) {
                let c = IVec2::new(cx,cy)/TILE_SIZEI + vec2i(&player_cell)-TILE_SIZEI;
                let text = self.get_tower_texture(&c);
                // let translated_x = cx as f32-TILE_SIZE+offset.x;
                // let translated_y = cy as f32-TILE_SIZE+offset.y;
                // let current_cell = (camera_pos+c/TILE_SIZE);
                draw_texture_ex(text, cx as f32-offset.x-TILE_SIZE,cy as f32+offset.y-TILE_SIZE, WHITE, DrawTextureParams {
                    dest_size: Some(dest_size),
                    ..Default::default()
                });
            }
        }
        Ok(())
    }
    pub async fn empty() -> Result<Self> {
        Ok(Self { map: Default::default(), empty: Tower::Empty.load_texture().await? })
    }
}

pub fn vec2i(vec2f: &Vec2) -> IVec2 {
    IVec2::new(vec2f.x as _,vec2f.y as _)
}