use noise::Perlin;
use strum::{EnumProperty, IntoEnumIterator};
use utils::hashbrown::HashMap;

use crate::{machines::{self, MachineType}, prelude::*};

pub const TILE_WIDTH: f32 = 32.0;
pub const TILE_HEIGHT: f32 = 32.0;

// pub const MACHINES: [(MachineType, &str); 4] = [(MachineType::Empty, "empty.png"), (MachineType::Electron, "electron.png"), (MachineType::Energy, "energy.png"), (MachineType::StringCreator, "string creator.png")];



#[derive(Resource)]
pub struct World {
    pub perlin: Perlin,
    pub frame_diff: Vec<(IVec2, Machine)>,
    pub diff: HashMap<IVec2, Machine>,
}
impl World {
    pub fn generate(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            diff: Default::default(),
            frame_diff: Vec::new(),
        }
    }
    pub fn set_tower(&mut self, coords: IVec2, machine: Machine) -> Option<&Machine> {
        if let Some(tower) = self.diff.get(&coords) {
            return Some(tower)
        } else {
            self.frame_diff.push((coords, machine));
            dbg!(coords);
        }
        None
    }
    pub fn get_tower(&self, coords: &IVec2) -> Option<&Machine> {
        self.diff.get(coords)
    }
    pub fn update_changes(&mut self) {
        self.diff.extend(std::mem::take(&mut self.frame_diff).into_iter());
    }
}

pub fn update_changes(
    mut world: ResMut<World>,
    machines: Res<MachineMaterialsHandles>,
    mut cmd: Commands,
) {
    for (pos, machine) in &world.frame_diff {
        dbg!(pos, machine);
        
        cmd.spawn((
            MeshMaterial2d(machines.mats[machine.ty().as_index()].clone_weak()),
            Mesh2d(machines.cell_mesh.clone_weak()),
            Transform::from_translation(Vec3::new(pos.x as f32*TILE_WIDTH, pos.y as f32*TILE_HEIGHT, 1.)),
            Visibility::Visible
        ));
    }
    world.update_changes();
}

#[derive(Resource)]
pub struct MachineMaterialsHandles {
    pub imgs: Vec<Handle<Image>>,
    pub mats: Vec<Handle<ColorMaterial>>,
    pub cell_mesh: Handle<Mesh>,
}

pub type CellMesh = Mesh2d;

pub fn load_mats_n_mesh(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats_assets: ResMut<Assets<ColorMaterial>>,
    images: Res<AssetServer>,
) {
    let mut imgs = vec![];
    let mut mats = vec![];
    for ty in MachineType::iter() {
        let img = images.load(ty.get_str("path").unwrap());
        let mut c = ColorMaterial::from_color(Color::WHITE);
        c.texture = Some(img.clone_weak());
        let mat = mats_assets.add(c);
        imgs.push(img);
        mats.push(mat);
    }

    cmd.insert_resource(MachineMaterialsHandles {
        imgs,
        mats,
        cell_mesh: meshes.add(Rectangle::from_length(TILE_WIDTH)),
    });
}


pub fn spawn_world(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let seed = 0;
    cmd.insert_resource(World::generate(seed));
}
#[derive(Debug)]
pub struct Machine {
    pub _machine: Box<dyn _Machine>,
}
impl Machine {
    pub(crate) fn new_from_machine_type(ty: MachineType) -> Machine {
        match ty {
            MachineType::StringCreator => {
                        Self {
                            _machine: Box::new(StringCreator {}),
                        }
                    },
            MachineType::Electron => {
                        Self {
                            _machine: Box::new(Electron {}),
                        }
                    },
            MachineType::Energy => {
                        Self {
                            _machine: Box::new(Energy {}),
                        }
                    },
            MachineType::Empty => todo!(),
        }
    }
    
    fn ty(&self) -> MachineType {
        self._machine.ty()
    }
}

pub trait _Machine: Sync + Send + std::fmt::Debug {
    fn update(&mut self);
    fn ty(&self) -> MachineType;
}

#[derive(Debug)]
pub struct StringCreator {

}
impl _Machine for StringCreator {
    fn update(&mut self) {
        todo!()
    }
    
    fn ty(&self) -> MachineType {
        MachineType::StringCreator
    }
}
#[derive(Debug)]
pub struct Electron {

}
impl _Machine for Electron {
    fn update(&mut self) {
        todo!()
    }
    
    fn ty(&self) -> MachineType {
        MachineType::Electron
    }
}
#[derive(Debug)]
pub struct Energy {

}
impl _Machine for Energy {
    fn update(&mut self) {
        todo!()
    }
    
    fn ty(&self) -> MachineType {
        MachineType::Energy
    }
}



pub fn draw_world() {

}


// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// pub struct WorldMaterial {
//     #[uniform(0)]
//     color: LinearRgba,
//     #[texture(1)]
//     #[sampler(2)]
//     color_texture: Option<Handle<Image>>,
// }

// impl sprite::Material2d for WorldMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/world_render.wgsl".into()
//     }
// }