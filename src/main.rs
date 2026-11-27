mod dungeon_generator;

use bevy::prelude::*;

const TILE_SIZE: f32 = 20.0;

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

    let grid = dungeon_generator::generate_dungeon();
    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let x = (j as f32 - (grid[0].len() as f32 - 1.0) / 2.0) * TILE_SIZE;
            let y = ((grid.len() as f32 - 1.0) / 2.0 - i as f32) * TILE_SIZE;
            if *tile == "." {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                    MeshMaterial2d(materials.add(Color::linear_rgb(0.6, 0.3, 0.0))),
                    Transform::from_xyz(x, y, 0.0),
                ));

            } else {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                    MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 0.0, 0.0))),
                    Transform::from_xyz(x, y, 0.0),
                ));
            }
        }
    }
}
