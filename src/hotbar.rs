use strum::IntoEnumIterator as _;
use window::PrimaryWindow;

use crate::{camera::{screen_to_cell_pos, screen_to_world_pos}, machines::MachineType, prelude::*, world::{machine_from_ty, Machine, MachineMaterialsHandles, World, TILE_HEIGHT, TILE_WIDTH}};



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
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (button, slot) in slots.iter() {
        match button {
            Interaction::Pressed => {
                let (mut mac, mut vis, mut material) = selected_machine.single_mut().unwrap();
                mac.inner.replace(slot.inner.clone());
                *vis = Visibility::Visible;
                material.0 = machines.mats[slot.inner.as_index()].clone_weak();
                mouse.clear_just_pressed(MouseButton::Left);
            },
            _ => {}
        }
    }
    if keys.just_pressed(KeyCode::Escape) || mouse.just_pressed(MouseButton::Right) {
        let (mut mac, mut vis, mut material) = selected_machine.single_mut().unwrap();
        mac.inner = None;
        *vis = Visibility::Hidden;
        // material.0 = ; Don't change it but shoudln't be an issue
        mouse.clear_just_pressed(MouseButton::Right);
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
    let mut selected_machine = selected_machine.single_mut().unwrap();
    match window_q.single() {
        Ok(w) => {
            if let Some(pos) = w.cursor_position() {
                let t = screen_to_cell_pos(pos, camera_q);
                selected_machine.0.translation.x = t.x as f32 * TILE_WIDTH;
                selected_machine.0.translation.y = t.y as f32 * TILE_HEIGHT;
                if let Some(inner) = selected_machine.1.inner {
                    if mouse.just_pressed(MouseButton::Left) {
                        world.set_tower(t, machine_from_ty(inner));
                        mouse.clear_just_pressed(MouseButton::Left);
                    }
                }
            }
        },
        Err(_) => {},
    }
}
