use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Playing),
        );
        //.add_system_set(
        //    SystemSet::on_enter(AppState::Loading)
        //        .with_system(debug_entering)
        //);
    }
}

//fn debug_entering(mut _cmd: Commands) {
//    info!("Entering Loading state");
//}
