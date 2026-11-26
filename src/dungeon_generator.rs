const DUNGEON_HEIGHT: usize = 20;
const DUNGEON_WIDTH: usize = 60;

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
fn make_rooms(grid: &mut [[&str; DUNGEON_WIDTH]; DUNGEON_HEIGHT]) {
    let room_width = rand::random_range(1..=MAX_ROOM_AREA);

    let min_height = MIN_ROOM_AREA.div_ceil(room_width);
    let max_height = MAX_ROOM_AREA / room_width;

    let room_height = rand::random_range(min_height..=max_height);

    // Get random coords for top-left corner of room
    let row = rand::random_range(1..(grid.len() - room_height));
    let col = rand::random_range(1..(grid[0].len() - room_width));

    // Create room
    for r in row..(row + room_height) {
        for c in col..(col + room_width) {
            grid[r][c] = ".";
        }
    }

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
        let grid = generate_dungeon();

        let floor_tiles = grid
            .iter()
            .flatten()
            .filter(|&&tile| tile == ".")
            .count();

        assert!(
            (MIN_ROOM_AREA..=MAX_ROOM_AREA).contains(&floor_tiles)
        );
    }
}