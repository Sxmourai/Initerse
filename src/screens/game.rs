use crate::*;

use super::Screen;

pub fn plugin(app: &mut App) {
    app.init_asset::<saves::GameConfig>()
    .init_asset_loader::<saves::GameConfigLoader>()
    .add_systems(OnEnter(Screen::Game), (
        world::load_mats_n_mesh,
        world::spawn_world,
        hotbar::spawn_hotbar.after((world::load_mats_n_mesh)),
    ))
    .add_systems(Update, (
        camera::movement,
        camera::zoom,
        world::draw_world,
        hotbar::change_selected,
        hotbar::build_placeholder.after(hotbar::change_selected),
        world::update_changes.after(hotbar::build_placeholder),
    ).run_if(in_state(Screen::Game)));
} 
