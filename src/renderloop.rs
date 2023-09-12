// Called each frame after the physics loop.
// No values should be updated here, this is only used to redraw the screen.

use super::WORLDSIZE;

// the world dimensions are 500x30
pub fn draw(world: [[char; WORLDSIZE.0]; WORLDSIZE.1], camera_pos: (usize,usize)) {
    // Print a wall of empty text so old state cannot be seen.
    //println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    
    // need to find a way to "draw" the world, i.e. print out the world

    // this will iterate from 0-29 inclusive; each row
    for row in 0..=WORLDSIZE.1-1 {
        let current_row = world[row];
        let row_string: String = current_row.iter().collect();
        // print the row to the terminal
        print!("\n{}", row_string);
    }

    // println!("FINISHED DRAWING WORLD!");
}