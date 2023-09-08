use std::time::Duration;
use inputbot::{KeybdKey, MouseButton, KeySequence, KeybdKey::*};

fn main() {
    // WorldGen
    
    // Holds the millis time of the last frame.
    let mut last_frame = std::time::Instant::now();
    // Tracks currently-down keys
    let mut i: i32 = 0;
    
    // Main loop
    let mut exit_bool: bool = false;
    while !exit_bool {
        
        // Read Key Inputs
        i = read_key_inputs(i);
        // Simulate the world
        simulate();
        // Draw the world
        draw();

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

// Main loop for simulating a frame of the world update
fn simulate() {

}

fn draw() {

}