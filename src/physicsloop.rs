// All code for the physics loop should go in here.
use super::WORLDSIZE;
use super::Player;

// simulate is called every frame before the frame is drawn.
// simulate is currently called 20 times per second, but this can be changed with the const in main.rs
pub fn simulate(world: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], keys_pressed: [bool; 6], player: &Player) 
{
    // Begin by syncing the world map to the game state. Anything drawn from this point should
    // only override ' ' characters.
    for i in 0..WORLDSIZE.1 {
        for j in 1..WORLDSIZE.0 {
            game_state[i][j] = world[i][j];
        }
    }

    // need to insert player at its position. Player position is middle bottom
    // if we're in initial animation state
    if player.animation == 0 {
        println!("animation is zero");
        //   o
        //  /|\
        //  / \
        game_state[player.pos.1][player.pos.0] = ' ';
        game_state[player.pos.1][player.pos.0 - 1] = '/';
        game_state[player.pos.1][player.pos.0 + 1] = '\\';
        game_state[player.pos.1 - 1][player.pos.0] = '|';
        game_state[player.pos.1 - 1][player.pos.0 - 1] = '/';
        game_state[player.pos.1 - 1][player.pos.0 + 1] = '\\';
        game_state[player.pos.1 - 2][player.pos.0] = 'o';
        game_state[player.pos.1 - 2][player.pos.0 - 1] = ' ';
        game_state[player.pos.1 - 2][player.pos.0 + 1] = ' ';
    }
    // if we're in the walking animation state
    else if player.animation == 1 {
        println!("animation is one");
        //   o
        //  /|\
        //   |
        game_state[player.pos.1][player.pos.0] = '|';
        game_state[player.pos.1][player.pos.0 - 1] = ' ';
        game_state[player.pos.1][player.pos.0 + 1] = ' ';
        game_state[player.pos.1 - 1][player.pos.0] = '|';
        game_state[player.pos.1 - 1][player.pos.0 - 1] = '/';
        game_state[player.pos.1 - 1][player.pos.0 + 1] = '\\';
        game_state[player.pos.1 - 2][player.pos.0] = 'o';
        game_state[player.pos.1 - 2][player.pos.0 - 1] = ' ';
        game_state[player.pos.1 - 2][player.pos.0 + 1] = ' ';
    }
    else {
        println!("Error setting animation state!");
    }
    
    
    //println!("Tick!")
}