use build_mode::BuildMode;
use strum::{EnumProperty, IntoEnumIterator};

use super::*;

pub const SLOTS: usize = 10;

pub type Slot = Tower;

pub struct Hotbar {
    slots: [Slot; SLOTS],
}
impl Hotbar {
    pub fn new() -> Self {
        Self {
            slots: {
                let mut slots = [Tower::Empty; SLOTS];
                let mut si = 0;
                for (i, tower) in Tower::iter().enumerate() {
                    if tower.get_str("buildable") == Some("true") {
                        slots[si] = tower;
                        si += 1;
                    }
                }
                slots
            },
        }
    }
    pub async fn draw(&mut self, build_mode: &mut BuildMode) -> Result<bool> {
        let x = 100.;
        let w = screen_width()-x*2.;
        let h = 50.;
        let y = screen_height()-h;
        let slot_size = vec2(w/SLOTS as f32, h);
        draw_rectangle(x, y, w, h, GRAY);
        for (i, slot) in self.slots.iter().enumerate() {
            if *slot == Tower::Empty {continue;}
            let text = slot.loaded_texture().await;
            draw_texture_ex(&text, x+slot_size.x*i as f32, y as f32, WHITE, DrawTextureParams { dest_size: Some(slot_size), ..Default::default() })
        }
        let (mx,my) = mouse_position();
        let mut on_hotbar = my >= y && my < y+h && mx > x && mx < x+w;
        if on_hotbar {
            let slot_x = (mx-x) / slot_size.x;
            if is_mouse_button_released(MouseButton::Left) {
                let idx = slot_x as usize;
                build_mode.current = self.slots[idx];
            }
        }
        Ok(on_hotbar)
    }
}