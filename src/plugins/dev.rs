use crate::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
    }
}