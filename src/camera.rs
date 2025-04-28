use crate::{
    prelude::*,
    screens::game::PlayerAction,
};

#[derive(Component)]
pub struct PlayerCamera {
    movement_speed: Vec2,
    movement_accel: f32,
}

pub fn add(mut commands: Commands) {
    let input_map = InputMap::new([
        (PlayerAction::Up, KeyCode::KeyW),
        (PlayerAction::Down, KeyCode::KeyS),
        (PlayerAction::Right, KeyCode::KeyD),
        (PlayerAction::Left, KeyCode::KeyA),
    ]);
    commands.spawn((
        Camera2d, 
        // bevy_vello::render::VelloView, 
        Transform::from_xyz(0., 0., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera {
            movement_speed: Vec2::ZERO,
            movement_accel: 5.0,
        },
        InputManagerBundle::with_map(input_map),
    ));
}

pub fn movement(
    mut camera: Query<(
        &ActionState<PlayerAction>,
        &mut Transform,
        &mut PlayerCamera,
    )>,
) {
    let (actions, mut trans, mut cam) = camera.single_mut().unwrap();
    if actions.pressed(&PlayerAction::Up) {
        cam.movement_speed.y += cam.movement_accel;
    }
    if actions.pressed(&PlayerAction::Down) {
        cam.movement_speed.y -= cam.movement_accel;
    }
    if actions.pressed(&PlayerAction::Left) {
        cam.movement_speed.x -= cam.movement_accel;
    }
    if actions.pressed(&PlayerAction::Right) {
        cam.movement_speed.x += cam.movement_accel;
    }
    cam.movement_speed = cam.movement_speed.clamp_length_max(cam.movement_accel * 5.);
    trans.translation.x += cam.movement_speed.x;
    trans.translation.y += cam.movement_speed.y;
    // No movement
    cam.movement_speed *= 0.6;
}

pub fn zoom(
    mut evr_scroll: EventReader<input::mouse::MouseWheel>,
    mut camera: Query<(&mut Transform, &mut PlayerCamera)>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                // ev.y between -1 and 1, so we can scale it
                let mut t = camera.single_mut().unwrap().0;
                let scale = ev.y * -1.;
                t.scale.x += scale;
                t.scale.x = t.scale.x.abs().clamp(0.1, 1000.);
                t.scale.y += scale;
                t.scale.y = t.scale.y.abs().clamp(0.1, 1000.);
            }
            MouseScrollUnit::Pixel => {
                todo!("Pixel scrolling");
            }
        }
    }
}

pub fn screen_to_world_pos(
    screen_pos: Vec2,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Vec2 {
    let (camera, camera_transform) = camera_q.single().unwrap();
    camera
        .viewport_to_world_2d(camera_transform, screen_pos)
        .unwrap()
}
