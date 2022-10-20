use crate::prelude::*;

pub fn spawn_main_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(Name::new("Main Camera"));
}

pub fn spawn_directional_light(mut cmd: Commands) {
    cmd.spawn_bundle(DirectionalLightBundle {
        ..default()
    });
}

pub fn spawn_table(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let debug_material = materials.add(StandardMaterial { base_color: Color::ORANGE, ..default() });
    cmd.spawn_bundle(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::default())),
            material: debug_material.clone(),
           ..default()
    })
    .insert(Name::new("Table"));
}

pub fn spawn_board(mut _cmd: Commands) {}

pub fn spawn_circle(mut _cmd: Commands) {}

pub fn spawn_cross(mut _cmd: Commands) {}