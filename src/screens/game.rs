use bevy::input::common_conditions::input_just_pressed;

use crate::*;
use crate::game::*;

use super::Screen;

pub fn plugin(app: &mut App) {
    app.init_asset::<saves::GameConfig>()
    .init_asset_loader::<saves::GameConfigLoader>()
    .add_plugins(InputManagerPlugin::<GameAction>::default())
    .add_plugins(InputManagerPlugin::<PlayerAction>::default())
    .add_plugins(bevy_prototype_lyon::plugin::ShapePlugin)
    .add_plugins(sprite::Material2dPlugin::<particles::BackgroundMaterial>::default())
    .add_systems(OnEnter(Screen::Game), (
        world::cache_images,
        spawn_guis,
        hotbar::spawn_hotbar.after((world::cache_images)),
        particles::enemy_particle_random_motion_setup,
        world::spawn_background,
    ))
    .add_systems(Update, (
        camera::movement,
        camera::zoom,
        // world::mouse_interact.before(update_changes),
        hotbar::change_selected,
        // world::update_changes.after(hotbar::build_placeholder),
        hotbar::build_placeholder,
        world::update,
        machines::string_creator_updates,
        machines::electron_updates,
        particles::enemy_particle_random_motion,
        show_options.run_if(input_just_pressed(KeyCode::Escape))
    ).run_if(in_state(Screen::Game)))
    ;
} 


#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GameAction {
    PlayerAction,
    OpenOptionsScreen,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct OptionsMenu;
#[derive(Component)]
pub struct SettingsMenu;

pub fn spawn_guis(
    mut cmd: Commands,
) {
    cmd.spawn((
        widget::ui_root("Options menu"), StateScoped(Screen::Game), Visibility::Hidden, OptionsMenu
    )).with_children(|parent| {
        parent.spawn(widget::label("Options menu"));

        parent.spawn(widget::button("Resume")).observe(hide_options);
        parent.spawn(widget::button("Settings")).observe(show_settings);
        parent.spawn(widget::button("Save & Quit")).observe(save_and_quit);
        parent.spawn(widget::button("Quit")).observe(quit);
    });
    
    cmd.spawn((
        widget::ui_root("Settings menu"), StateScoped(Screen::Game), Visibility::Hidden, SettingsMenu
    )).with_children(|parent| {
        parent.spawn(widget::label("Settings menu"));

        // parent.spawn(widget::button("Controls")).observe(show_controls);
        parent.spawn(widget::button("Back")).observe(hide_settings);
    });
}

pub fn show_options(
    mut options_query: Query<&mut Visibility, (With<OptionsMenu>, Without<SettingsMenu>)>
) {
    let mut vis = options_query.single_mut().unwrap();
    *vis = Visibility::Visible;
}

pub fn hide_options(
    _: Trigger<Pointer<Released>>,
    mut options_query: Query<&mut Visibility, (With<OptionsMenu>, Without<SettingsMenu>)>
) {
    let mut vis = options_query.single_mut().unwrap();
    *vis = Visibility::Hidden;
}

pub fn show_settings(
    trigger: Trigger<Pointer<Released>>,
    mut settings_q: Query<&mut Visibility, With<SettingsMenu>>,
    mut options_q: Query<&mut Visibility, (With<OptionsMenu>, Without<SettingsMenu>)>,
) {
    let mut vis = settings_q.single_mut().unwrap();
    *vis = Visibility::Visible;
    hide_options(trigger, options_q);
}
pub fn hide_settings(
    _: Trigger<Pointer<Released>>,
    mut settings_q: Query<&mut Visibility, With<SettingsMenu>>,
    mut options_q: Query<&mut Visibility, (With<OptionsMenu>, Without<SettingsMenu>)>,
) {
    dbg!();
    let mut vis = settings_q.single_mut().unwrap();
    *vis = Visibility::Hidden;
    show_options(options_q);
}

pub fn save_and_quit(
    trigger: Trigger<Pointer<Released>>,
    mut next_screen: ResMut<NextState<Screen>>,

) {
    // Save
    todo!("Save !");
    quit(trigger, next_screen)
}

pub fn quit(
    _: Trigger<Pointer<Released>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Title);
}
