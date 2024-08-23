use std::cell::OnceCell;

use super::*;
use color_eyre::eyre::ContextCompat;
use hashbrown::HashMap;
use serde::{de::Visitor, ser::SerializeStruct, Serialize};
use strum::IntoEnumIterator;

pub static mut CONFIG: OnceCell<Config> = OnceCell::new();

// A clone of macroquad's keycode, that you can deserialize
#[derive(strum_macros::EnumString)]
#[repr(u16)]
pub enum DeserializeKeyCode {
    Space = 0x0020,
    Apostrophe = 0x0027,
    Comma = 0x002c,
    Minus = 0x002d,
    Period = 0x002e,
    Slash = 0x002f,
    Key0 = 0x0030,
    Key1 = 0x0031,
    Key2 = 0x0032,
    Key3 = 0x0033,
    Key4 = 0x0034,
    Key5 = 0x0035,
    Key6 = 0x0036,
    Key7 = 0x0037,
    Key8 = 0x0038,
    Key9 = 0x0039,
    Semicolon = 0x003b,
    Equal = 0x003d,
    A = 0x0041,
    B = 0x0042,
    C = 0x0043,
    D = 0x0044,
    E = 0x0045,
    F = 0x0046,
    G = 0x0047,
    H = 0x0048,
    I = 0x0049,
    J = 0x004a,
    K = 0x004b,
    L = 0x004c,
    M = 0x004d,
    N = 0x004e,
    O = 0x004f,
    P = 0x0050,
    Q = 0x0051,
    R = 0x0052,
    S = 0x0053,
    T = 0x0054,
    U = 0x0055,
    V = 0x0056,
    W = 0x0057,
    X = 0x0058,
    Y = 0x0059,
    Z = 0x005a,
    LeftBracket = 0x005b,
    Backslash = 0x005c,
    RightBracket = 0x005d,
    GraveAccent = 0x0060,
    World1 = 0x0100,
    World2 = 0x0101,
    Escape = 0xff1b,
    Enter = 0xff0d,
    Tab = 0xff09,
    Backspace = 0xff08,
    Insert = 0xff63,
    Delete = 0xffff,
    Right = 0xff53,
    Left = 0xff51,
    Down = 0xff54,
    Up = 0xff52,
    PageUp = 0xff55,
    PageDown = 0xff56,
    Home = 0xff50,
    End = 0xff57,
    CapsLock = 0xffe5,
    ScrollLock = 0xff14,
    NumLock = 0xff7f,
    PrintScreen = 0xfd1d,
    Pause = 0xff13,
    F1 = 0xffbe,
    F2 = 0xffbf,
    F3 = 0xffc0,
    F4 = 0xffc1,
    F5 = 0xffc2,
    F6 = 0xffc3,
    F7 = 0xffc4,
    F8 = 0xffc5,
    F9 = 0xffc6,
    F10 = 0xffc7,
    F11 = 0xffc8,
    F12 = 0xffc9,
    F13 = 0xffca,
    F14 = 0xffcb,
    F15 = 0xffcc,
    F16 = 0xffcd,
    F17 = 0xffce,
    F18 = 0xffcf,
    F19 = 0xffd0,
    F20 = 0xffd1,
    F21 = 0xffd2,
    F22 = 0xffd3,
    F23 = 0xffd4,
    F24 = 0xffd5,
    F25 = 0xffd6,
    Kp0 = 0xffb0,
    Kp1 = 0xffb1,
    Kp2 = 0xffb2,
    Kp3 = 0xffb3,
    Kp4 = 0xffb4,
    Kp5 = 0xffb5,
    Kp6 = 0xffb6,
    Kp7 = 0xffb7,
    Kp8 = 0xffb8,
    Kp9 = 0xffb9,
    KpDecimal = 0xffae,
    KpDivide = 0xffaf,
    KpMultiply = 0xffaa,
    KpSubtract = 0xffad,
    KpAdd = 0xffab,
    KpEnter = 0xff8d,
    KpEqual = 0xffbd,
    LeftShift = 0xffe1,
    LeftControl = 0xffe3,
    LeftAlt = 0xffe9,
    LeftSuper = 0xffeb,
    RightShift = 0xffe2,
    RightControl = 0xffe4,
    RightAlt = 0xffea,
    RightSuper = 0xffec,
    Menu = 0xff67,
    Unknown = 0x01ff,
}


#[derive(strum_macros::EnumIter, strum_macros::EnumProperty,strum_macros::EnumString, Hash, Debug, PartialEq, Eq, Clone, Copy)]
// #[derive(Serialize)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
}
impl Action {
    pub fn default_keycode(self) -> KeyCode {
        match self {
            Action::Forward => KeyCode::Z,
            Action::Backward => KeyCode::S,
            Action::Left => KeyCode::Q,
            Action::Right => KeyCode::D,
        }
    }
}

pub type KeyMap = HashMap<Action, KeyCode>;
#[derive(Debug)]
pub struct Config {
    pub keymap: KeyMap,
}
impl Config {
    fn parse_keybind_line(line: &str) -> Result<(Action, KeyCode)> {
        let mut l = line.split(" = ");
        let action = l.next().context("Need an action and keybind (e.g. Forward = \"Z\")")?.parse()?;
        let raw_bind = l.next().context("Need an action and keybind (e.g. Forward = \"Z\")")?.replace("\"", "");
        // Little cheat I found 🤣
        let bind = unsafe { core::mem::transmute(raw_bind.parse::<DeserializeKeyCode>()?) };
        Ok((action, bind))
    }
    fn read() -> Result<Self> {
        let raw = std::fs::read_to_string("config.toml")?;
        let mut keymap = KeyMap::default();
        for l in raw.split("\n") {
            let l = l.trim();
            if l.is_empty() {continue}
            match Self::parse_keybind_line(l) {
                Ok((action, bind)) => {keymap.insert(action, bind);},
                Err(e) => {miniquad::warn!("Couldn't parse keybind: {:?}", e);},
            } 
        }
        for action in Action::iter() {
            keymap.entry(action).or_insert(action.default_keycode());
        }
        Ok(Self {
            keymap,
        })
    }
    pub fn get() -> Self {
        if let Ok(slf) = Self::read() {
            return slf
        }
        miniquad::info!("Couldn't read config, using default config instead");
        Self {
            keymap: {
                let mut map = KeyMap::default();
                for action in <Action as strum::IntoEnumIterator>::iter() {
                    let code = action.default_keycode();
                    map.insert(action, code);
                }
                map
            },
        }
    } 
    pub fn write(&self) -> Result<()> {
        let mut raw = String::new();
        for (action, bind) in self.keymap.iter() {
            raw.push_str(&format!("{action:?} = \"{bind:?}\"\n"));
        }
        std::fs::write("config.toml", raw)?;
        Ok(())
    }
}
// impl serde::Serialize for Config {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//         let mut s = serializer.serialize_struct("Keybinds", self.keymap.len()).unwrap();
//         for (action, bind) in self.keymap.iter() {
//             s.serialize_field(format!("{:?}", action).leak(), &format!("{bind:?}")).unwrap();
//         }
//         s.end()
//     }
// }
// impl<'de> serde::Deserialize<'de> for Config {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//         let mut fields = vec![];
//         for action in <Action as strum::IntoEnumIterator>::iter() {
//             fields.push(&*format!("{action:?}").leak());
//         }
        
//         let mut s = deserializer.deserialize_struct("Keybinds", &*fields.leak(), MyVisitor{s:String::new()}).unwrap();
//         dbg!(s);
//         todo!()
//     }
// }
// struct MyVisitor {
//     s: String,
// }
// impl Visitor<'_> for MyVisitor {
//     type Value = String;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "").unwrap();
//         Ok(())
//     }
// }

pub struct ConfigMenu {
    config: &'static mut Config,
    modifying: Option<(usize, Action)>,
}
impl ConfigMenu {
    pub fn new() -> Self {
        Self {
            config: unsafe { CONFIG.get_mut().unwrap() },
            modifying: None,
        }
    }
    pub fn update(&mut self) -> bool {
        draw_text("Keybinds", screen_width()/2., 100., 24., WHITE);
        for (i,(action, bind)) in self.config.keymap.iter().enumerate() {
            if button(Rect::new(screen_width()/2.0-100., screen_height()/2.0-200.+50.*i as f32, 200., 50.), &format!("{:?}: {:?}", action, bind), 32., DARKGRAY) {
                self.modifying.replace((i, *action));
                return false; // If clicked on this button, won't click on other buttons
            }
        }
        if button(Rect::new(screen_width()/2.0-100., screen_height()/2.0+100., 200., 50.), "Back", 32., DARKGRAY) {
            if let Err(e) = self.config.write() {
                miniquad::warn!("Error saving config ! {e:?}");
            }
            return true
        }
        if let Some((i, action)) = self.modifying {
            let _ = button(Rect::new(screen_width()/2.0-100., screen_height()/2.0-200.+50.*i as f32, 200., 50.), "Press a key", 32., DARKGRAY);
            if let Some(key) = get_last_key_pressed() {
                self.modifying = None;
                
                self.config.keymap.entry(action).and_modify(|e| *e = key);
            }
        }
        false
    }
}