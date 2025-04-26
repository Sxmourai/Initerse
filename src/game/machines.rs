use strum_macros::{self, EnumIter, EnumProperty};

use crate::*;


#[derive(EnumProperty, EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum MachineType {
    #[strum(props(path = "string creator.png", name = "String creator"))]
    StringCreator,
    #[strum(props(path = "electron.png", name = "Electron"))]
    Electron,
    #[strum(props(path = "energy.png", name = "Energy"))]
    Energy,
    #[strum(props(path = "empty.png", name = "Empty"))]
    Empty,
}
impl MachineType {
    pub fn as_index(self) -> usize {
        self as _
    }
    pub fn path(self) -> &'static str {
        strum::EnumProperty::get_str(&self, "path").unwrap()
    }
    pub fn name(self) -> &'static str {
        strum::EnumProperty::get_str(&self, "name").unwrap()
    }
}



pub type MachineCommonMut<'a> = (&'a mut Sprite, &'a mut Visibility, &'a mut Transform);

pub trait MachineTrait: Sync + Send + std::fmt::Debug {
    fn update(&mut self, com: MachineCommonMut<'_>) {
        dbg!(self.name());
    }
    fn ty(&self) -> MachineType;
    fn name(&self) -> String {self.ty().name().to_string()}
}
pub fn machine_box_from_ty(ty: MachineType) -> Box<dyn MachineTrait> {
    match ty {
        MachineType::StringCreator => Box::new(StringCreator {}),
        MachineType::Electron => Box::new(Electron {}),
        MachineType::Energy => Box::new(Energy {}),
        MachineType::Empty => todo!(),
    }
}



#[derive(Debug)]
pub struct StringCreator {}
impl MachineTrait for StringCreator {

    fn ty(&self) -> MachineType {
        MachineType::StringCreator
    }
    
    fn name(&self) -> String {
        "String creator".to_string()
    }
}

#[derive(Debug)]
pub struct Electron {}
impl MachineTrait for Electron {

    fn ty(&self) -> MachineType {
        MachineType::Electron
    }
}

#[derive(Debug)]
pub struct Energy {}
impl MachineTrait for Energy {

    fn ty(&self) -> MachineType {
        MachineType::Energy
    }
}


#[derive(Debug, Bundle)]
pub struct Machine {
    pub com: MachineCommon,
    pub machine: M,
}

impl Machine {
    pub fn new(sprite: Sprite, pos: Vec2, machine: Box<dyn MachineTrait>) -> Self {
        Self {
            com: MachineCommon { sprite, vis: Visibility::Visible, transform: Transform::from_translation(pos.extend(1.)), _state_scoped: StateScoped(Screen::Game), },
            machine: M(machine),
        }
    }
    pub fn pos(&self) -> Vec2 {
        self.com.transform.translation.xy()
    }
    /// To be able to create invisible machines easily: 
    /// ```
    /// let machine = MachineComponent::new().invisible();
    /// ```
    pub fn invisible(mut self) -> Self {
        self.com.vis = Visibility::Hidden;
        self
    }

    // pub fn update(&mut self) {
    //     self.machine.update(&mut self.com);
    // }
}

impl std::ops::Deref for Machine {
    type Target = Box<dyn MachineTrait>;

    fn deref(&self) -> &Self::Target {
        &self.machine
    }
}
impl std::ops::DerefMut for Machine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.machine
    }
}

#[derive(Bundle)]
pub struct MachineCommon {
    pub sprite: Sprite,
    pub vis: Visibility,
    pub transform: Transform,
    _state_scoped: StateScoped<Screen>,
}
impl std::fmt::Debug for MachineCommon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachineCommon").field("sprite", &self.sprite).field("vis", &self.vis).field("transform", &self.transform).finish()
    }
}

pub fn spawn_machine(
    mats: Res<MachineMaterialsHandles>,
    ty: MachineType,
    world_pos: Vec2,
) -> Machine {
    Machine::new(
        Sprite::from_image(mats.imgs[ty.as_index()].clone_weak()),
        world_pos, machine_box_from_ty(ty)
    )
}


/// Small wrapper around box to implement Component
#[derive(Component, Debug)]
pub struct M(pub Box<dyn MachineTrait>);
impl std::ops::Deref for M {
    type Target = Box<dyn MachineTrait>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for M{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}