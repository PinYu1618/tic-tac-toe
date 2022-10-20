use crate::*;

pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(debug_entering)
                .with_system(systems::spawn_main_camera)
                .with_system(systems::spawn_directional_light)
                .with_system(systems::spawn_table)
                .with_system(systems::spawn_board)
        );
    }
}

fn debug_entering(mut _cmd: Commands) {
    info!("Entering `Playing` state");
}