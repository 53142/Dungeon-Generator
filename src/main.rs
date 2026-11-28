mod dungeon_generator;

use bevy::prelude::*;

use crate::dungeon_generator::Tile;

const TILE_SIZE: f32 = 20.0;

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

    // Premake so can just clone each one instead of recreating them entirely
    let mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));
    let floor_material = materials.add(Color::linear_rgb(0.0, 0.0, 0.0));
    let wall_material = materials.add(Color::linear_rgb(0.2, 0.1, 0.0));

    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let x = (j as f32 - (grid[0].len() as f32 - 1.0) / 2.0) * TILE_SIZE;
            let y = ((grid.len() as f32 - 1.0) / 2.0 - i as f32) * TILE_SIZE;
            let material = match tile {
                Tile::Floor => &floor_material,
                Tile::Wall => &wall_material,
            };

            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
