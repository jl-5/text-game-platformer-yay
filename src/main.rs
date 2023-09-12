use std::time::Duration;
use std::time::Instant;
use inputbot::{KeybdKey, MouseButton, KeySequence, KeybdKey::*};

mod worldgen; 
mod physicsloop; 
mod renderloop;

pub const WORLDSIZE: (usize,usize) = (30,10);
// Duration of 1 frame
const FRAME_TIME: f64 = 0.05;

// 80x24 terminal size
fn main() {
    /*
        Main Variables
     */
        // Holds the Raw terrain. Update on block break, for example.
        let mut terrain: [[char; WORLDSIZE.0]; WORLDSIZE.1] = [[{' '}; WORLDSIZE.0];WORLDSIZE.1];
        // Holds the world this frame. Recalculated each frame.
        let mut world: [[char; WORLDSIZE.0]; WORLDSIZE.1] = [[{'#'}; WORLDSIZE.0];WORLDSIZE.1];

        // Camera pos
        let mut camera_pos: (usize,usize) = (50,15); 
    
        // Holds the millis time of the last frame.
        let mut last_frame = std::time::Instant::now();
        // Tracks currently-down keys
        let mut i: i32 = 0;
    
    /*
        WorldGen
     */ 
    worldgen::gen_world(&mut world);
    
    // Main loop
    let mut exit_bool: bool = false;
    while !exit_bool {
        
        // Read Key Inputs
        i = read_key_inputs(i);

        // Run these once every FRAME_TIME (seconds)
        if last_frame.elapsed().as_secs_f64() >= FRAME_TIME {
            last_frame = Instant::now();
            // Simulate the world
            physicsloop::simulate();
            // Draw the world
            renderloop::draw(world, camera_pos);
        }

// End timer

        // End program
    }
}

// Interpreter for key inputs by int id.
fn read_key_inputs(mut i: i32) -> i32{
    if LeftKey.is_pressed() {
        i += 1;
        println!("Left Frames: {}", i);
    }
    return i;
}