#![allow(dead_code, unused)]

pub mod camera;
pub mod world;
pub mod hotbar;
pub mod prelude;
pub mod machines;
pub mod saves;
pub mod screens;
pub mod widget;
pub mod helpers;

use prelude::*;

fn main() -> AppExit {
    App::new()
    .add_plugins((
        DefaultPlugins
        .set(WindowPlugin {
            primary_window: Window {
                title: "INITERSE".to_string(),
                fit_canvas_to_parent: true,
                resolution: window::WindowResolution::new(1280., 800.),
                ..default()
            }
            .into(),
            ..default()
        }),
        widget::button_interaction_plugin,
        screens::plugin,
    )).add_systems(Startup, camera::add)
    .run()
}

pub fn exit_app(_: Trigger<Pointer<Released>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
