use std::time::Instant;
use inputbot::KeybdKey::*;

mod worldgen; 
mod physicsloop; 
mod renderloop;

pub const WORLDSIZE: (usize,usize) = (500,30);
// Duration of 1 frame
const FRAME_TIME: f64 = 0.1;

// The Player Struct
pub struct Player {
    pos: (usize, usize),
    // Add more stuff here as makes sense
    // i.e. sprites, animationState, etc.

    // this is a 0 if in initial animation state and a 1 if in the walking state
    animation: (usize),
    is_walking: (bool),
}

impl Player {
    fn set_pos(&mut self, x: usize, y: usize) {
        // ^^^ Here
        self.pos = (x, y);
    }
}

// 80x24 terminal size
fn main() {
    /*
        Main Variables
     */
        // Holds the final charmap to be drawn, including player and entities
        // WORLD TOP IS 0, WORLD BOTTOM IS WORLDSIZE.1.
        let mut gamestate: [[char; WORLDSIZE.0]; WORLDSIZE.1] = [[{' '}; WORLDSIZE.0];WORLDSIZE.1];
        // Holds the world, excluding the player and entities
        let mut world: [[char; WORLDSIZE.0]; WORLDSIZE.1] = [[{' '}; WORLDSIZE.0];WORLDSIZE.1];

        // Camera pos
        let mut camera_pos: (usize,usize) = (50,28); 
    
        // Holds the millis time of the last frame.
        let mut last_frame = std::time::Instant::now();

        // Keypress storage. True when key is pressed and input not yet consumed.
        /* 
        0: LEFT
        1: RIGHT
        2: UP
        3: DOWN
        4: ESC
        5: Z
         */
        let mut keys_pressed: [bool; 6] = [false; 6];

        // Player Instantiation
        let player = Player {
            pos: (10, 28),
            animation: 0,
            is_walking: false,
        };
    /*
        WorldGen
     */ 
        worldgen::gen_world(&mut world);

    
    // Main loop
    let mut exit_bool: bool = false;
    while !exit_bool {
        
        // Read Key Inputs
        keys_pressed = read_key_inputs(keys_pressed);

        // Run these once every FRAME_TIME (seconds)
        if last_frame.elapsed().as_secs_f64() >= FRAME_TIME {
            last_frame = Instant::now();
            // Simulate the world
            physicsloop::simulate(&mut world, &mut gamestate, keys_pressed, &player);
            // Sync Camera Pos to Player for now.
            camera_pos = player.pos;
            // Draw the world (from gamestate)
            renderloop::draw(gamestate, camera_pos);
        }

        if keys_pressed[4] {
            exit_bool = true;
        }

        // End program
        if exit_bool {
            break;
        }
    }
}

// Interpreter for key inputs by int id.
fn read_key_inputs(mut keys: [bool; 6]) -> [bool; 6]{
    if LeftKey.is_pressed() {
        keys[0] = true;
    }
    if RightKey.is_pressed() {
        keys[1] = true;
    }
    if UpKey.is_pressed() {
        keys[2] = true;
    }
    if DownKey.is_pressed() {
        keys[3] = true;
    }
    if EscapeKey.is_pressed() {
        keys[4] = true;
    }
    if ZKey.is_pressed() {
        keys[5] = true;
    }
    return keys;
}