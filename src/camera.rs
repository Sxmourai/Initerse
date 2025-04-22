use crate::{prelude::*, world::{TILE_HEIGHT, TILE_WIDTH}};


#[derive(Component)]
pub struct PlayerCamera {
    movement_speed: Vec2,
    movement_accel: f32,
}


pub fn add(
    mut commands: Commands,

) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 1.).looking_at(Vec3::ZERO, Vec3::Y), PlayerCamera {
        movement_speed: Vec2::ZERO,
        movement_accel: 2.0,
    }));
}

pub fn movement(
    mut camera: Query<(&mut Transform, &mut PlayerCamera)>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let (mut trans, mut cam) = camera.single_mut().unwrap();
    if inputs.pressed(KeyCode::KeyW) {
        cam.movement_speed.y += cam.movement_accel;
    }
    if inputs.pressed(KeyCode::KeyS) {
        cam.movement_speed.y -= cam.movement_accel;
    }
    if inputs.pressed(KeyCode::KeyA) {
        cam.movement_speed.x -= cam.movement_accel;
    }
    if inputs.pressed(KeyCode::KeyD) {
        cam.movement_speed.x += cam.movement_accel;
    }
    cam.movement_speed = cam.movement_speed.clamp_length_max(cam.movement_accel*5.);
    trans.translation.x += cam.movement_speed.x;
    trans.translation.y += cam.movement_speed.y;
    if !inputs.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD]) {
        cam.movement_speed *= 0.6;
    }
}

pub fn zoom(
    mut evr_scroll: EventReader<input::mouse::MouseWheel>,
    mut camera: Query<(&mut Transform, &mut PlayerCamera)>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => { // ev.y between -1 and 1, so we can scale it
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
    camera.viewport_to_world_2d(camera_transform, screen_pos).unwrap()
}

pub fn screen_to_cell_pos(
    screen_pos: Vec2,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> IVec2 {
    let mut wp = screen_to_world_pos(screen_pos, camera_q);
    wp.x += TILE_WIDTH/2.;
    wp.y += TILE_HEIGHT/2.;
    let tx = (wp.x / TILE_WIDTH).floor() as i32;
    let ty = (wp.y / TILE_HEIGHT).floor() as i32;
    IVec2::new(tx, ty)
}
