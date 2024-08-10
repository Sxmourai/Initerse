use color_eyre::eyre::ContextCompat as _;

use super::*;
pub struct BuildMode {
    pub current: Tower,
}
impl BuildMode {
    pub fn new() -> Self {
        Self {
            current: Tower::default()
        }
    }
    pub async fn draw(&mut self, world: &mut World, player_cell: Vec2, on_hotbar: bool) -> Result<()> {
        if on_hotbar == true {return Ok(())}
        if self.current == Tower::Empty && !is_mouse_button_down(MouseButton::Right)  {return Ok(())}
        if is_mouse_button_released(MouseButton::Right) || is_key_released(KeyCode::Escape) {self.current = Tower::Empty; return Ok(())}
        let offset = (player_cell.fract_gl()*world.tilesize()).abs();
        let mut mp = Vec2::from(mouse_position());
        let texture = self.current.loaded_texture().await;
        let world_cell = world.screen_to_world(mp, player_cell);
        // Check if left click is pressed, if so, build at the current pointed cell
        // If right click is set and we are not in building mode, it means we want to erase some machines
        // We know that self.current = Tower::Empty, so it's like removing the tower
        if is_mouse_button_down(MouseButton::Left) || (is_mouse_button_down(MouseButton::Right) && self.current == Tower::Empty) {
            let _prev = world.set_tower(world_cell, self.current.new_machine().context("Can't build new machine")?);
        }
        let scr = world.world_to_screen_offset(world_cell, offset)-(player_cell.floor())*world.tilesize();
        draw_texture_ex(&texture, scr.x, scr.y, Color::from_rgba(255,255,255,150), DrawTextureParams { dest_size: Some(Vec2::splat(world.tilesize())), ..Default::default() });
        Ok(())
    }
}
