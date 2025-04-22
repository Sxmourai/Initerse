use strum_macros::{self, EnumIter, EnumProperty};


#[derive(EnumProperty, EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum MachineType {
    #[strum(props(path = "string creator.png"))]
    StringCreator,
    #[strum(props(path = "electron.png"))]
    Electron,
    #[strum(props(path = "energy.png"))]
    Energy,
    #[strum(props(path = "empty.png"))]
    Empty,
}
impl MachineType {
    pub fn as_index(self) -> usize {
        self as _
    }
    pub fn path(self) -> &'static str {
        strum::EnumProperty::get_str(&self, "path").unwrap()
    }
}