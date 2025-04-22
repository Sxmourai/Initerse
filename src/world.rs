use noise::Perlin;
use strum::{EnumProperty, IntoEnumIterator};

use crate::{
    camera::{screen_to_cell_pos, screen_to_world_pos},
    machines::{self, MachineType},
    prelude::*,
    saves::GameConfig,
    widget,
};

pub const TILE_WIDTH: f32 = 32.0;
pub const TILE_HEIGHT: f32 = 32.0;

// pub const MACHINES: [(MachineType, &str); 4] = [(MachineType::Empty, "empty.png"), (MachineType::Electron, "electron.png"), (MachineType::Energy, "energy.png"), (MachineType::StringCreator, "string creator.png")];

#[derive(Resource)]
pub struct World {
    pub frame_diff: Vec<(IVec2, Machine)>,
    pub diff: std::collections::HashMap<IVec2, Machine>,
    pub config: Handle<GameConfig>,
}
impl World {
    pub fn generate(config: Handle<GameConfig>) -> Self {
        Self {
            diff: Default::default(),
            frame_diff: Vec::new(),
            config,
        }
    }
    pub fn set_tower(&mut self, coords: IVec2, machine: Machine) -> Option<&Machine> {
        if let Some(tower) = self.diff.get(&coords) {
            return Some(tower);
        } else {
            self.frame_diff.push((coords, machine));
        }
        None
    }
    pub fn get_tower(&self, coords: &IVec2) -> Option<&Machine> {
        self.diff.get(coords)
    }
    pub fn update_changes(&mut self) {
        self.diff
            .extend(std::mem::take(&mut self.frame_diff).into_iter());
    }
}

pub fn update_changes(
    mut world: ResMut<World>,
    machines: Res<MachineMaterialsHandles>,
    mut cmd: Commands,
) {
    for (pos, machine) in &world.frame_diff {
        cmd.spawn((
            MeshMaterial2d(machines.mats[machine.ty().as_index()].clone_weak()),
            Mesh2d(machines.cell_mesh.clone_weak()),
            Transform::from_translation(Vec3::new(
                pos.x as f32 * TILE_WIDTH,
                pos.y as f32 * TILE_HEIGHT,
                1.,
            )),
            Visibility::Visible,
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
    bevy::log::info!("Loading materials & meshes...");
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

pub fn spawn_world(mut cmd: Commands, mut asset_server: ResMut<AssetServer>) {
    cmd.insert_resource(World::generate(asset_server.load("saves/1.ron")));
}

pub type Machine = Box<dyn MachineTrait>;

pub trait MachineTrait: Sync + Send + std::fmt::Debug {
    fn update(&mut self);
    fn ty(&self) -> MachineType;
    fn name(&self) -> String {"Electron".to_string()}
}
pub fn machine_from_ty(ty: MachineType) -> Machine {
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
    fn update(&mut self) {
        todo!()
    }

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
    fn update(&mut self) {
        todo!()
    }

    fn ty(&self) -> MachineType {
        MachineType::Electron
    }
}
#[derive(Debug)]
pub struct Energy {}
impl MachineTrait for Energy {
    fn update(&mut self) {
        todo!()
    }

    fn ty(&self) -> MachineType {
        MachineType::Energy
    }
}

pub fn mouse_interact(
    window_q: Query<&Window>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    world: Res<World>,
    mut cmd: Commands,
) {
    if let Ok(w) = window_q.single() {
        if let Some(pos) = w.cursor_position() {
            let cell_pos = screen_to_cell_pos(pos, camera_q);
            if let Some(machine) = world.get_tower(&cell_pos) {
                if mouse.just_pressed(MouseButton::Left) {
                    show_inventory(cmd, cell_pos, machine);
                    mouse.clear_just_pressed(MouseButton::Left);
                }
            }
        }
    }
}

pub fn show_inventory(mut cmd: Commands, cell_pos: IVec2, machine: &Machine) {
    cmd.spawn((widget::ui_root(machine.name()), children![(
        widget::label(machine.name()),
    )]));
}
