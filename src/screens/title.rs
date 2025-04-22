use bevy::prelude::*;

use crate::{exit_app, widget};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);

    app.add_systems(Update, update_title_screen.run_if(in_state(Screen::Title)));

    app.add_systems(OnExit(Screen::Title), remove_title_screen);

            // .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::Splash))),
}

fn spawn_title_screen(
    mut cmd: Commands,
) {
    cmd
        .spawn((widget::ui_root("Title Screen"), StateScoped(Screen::Title)))
        .with_children(|parent| {
            parent
                .spawn(widget::button("New Game"))
                .observe(enter_game_screen);
            parent
                .spawn(widget::button("Load game"))
                .observe(enter_load_screen);

            parent.spawn(widget::button("Exit")).observe(exit_app);
        });
}
fn update_title_screen(
    mut cmd: Commands,
) {
    
}
fn remove_title_screen(
    mut cmd: Commands,
) {
    
}

fn enter_load_screen(
    _: Trigger<Pointer<Released>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    todo!();
    next_screen.set(Screen::Game);
}




fn enter_game_screen(
    _: Trigger<Pointer<Released>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Game);
}

