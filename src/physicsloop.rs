// All code for the physics loop should go in here.
use super::WORLDSIZE;
use super::Player;

// simulate is called every frame before the frame is drawn.
// simulate is currently called 20 times per second, but this can be changed with the const in main.rs
pub fn simulate(world: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], keys_pressed: [bool; 6], player: &mut Player) 
{
    if keys_pressed[5] {
        jump(player);
    }

    player.increment_frame_counter();

    // Begin by syncing the world map to the game state. Anything drawn from this point should
    // only override ' ' characters.
    for i in 0..WORLDSIZE.1 {
        for j in 1..WORLDSIZE.0 {
            game_state[i][j] = world[i][j];
        }
    }

    // This has to go before any player rendering. Does vertical collision and movement via ancient dark magic.
    do_gravity(player, game_state);

    // if player is walking left or right
    if keys_pressed[0] || keys_pressed[1] {
        player.set_is_walking(true);
        player.animate();
    }
    else {
        player.set_is_walking(false);
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

const GRAV_PER_FRAME: i32 = 1;

fn do_gravity(player: &mut Player, game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1]) {
    // Apply gravity
    player.set_accel(player.accel.0, player.accel.1 + GRAV_PER_FRAME);

    // Ground collision
    // If falling (positive Y velocity)
    if player.accel.1 > 0
    {
        let mut snapto: i32 = player.pos.1 as i32 + player.accel.1;
        // player's base
        for i in 0..=2 {
            for j in 1..=player.accel.1 {
                let targety = if player.pos.1 + j as usize > WORLDSIZE.1 - 1 {WORLDSIZE.1 - 1} else {player.pos.1 + j as usize};
                match game_state[targety][i + player.pos.0 - 1] {
                    '#' | 'T' => {
                        snapto = targety as i32 - 1;
                        player.set_accel(player.accel.0, 0);
                    }
                    _ => {

                    }
                }
            }
        }
        player.set_pos(player.pos.0, snapto as usize);
    }
}

fn jump(player: &mut Player) {
    player.set_accel(0, -5);
}