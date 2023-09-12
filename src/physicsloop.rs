// All code for the physics loop should go in here.
use super::WORLDSIZE;

// simulate is called every frame before the frame is drawn.
// simulate is currently called 20 times per second, but this can be changed with the const in main.rs
pub fn simulate(world: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], keys_pressed: [bool; 6]) 
{
    for i in 0..WORLDSIZE.1 {
        for j in 1..WORLDSIZE.0 {
            game_state[i][j] = world[i][j];
        }
    }
    
    //println!("Tick!")
}