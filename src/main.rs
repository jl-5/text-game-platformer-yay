use std::time::Instant;
use inputbot::KeybdKey::*;

mod worldgen; 
mod physicsloop; 
mod renderloop;

// this should be in the formmat x,y
pub const WORLDSIZE: (usize,usize) = (573,30);
// Duration of 1 frame
const FRAME_TIME: f64 = 0.05;
const ANIM_RATE: usize = 5;

// The Player Struct
pub struct Player {
    pos: (usize, usize),
    // Add more stuff here as makes sense
    // i.e. sprites, animationState, etc.

    // this is a 0 if in initial animation state and a 1 if in the walking state
    animation: usize,
    is_walking: bool,
    // this has no business being here lol
    frame_counter: usize,
    // this is so mf janky
    grav_counter: usize,
    accel: (i32, i32),
    has_won: bool,
}

impl Player {
    fn set_pos(&mut self, x: usize, y: usize) {
        self.pos = (x, y);
    }

    fn set_is_walking(&mut self, walk_bool: bool) {
        self.is_walking = walk_bool;
    }

    // this function should toggle the animation state if enough frames have passed
    fn animate(&mut self) {
        // if the regular game is still going
        if !self.has_won{
            if self.frame_counter >= ANIM_RATE {
                if self.animation == 0 {
                    self.animation = 1;
                }
                else if self.animation == 1 {
                    self.animation = 0;
                }
                self.frame_counter = 0;
            }
        }
        else {
            if self.animation == 2 {
                self.animation = 3;
            }
            else if self.animation == 3 {
                self.animation = 2;
            }
            self.frame_counter = 0;
        }

    }

    fn win(&mut self){
        self.has_won = true;
        self.animation = 2;
    }

    fn increment_frame_counter(&mut self){
        self.frame_counter += 1;
    }

    fn increment_grav_counter(&mut self){
        self.grav_counter += 1;
    }

    fn reset_grav_counter(&mut self){
        self.grav_counter = 0;
    }

    fn set_accel(&mut self, x: i32, y: i32) {
        self.accel = (x, y);
    }
}

pub struct Enemy {
    pos: (usize, usize),
    animation: usize,
    frame_counter: usize,
    // 0 for left, 1 for right
    dir: usize,
}

impl Enemy {
    fn set_pos(&mut self, x: usize, y: usize) {
        self.pos = (x, y);
    }

    // this function should toggle the animation state if enough frames have passed
    fn animate(&mut self) {
        // if the regular game is still going
        if self.frame_counter >= ANIM_RATE {
            if self.animation == 0 {
                self.animation = 1;
            }
            else if self.animation == 1 {
                self.animation = 0;
            }
            self.frame_counter = 0;
        }
    }

    fn increment_frame_counter(&mut self){
        self.frame_counter += 1;
    }

    fn flip_dir(&mut self) {
        if self.dir == 0 {
            self.dir = 1;
        }
        else if self.dir == 1 {
            self.dir = 0;
        }
        else {
            println!("you done messed up.");
        }
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
        let mut player = Player {
            pos: (10, 24),
            animation: 0,
            is_walking: false,
            frame_counter: 0,
            accel: (0, 0),
            grav_counter: 0,
            has_won: false,
        };

        // Enemy instatiations
        let enemy1: Enemy = Enemy {
            pos: (263, 22),
            animation: 0,
            frame_counter: 0,
            dir: 0
        };

        let enemy2: Enemy = Enemy {
            pos: (303, 10),
            animation: 0,
            frame_counter: 0,
            dir: 1
        };

        let mut enemies: [Enemy; 2] = [enemy1, enemy2];
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
            physicsloop::simulate(&mut world, &mut gamestate, &mut keys_pressed, &mut player, &mut enemies);
            // Sync Camera Pos to Player for now.
            let camera_pos = player.pos;
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

fn reset_map(player: &mut Player) {
    player.set_pos(10, 24);
    player.set_accel(0, 0);
}