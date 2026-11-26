mod dungeon_generator;

use bevy::prelude::*;

// Simple dungeon generator
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create default camera
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
