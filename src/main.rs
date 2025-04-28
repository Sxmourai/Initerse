#![allow(dead_code, unused)]

pub mod camera;
pub mod game;
pub mod helpers;
pub mod prelude;
pub mod saves;
pub mod screens;
pub mod widget;

pub use prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "INITERSE".to_string(),
                    fit_canvas_to_parent: true,
                    resolution: window::WindowResolution::new(1280., 800.),
                    ..default()
                }
                .into(),
                ..default()
            }),
            screens::plugin,
            // External plugins
            widget::button_interaction_plugin, // From bevy_new_2d, simple buttons
            bevy_inspector_egui::bevy_egui::EguiPlugin {
                enable_multipass_for_primary_context: true,
            }, // For bevy_inspector_egui
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(), // Bevy_inspector_egui, basic inspector
                                                                     // bevy_vello::VelloPlugin::default(),
        ))
        .add_loading_state(
            LoadingState::new(Screen::Splash)
                .continue_to_state(Screen::Title)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "dynamic_asset.assets.ron",
                )
                .load_collection::<helpers::GameAssets>(),
        )
        .add_systems(Startup, camera::add)
        .add_systems(
            Update,
            helpers::cleanup_system::<helpers::MarkedForDeletion>,
        )
        .run()
}

pub fn exit_app(_: Trigger<Pointer<Released>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
