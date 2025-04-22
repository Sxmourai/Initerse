use strum::IntoEnumIterator as _;
use window::PrimaryWindow;

use crate::{camera::screen_to_world_pos, machines::MachineType, prelude::*, world::{Machine, MachineMaterialsHandles, World, TILE_HEIGHT, TILE_WIDTH}};



#[derive(Component, Debug)]
pub struct Slot {
    pub inner: MachineType,
}
#[derive(Component, Debug)]
pub struct SelectedMachine {
    pub inner: Option<MachineType>,
}


pub fn spawn_hotbar(
    mut cmd: Commands,
    machines: Res<MachineMaterialsHandles>,
) {
    for (i, ty) in MachineType::iter().enumerate() {
        let x = i as f32*100.+200.;
        cmd.spawn((
            Button,
            Slot {
                inner: ty,
            }, 
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                left: Val::Px(x), top: Val::Percent(90.),
                ..Default::default()
            }, 
            ImageNode::new(machines.imgs[i].clone_weak())
        ));
    }

    cmd.spawn((
        SelectedMachine {inner: None},
        MeshMaterial2d(machines.mats[0].clone_weak()),
        Mesh2d(machines.cell_mesh.clone_weak()),
        Visibility::Hidden,
        Transform::default(),
    ));
}


pub fn change_selected(
    slots: Query<(&Interaction, &Slot), (Changed<Interaction>, With<Button>)>,
    mut selected_machine: Query<(&mut SelectedMachine, &mut Visibility, &mut MeshMaterial2d<ColorMaterial>)>,
    machines: Res<MachineMaterialsHandles>,
) {
    for (button, slot) in slots.iter() {
        match button {
            Interaction::Pressed => {
                let (mut mac, mut vis, mut material) = selected_machine.single_mut();
                mac.inner.replace(slot.inner.clone());
                *vis = Visibility::Visible;
                material.0 = machines.mats[slot.inner.as_index()].clone_weak();
            },
            _ => {}
        }
    }
}

pub fn build_placeholder(
    mut selected_machine: Query<(&mut Transform, &SelectedMachine)>,
    mut cmd: Commands,
    window_q: Query<&Window>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut world: ResMut<World>,
    machines: Res<MachineMaterialsHandles>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let mut selected_machine = selected_machine.single_mut();
    match window_q.get_single() {
        Ok(w) => {
            if let Some(pos) = w.cursor_position() {
                let mut pos = screen_to_world_pos(pos, camera_q);
                pos.x += TILE_WIDTH/2.;
                pos.y += TILE_HEIGHT/2.;
                let tx = (pos.x / TILE_WIDTH).floor() as i32;
                let ty = (pos.y / TILE_HEIGHT).floor() as i32;
                pos.x = tx as f32 * TILE_WIDTH;
                pos.y = ty as f32 * TILE_HEIGHT;
                selected_machine.0.translation.x = pos.x;
                selected_machine.0.translation.y = pos.y;
                if let Some(inner) = selected_machine.1.inner {
                    if mouse.just_pressed(MouseButton::Left) {
                        world.set_tower(IVec2::new(tx, ty), Machine::new_from_machine_type(inner));
                        mouse.clear_just_pressed(MouseButton::Left);
                    }
                }
            }
        },
        Err(_) => {},
    }
}
