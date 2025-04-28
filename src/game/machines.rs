use std::{
    ptr::NonNull,
    time::{Duration, Instant},
};

use bevy::{
    ecs::{
        bundle::{BundleEffect, DynamicBundle, NoBundleEffect},
        component::Mutable,
    },
    ptr::OwningPtr,
};
use strum_macros::{self, EnumIter, EnumProperty};

use crate::*;

use super::particles::{particle_bundle, ParticleTag};

#[derive(EnumProperty, EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum MachineType {
    #[strum(props(path = "string creator.png", name = "String creator"))]
    StringCreator,
    #[strum(props(path = "electron.png", name = "Electron"))]
    Electron,
    #[strum(props(path = "energy.png", name = "Energy"))]
    Energy,
    // #[strum(props(path = "empty.png", name = "Empty"))]
    // Empty,
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

// #[derive(Debug)]
// pub enum MachineTrait {
//     StringCreator(StringCreator),
//     Electron(Electron),
//     Energy(Energy),
//     Empty,
// }
// impl ecs::bundle::DynamicBundle for MachineTrait {
//     type Effect = ();

//     fn get_components(self, func: &mut impl FnMut(ecs::component::StorageType, ptr::OwningPtr<'_>)) -> Self::Effect {
//         let non_null_machine_ptr = match self {
//             MachineTrait::StringCreator(mut string_creator) => NonNull::new(&mut string_creator as *mut StringCreator as *mut u8),
//             MachineTrait::Electron(mut electron) => NonNull::new(&mut electron as *mut Electron as *mut u8),
//             MachineTrait::Energy(mut energy) => NonNull::new(&mut energy as *mut Energy as *mut u8),
//             MachineTrait::Empty => todo!(),
//         }.unwrap();
//         func(ecs::component::StorageType::Table, unsafe { OwningPtr::new(non_null_machine_ptr) })
//     }
// }
// impl MachineTrait {
//     pub fn update(&mut self, com: MachineCommonMut<'_>, cmd: &mut Commands) {
//         // dbg!(self.name());
//     }
//     pub fn ty(&self) -> MachineType {
//         match self {
//             MachineTrait::StringCreator(string_creator) => MachineType::StringCreator,
//             MachineTrait::Electron(electron) => MachineType::Electron,
//             MachineTrait::Energy(energy) => MachineType::Energy,
//             MachineTrait::Empty => MachineType::Empty,
//         }
//     }
//     pub fn name(&self) -> String {self.ty().name().to_string()}
//     // pub fn inner_component(self) -> impl Bundle {
//     // }
// }

// pub trait MachineTrait: Sync + Send + std::fmt::Debug + Component {
//     fn update(&mut self, com: MachineCommonMut<'_>, cmd: &mut Commands) {
//         dbg!(self.name());
//     }
//     fn ty(&self) -> MachineType;
//     fn name(&self) -> String {self.ty().name().to_string()}
// }
// pub fn machine_box_from_ty(ty: MachineType) -> MachineTrait {
//     match ty {
//         MachineType::StringCreator => MachineTrait::StringCreator(StringCreator::new()),
//         MachineType::Electron => MachineTrait::Electron(Electron {  }),
//         MachineType::Energy => MachineTrait::Energy(Energy {  }),
//         MachineType::Empty => todo!(),
//     }
// }

#[derive(Debug, Component)]
pub struct StringCreator {
    last_creation: Instant,
}
// impl_machine_trait!(StringCreator);
impl StringCreator {
    pub fn new() -> Self {
        Self {
            last_creation: Instant::now(),
        }
    }
}

#[derive(Debug, Component)]
pub struct Electron {}

#[derive(Debug, Component)]
pub struct Energy {}

// #[macro_export]
// macro_rules! impl_machine_trait {
//     ($struct_name: tt) => {

//     };
//     ($struct_name: tt, $update_fn: expr) => {
//         impl MachineTrait for $struct_name {

//             fn ty(&self) -> MachineType {
//                 MachineType::$struct_name
//             }
//             fn update(&mut self, com: MachineCommonMut<'_>) {
//                 let f: &dyn FnOnce(MachineCommonMut<'_>) -> () = &$update_fn;
//                 $update_fn(com)
//             }
//         }
//     };
// }

// #[derive(Debug)]
// pub struct Machine {
//     pub com: MachineCommon,
//     pub machine: MachineTrait,
// }

// impl Machine {
//     pub fn new(sprite: Sprite, pos: Vec2, machine: MachineTrait) -> Self {
//         Self {
//             com: MachineCommon { sprite, vis: Visibility::Visible, transform: Transform::from_translation(pos.extend(1.)), _state_scoped: StateScoped(Screen::Game), },
//             machine,
//         }
//     }
//     pub fn pos(&self) -> Vec2 {
//         self.com.transform.translation.xy()
//     }
//     /// To be able to create invisible machines easily:
//     /// ```
//     /// let machine = MachineComponent::new().invisible();
//     /// ```
//     pub fn invisible(mut self) -> Self {
//         self.com.vis = Visibility::Hidden;
//         self
//     }

//     // pub fn as_bundle(self) -> impl Bundle {
//     //     (self.com, )
//     // }

//     // pub fn update(&mut self) {
//     //     self.machine.update(&mut self.com);
//     // }
// }

// impl std::ops::Deref for Machine {
//     type Target = MachineTrait;

//     fn deref(&self) -> &Self::Target {
//         &self.machine
//     }
// }
// impl std::ops::DerefMut for Machine {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.machine
//     }
// }

#[derive(Bundle)]
pub struct MachineCommon {
    pub sprite: Sprite,
    pub vis: Visibility,
    pub transform: Transform,
    pub _state_scoped: StateScoped<Screen>,
}
impl std::fmt::Debug for MachineCommon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachineCommon")
            .field("sprite", &self.sprite)
            .field("vis", &self.vis)
            .field("transform", &self.transform)
            .finish()
    }
}

// pub fn spawn_machine(
//     mats: Res<MachineMaterialsHandles>,
//     ty: MachineType,
//     world_pos: Vec2,
// ) -> Machine {
//     Machine::new(
//         Sprite::from_image(mats.imgs[ty.as_index()].clone_weak()),
//         world_pos, machine_box_from_ty(ty)
//     )
// }

#[derive(Component, Debug)]
pub struct MachineTag;

pub fn string_creator_updates(
    mut cmd: Commands,
    string_creators: Query<(&mut StringCreator, &Transform)>,
    assets: Res<GameAssets>,
) {
    for (mut s, trans) in string_creators {
        if s.last_creation.elapsed() > Duration::from_secs(1) {
            let mut p = particle_bundle(trans.translation.xy(), Vec2::ONE, get_image(&assets, "proton.png").unwrap().to_sprite());
            p.0.0.x = 1.; // Velocity
            cmd.spawn(p);
            s.last_creation = Instant::now();
        }
    }
}

pub fn electron_updates(
    mut cmd: Commands,
    electrons: Query<(&mut Electron, &Transform), Without<ParticleTag>>,
    mut particles: Query<(&Transform, &mut Visibility), With<ParticleTag>>,
) {
    for (mut e, machine_pos) in electrons {
        particles.par_iter_mut().for_each(|(p_pos, mut p_vis)| {
            if machine_pos
                .translation
                .xy()
                .distance_squared(p_pos.translation.xy())
                < 100.
            {
                *p_vis = Visibility::Hidden;
                // e.inventory.push()
            }
        });
    }
}


