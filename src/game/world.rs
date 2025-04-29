use noise::Perlin;
use strum::{EnumProperty, IntoEnumIterator};

use crate::*;

use super::{machines::{MachineCommon, MachineTag}, particles::{BackgroundMaterial, Velocity}};


#[derive(Resource)]
pub struct CachedImages {
    pub imgs: Vec<Handle<Image>>,
    // pub mats: Vec<Handle<ColorMaterial>>,
    // pub cell_mesh: Handle<Mesh>,
}


pub fn cache_images(
    mut cmd: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut mats_assets: ResMut<Assets<ColorMaterial>>,
    images: Res<AssetServer>,
) {
    // bevy::log::info!("Loading textures...");
    // let all_assets = images.load_folder("./");
    // for ty in AssetPaths::iter() {
    //     let img = images.load(ty.get_str("path").unwrap());
    //     // let mut c = ColorMaterial::from_color(Color::WHITE);
    //     // c.texture = Some(img.clone_weak());
    //     // let mat = mats_assets.add(c);
    //     imgs.push(img);
    //     // mats.push(mat);
    // }

    // cmd.insert_resource(MachineMaterialsHandles {
    //     imgs,
    //     // mats,
    //     // cell_mesh: meshes.add(Rectangle::from_length(1.)),
    // });
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

// pub fn show_inventory(mut cmd: Commands, cell_pos: Vec2, machine: &Machine) {
//     cmd.spawn((widget::ui_root(machine.name()), children![(
//         widget::label(machine.name()),
//     )], StateScoped(Screen::Game)));
// }

pub fn update(
    // mut world: ResMut<World>,
    mut cmd: Commands,
    mut machines: Query<(&mut Sprite, &mut Visibility, &mut Transform, &StateScoped<Screen>, &mut MachineTag)>,
    mut particles: Query<(&mut Sprite, &mut Visibility, &mut Transform, &StateScoped<Screen>, &mut Velocity), (With<game::particles::ParticleTag>, Without<MachineTag>)>,
) {
    for (mut sprite, mut vis, mut transform,_state_scoped, mut m) in &mut machines { // Shared machine behaviour
        
    }
    for mut p in &mut particles { // Shared particle behaviour
        p.2.translation = (p.2.translation.xy()+p.4.0).extend(1.);
    }
}

pub fn spawn_background(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    mut cmd: Commands,
    w: Query<&Window>,
) {
    let w = w.single().unwrap();
    cmd.spawn((
        Mesh2d(meshes.add(Rectangle::new(w.width(), w.height()))),
        MeshMaterial2d(materials.add(BackgroundMaterial { seed: 1 })),
        Transform::from_xyz(0., 0., 0.),
    ));
}

