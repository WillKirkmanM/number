use crate::drawing::GRID_SIZE;

// Finds the bounding box of the drawn number
fn find_bounding_box(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> Option<(usize, usize, usize, usize)> {
    let mut min_x = GRID_SIZE;
    let mut max_x = 0;
    let mut min_y = GRID_SIZE;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if grid[y][x] > 0 {
                found = true;
                if x < min_x { min_x = x; }
                if x > max_x { max_x = x; }
                if y < min_y { min_y = y; }
                if y > max_y { max_y = y; }
            }
        }
    }
    
    if found { Some((min_x, min_y, max_x, max_y)) } else { None }
}

// Centers the drawing within the grid
pub fn center_drawing(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> [[u8; GRID_SIZE]; GRID_SIZE] {
    let mut new_grid = [[0; GRID_SIZE]; GRID_SIZE];
    
    if let Some((min_x, min_y, max_x, max_y)) = find_bounding_box(grid) {
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        let offset_x = (GRID_SIZE - width) / 2;
        let offset_y = (GRID_SIZE - height) / 2;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if grid[y][x] > 0 {
                    new_grid[offset_y + (y - min_y)][offset_x + (x - min_x)] = 255;
                }
            }
        }
    }
    
    new_grid
}