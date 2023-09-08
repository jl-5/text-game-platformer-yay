use std::time::Duration;
use inputbot::{KeybdKey, MouseButton, KeySequence, KeybdKey::*};

mod worldgen;

fn main() {
    /*
        Main Variables
     */
        // Holds the Raw terrain. Update on block break, for example.
        let mut terrain = [[{' '}; 500];30];
        // Holds the world this frame. Recalculated each frame.
        let mut world = [[{' '}; 500];30];

        // Camera pos
        let mut camera_pos: (usize,usize) = (50,15); 
    
        // Holds the millis time of the last frame.
        let mut last_frame = std::time::Instant::now();
        // Tracks currently-down keys
        let mut i: i32 = 0;
    
    /*
        WorldGen
     */ 
    gen_world(world);

    worldgen::printhi();
    
    // Main loop
    let mut exit_bool: bool = false;
    while !exit_bool {
        
        // Read Key Inputs
        i = read_key_inputs(i);
        // Simulate the world
        simulate();
        // Draw the world
        draw(world, camera_pos);

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

fn gen_world(world: [[char;500];30]) {

}

// Main loop for simulating a frame of the world update
fn simulate() {

}

fn draw(world: [[char;500];30], camera_pos: (usize,usize)) {

}