pub mod camera;
pub mod world;
pub mod hotbar;
pub mod prelude;
pub mod machines;
pub mod saves;
use prelude::*;

fn main() -> AppExit {
    App::new()
    .add_plugins((
        DefaultPlugins,
        // sprite::Material2dPlugin::<world::WorldMaterial>::default(),
        // sprite::Material2dPlugin::<hotbar::MachineMaterial>::default(),
    ))
    .init_asset::<saves::GameConfig>()
    .init_asset_loader::<saves::GameConfigLoader>()
    .add_systems(Startup, (
        camera::add,
        world::load_mats_n_mesh,
        world::spawn_world,
        hotbar::spawn_hotbar.after(world::spawn_world),
    ))
    .add_systems(Update, (
        camera::movement,
        camera::zoom,
        world::draw_world,
        hotbar::change_selected,
        hotbar::build_placeholder,
        world::update_changes.after(hotbar::build_placeholder),
    ))
    .run()
}
