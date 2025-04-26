use noise::Perlin;
use strum::{EnumProperty, IntoEnumIterator};

use crate::*;

use super::{machines::{MachineCommon, M}, particles::ParticleComponent};


#[derive(Resource)]
pub struct World {
    // pub frame_machines: Vec<(Vec2, Machine)>,
    pub machines: Vec<Entity>,
    pub particles: Vec<ParticleComponent>,
    pub config: Handle<GameConfig>,
}
impl World {
    pub fn generate(config: Handle<GameConfig>) -> Self {
        Self {
            config,
            // frame_machines: Default::default(),
            machines: Default::default(),
            particles: Default::default(),
        }
    }
    pub fn set_machine(&mut self, coords: Vec2, machine: Entity) {
        // if let Some(machine) = self.machines.get(&coords) {
        //     return Some(machine);
        // } else {
        //     self.frame_machines.push((coords, machine));
        // }
        // None
        self.machines.push(machine)
        // self.frame_machines.push((coords, machine))
    }
    pub fn get_machine(&self, coords: &Vec2) -> Option<&Machine> {
        todo!()
        // self.machines.get(coords)
    }
    pub fn update_changes(&mut self) {
        todo!()
        // self.machines
        //     .extend(std::mem::take(&mut self.frame_machines).into_iter());
    }
}

pub fn update_changes(
    mut world: ResMut<World>,
    machines: Res<MachineMaterialsHandles>,
    mut cmd: Commands,
) {
    // for (pos, machine) in &world.frame_machines {
    //     cmd.spawn((
    //         Sprite::from_image(machines.imgs[machine.ty().as_index()].clone_weak()),
    //         // Mesh2d(machines.cell_mesh.clone_weak()),
    //         Transform::from_translation(Vec3::new(
    //             pos.x,
    //             pos.y,
    //             1.,
    //         )),
    //         Visibility::Visible, StateScoped(Screen::Game)
    //     ));
    // }
    // world.update_changes();
}

#[derive(Resource)]
pub struct MachineMaterialsHandles {
    pub imgs: Vec<Handle<Image>>,
    // pub mats: Vec<Handle<ColorMaterial>>,
    // pub cell_mesh: Handle<Mesh>,
}

pub type CellMesh = Mesh2d;

pub fn load_mats_n_mesh(
    mut cmd: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut mats_assets: ResMut<Assets<ColorMaterial>>,
    images: Res<AssetServer>,
) {
    bevy::log::info!("Loading materials & meshes...");
    let mut imgs = vec![];
    // let mut mats = vec![];
    for ty in MachineType::iter() {
        let img = images.load(ty.get_str("path").unwrap());
        // let mut c = ColorMaterial::from_color(Color::WHITE);
        // c.texture = Some(img.clone_weak());
        // let mat = mats_assets.add(c);
        imgs.push(img);
        // mats.push(mat);
    }

    cmd.insert_resource(MachineMaterialsHandles {
        imgs,
        // mats,
        // cell_mesh: meshes.add(Rectangle::from_length(1.)),
    });
}

pub fn spawn_world(mut cmd: Commands, mut asset_server: ResMut<AssetServer>) {
    cmd.insert_resource(World::generate(asset_server.load("saves/1.ron")));
}

// pub fn mouse_interact(
//     window_q: Query<&Window>,
//     mut mouse: ResMut<ButtonInput<MouseButton>>,
//     camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
//     world: Res<World>,
//     mut cmd: Commands,
// ) {
//     if let Ok(w) = window_q.single() {
//         if let Some(pos) = w.cursor_position() {
//             let cell_pos = screen_to_cell_pos(pos, camera_q);
//             if let Some(machine) = world.get_machine(&cell_pos) {
//                 if mouse.just_pressed(MouseButton::Left) {
//                     show_inventory(cmd, cell_pos, machine);
//                     mouse.clear_just_pressed(MouseButton::Left);
//                 }
//             }
//         }
//     }
// }

pub fn show_inventory(mut cmd: Commands, cell_pos: Vec2, machine: &Machine) {
    cmd.spawn((widget::ui_root(machine.name()), children![(
        widget::label(machine.name()),
    )], StateScoped(Screen::Game)));
}

pub fn update(
    mut world: ResMut<World>,
    mut cmd: Commands,
    mut machines: Query<(&mut Sprite, &mut Visibility, &mut Transform, &StateScoped<Screen>, &mut M)>,
) {
    for (mut sprite, mut vis, mut transform,_state_scoped, mut m) in &mut machines {
        m.update((&mut sprite,&mut vis,&mut transform));
    }
    for particle in &mut world.particles {
        particle.update();
    }
}

