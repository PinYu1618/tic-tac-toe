// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, pbr::PbrPlugin};
use tic_tac_toe::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Tic Tac Toe".to_string(),
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PbrPlugin)
        .add_plugin(GamePlugin)
        .run();
}
