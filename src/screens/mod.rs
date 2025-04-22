use bevy::{app::App, state::{app::AppExtStates as _, state::States}};

pub mod splash;
pub mod title;
pub mod game;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        title::plugin,
        game::plugin,
    ));
}


#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Game,
}