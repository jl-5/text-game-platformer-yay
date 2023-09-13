// Called each frame after the physics loop.
// No values should be updated here, this is only used to redraw the screen.

use super::WORLDSIZE;

// how far around camera center the camera can view. Shouldn't probably change for this game.
// Bottom, Top, Left, Right
const CAMERA_RANGE: (usize,usize,usize,usize) = (15,14,20,60);

// the world dimensions are 500x30
pub fn draw(world: [[char; WORLDSIZE.0]; WORLDSIZE.1], camera_pos: (usize,usize)) {
    // Print a wall of empty text so old state cannot be seen. 
    let mut mutable_string = String::from("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    // Camera Axis bound protection.
    let range_bottom: usize = if ((camera_pos.1) as i32 - (CAMERA_RANGE.0 as i32)) < 0 { 0 } else { camera_pos.1 - CAMERA_RANGE.0 };
    let range_top: usize = if ((camera_pos.1) as i32 + (CAMERA_RANGE.1 as i32)) > (WORLDSIZE.1) as i32 { WORLDSIZE.1 } else { camera_pos.1 + CAMERA_RANGE.1 };
    let range_left: usize = if ((camera_pos.0) as i32 - (CAMERA_RANGE.2 as i32)) < 0 { 0 } else { camera_pos.0 - CAMERA_RANGE.2 };
    let range_right: usize = if ((camera_pos.0) as i32 + (CAMERA_RANGE.3 as i32)) > (WORLDSIZE.0) as i32 { WORLDSIZE.0 } else { camera_pos.0 + CAMERA_RANGE.3 };
    // Draw the world on this range.
    // Start by iterating through rows.
    for row in range_bottom..range_top {
        let current_row = world[row];
        // Then get board L/R range substrings.
        let (_, mid_right) = current_row.split_at(range_left);
        let (row, _) = mid_right.split_at(range_right - range_left);

        // print the row to the terminal
        for c in row {
            mutable_string.push(*c);
        }
        mutable_string.push_str("\n");
    }

    print!("{}", mutable_string);
}