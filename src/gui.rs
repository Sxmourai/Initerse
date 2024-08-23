use super::*;
pub fn button(button: Rect,label: &str,font_size:f32, color: Color) -> bool {
    let Rect{x,y,w,h} = button;
    draw_rectangle(x, y, w, h, color);
    draw_text(label, x+font_size/1.0, y+font_size/1.0, font_size, WHITE);
    clicked_button(button)
}
pub fn left_click() -> bool {is_mouse_button_released(MouseButton::Left)}
pub fn clicked_button(button: Rect) -> bool {
    left_click() && button.contains(mouse_position().into())
}

pub struct TextBox {
    pub text: String,
    pub color: Color,
    pub rect: Rect,
    pub focused: bool,
}
impl TextBox {
    pub fn new(text: String, rect: Rect, color: Color) -> Self {
        Self {
            text,
            rect,
            color,
            focused: false,
        }
    }
    pub fn empty(rect: Rect) -> Self {
        Self::new(String::new(), rect, DARKGRAY)
    }
    pub fn update(&mut self) {
        if left_click() {
            if clicked_button(self.rect) {
                self.focused = true;
                self.color.a = 0.5;
            } else {
                self.focused = false;
                self.color.a = 1.;
            }
        }
        if self.focused {
            while let Some(char) = get_char_pressed() {
                self.text.push(char);
            }
            self.rect.w = self.text.len() as f32*12.;
        }
    }
    pub fn draw(&self) {
        let Rect{x,y,w,h} = self.rect;
        draw_rectangle(x, y, w, h, self.color);
        draw_text(&self.text, x+10., y+28., 24., WHITE);

    }
}