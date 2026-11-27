use rand_distr::{Distribution, Normal};

const DUNGEON_HEIGHT: usize = 20;
const DUNGEON_WIDTH: usize = 40;

const MAX_ROOM_AREA: usize = 25;
const MIN_ROOM_AREA: usize = 4;

pub fn generate_dungeon() -> [[&'static str; DUNGEON_WIDTH]; DUNGEON_HEIGHT] {
    let mut grid = [["0"; DUNGEON_WIDTH]; DUNGEON_HEIGHT];

    make_rooms(&mut grid);

    grid
}

// Modifies the grid passed in to have rooms
// First choose a random room length & random room height
// Next, choose a random coordinate for the top-left corner of the room
// the coord should be at least 1 tile away from the top edge & the left edge
// and > the room height from the bottom edge and > the room width from the right edge

// TODO: Completely replace this guess and check algorithm with something way more efficient
fn make_rooms(grid: &mut [[&str; DUNGEON_WIDTH]; DUNGEON_HEIGHT]) {
    let mut failed_attempts = 0;
    'outer: loop {
        // Determine room dimensions
        let mean = ((MIN_ROOM_AREA + MAX_ROOM_AREA) as f64 / 2.0).sqrt();
        // Controls how much variation there is in room dimensions.
        let distribution = Normal::new(mean, 0.75).unwrap();

        let mut rng = rand::rng();

        let min_side = (MIN_ROOM_AREA as f64).sqrt().ceil();
        let max_side = (MAX_ROOM_AREA as f64).sqrt().floor();

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
        let row = rand::random_range(1..(grid.len() - room_height));
        let col = rand::random_range(1..(grid[0].len() - room_width));

        // Check if room would intersect with other rooms

        // Very simple algorithm that just checks each coord in room and makes sure it doesn't intersect other rooms
        for r in (row-1)..(row + room_height+1) {
            for c in (col-1)..(col + room_width+1) {
                if grid[r][c] == "." { // contacting other room
                    // Skip
                    failed_attempts += 1;
                    println!("failed attempts: {failed_attempts}");

                    if failed_attempts > 100 {
                        break 'outer
                    }
                    continue 'outer
                }
            }
        }

        // Create room
        for r in row..(row + room_height) {
            for c in col..(col + room_width) {
                grid[r][c] = ".";
            }
        }

        // Reset failed attempts bc just got a success
        failed_attempts = 0;
    }
    // Display for debugging
    for row in grid.iter() {
        for tile in row {
            print!("{tile}");
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