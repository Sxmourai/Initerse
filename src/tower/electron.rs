use super::*;

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
    pub fn deserialize(raw: &str) -> Result<Self> {
        let mut slf = Self::new();
        slf.buffer = raw["buffer: ".len()..].parse()?;
        Ok(slf)
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
        if clicked_button(collect_rect) {
            dbg!();
        }
        Ok(Rect::new(x, y, w, h))
    }


    fn update(&mut self, map: &mut Map, dt: f32) -> Result<()> {
        self.buffer += self.collect_speed * dt;
        Ok(())
    }

    fn ty(&self) -> Tower {
        Tower::Electron
    }
    
    fn serialize(&self) -> String {
        format!("buffer: {}", self.buffer)
    }
}