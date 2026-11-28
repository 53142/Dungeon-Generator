use std::fmt;

use rand::RngExt;
use rand_distr::{Distribution, Normal};

const DUNGEON_HEIGHT: usize = 20;
const DUNGEON_WIDTH: usize = 40;

const MAX_ROOM_AREA: usize = 25;
const MIN_ROOM_AREA: usize = 4;
const MAX_ROOMS: usize = 10;

const MAX_FAILED_ATTEMPTS: usize = 100;

// Struct to represent the different types of tiles
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}

// Custom debug implementation to actually render the symbols to be readable
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Wall => write!(f, "."),
            Self::Floor => write!(f, " "),
        }
    }

}

pub fn generate_dungeon() -> [[Tile; DUNGEON_WIDTH]; DUNGEON_HEIGHT] {
    let mut grid = [[Tile::Wall; DUNGEON_WIDTH]; DUNGEON_HEIGHT];

    // Create the rooms themselves
    let rooms = make_rooms(&mut grid);

    // Create hallways to connect the rooms together
    connect_rooms(&mut grid, &rooms);

    grid
}

// Modifies the grid passed in to have rooms
// First choose a random room length & random room height
// Next, choose a random coordinate for the top-left corner of the room
// the coord should be at least 1 tile away from the top edge & the left edge
// and > the room height from the bottom edge and > the room width from the right edge

// Returns a vector with the center of each room cord for connecting the rooms together easily

// TODO: Completely replace this guess and check algorithm with something way more efficient
fn make_rooms(grid: &mut [[Tile; DUNGEON_WIDTH]; DUNGEON_HEIGHT]) -> Vec<(usize, usize)> {
    let mut rooms = Vec::new();
    let mut failed_attempts = 0;

    // Determine room dimensions
    let mean = ((MIN_ROOM_AREA + MAX_ROOM_AREA) as f64 / 2.0).sqrt();
    // Controls how much variation there is in room dimensions.
    let distribution = Normal::new(mean, 0.75).unwrap();

    let mut rng = rand::rng();

    let min_side = (MIN_ROOM_AREA as f64).sqrt().ceil();
    let max_side = (MAX_ROOM_AREA as f64).sqrt().floor();
    loop {
        let room_width = distribution
            .sample(&mut rng)
            .round()
            .clamp(min_side, max_side) as usize;

        let room_height = distribution
            .sample(&mut rng)
            .round()
            .clamp(min_side, max_side) as usize;
        
        // Determine room cordinates
        // Get random coords for top-left corner of room, making sure the room doesn't exceed the boundaries of the dungeon
        let row = rng.random_range(1..(grid.len() - room_height));
        let col = rng.random_range(1..(grid[0].len() - room_width));

        // Check if room would intersect with other rooms

        // Very simple algorithm that just checks each coord in room and makes sure it doesn't intersect other rooms
        let overlaps = grid[(row - 1)..(row + room_height + 1)]
            .iter()
            .any(|a| {
                a[(col - 1)..(col + room_width + 1)].contains(&Tile::Floor)
            });
        
        if overlaps {
            failed_attempts += 1;
            if failed_attempts >= MAX_FAILED_ATTEMPTS {
                break;
            }

            continue;
        }

        // Create room
        for r in row..(row + room_height) {
            for c in col..(col + room_width) {
                grid[r][c] = Tile::Floor;
            }
        }

        // Add room center to vec
        rooms.push((row + room_height / 2, col + room_width / 2));

        if rooms.len() == MAX_ROOMS {
            break;
        }

        // Reset failed attempts bc just got a success
        failed_attempts = 0;
    }
    rooms
}

// Simple algorithm that connects each room horizontally and vertically in order that it was added
// The algorithm could definitely be optimized but it does guarantee a connection between every room which is fine for now
fn connect_rooms(grid: &mut [[Tile; DUNGEON_WIDTH]; DUNGEON_HEIGHT], rooms: &[(usize, usize)]) {
    // For each pair of room center coords next to each other
    // Basically, for each room center in order of generation
    for p in rooms.windows(2) {
        let (r1, c1) = p[0];
        let (r2, c2) = p[1];
        
        // Fill horizontally to connect rooms going from min col to max col
        for c in c1.min(c2)..=c1.max(c2) {
            grid[r1][c] = Tile::Floor;
        }

        // Fill vertically to connect rooms going from min row to max row
        for r in r1.min(r2)..=r1.max(r2) {
            grid[r][c2] = Tile::Floor;
        }
    }

    // Display for debugging
    for row in grid.iter() {
        for tile in row {
            print!("{tile:?}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dungeon_test() {
        generate_dungeon();
    }
}