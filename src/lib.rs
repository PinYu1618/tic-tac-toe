pub mod pieces;
pub mod plugins;
pub mod systems;

// Internal `prelude` for convenient
mod prelude {
    pub use bevy::prelude::*;
}

use bevy::prelude::*;
use self::plugins::*;
//#[cfg(debug_assertions)]
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    _Pausing,
    _Restarting,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Playing)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(DevPlugin)
            .add_plugin(TicTacToePlugin);

        //#[cfg(debug_assertions)]
        //{
        //    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //        .add_plugin(LogDiagnosticsPlugin::default());
        //}
    }
}
