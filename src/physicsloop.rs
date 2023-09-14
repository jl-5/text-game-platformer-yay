// All code for the physics loop should go in here.
use super::WORLDSIZE;
use super::Player;
use super::Enemy;

pub const JUMP_STRENGTH: i32 = 4;
const GRAV_EVERY_BLANK_FRAMES: usize = 3;
pub const WIN_LOCATION: (usize, usize) = (536,20);

// simulate is called every frame before the frame is drawn.
// simulate is currently called 20 times per second, but this can be changed with the const in main.rs
pub fn simulate(world: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1], keys_pressed: &mut [bool; 6], player: &mut Player, enemies: &mut [Enemy; 2]) 
{

    if keys_pressed[2] {
        println!("jumping!");
        jump(player,game_state);
        keys_pressed[2] = false;
    }

    // Set walk accel
    if keys_pressed[0] {
        set_walk(player, -1);
    }
    if keys_pressed[1] {
        set_walk(player, 1);
    }

    player.increment_frame_counter();
    player.increment_grav_counter();
    
    // Begin by syncing the world map to the game state. Anything drawn from this point should
    // only override ' ' characters.
    for i in 0..WORLDSIZE.1 {
        for j in 1..WORLDSIZE.0 {
            game_state[i][j] = world[i][j];
        }
    }

    // This has to go before any player rendering. Does vertical collision and movement via ancient dark magic.
    if player.grav_counter >= GRAV_EVERY_BLANK_FRAMES {
        do_gravity(player, game_state);
        player.reset_grav_counter();
    }
    
    do_walk(player, game_state);

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
    else if player.animation == 2 {
        //   
        //  /|\
        //  / \
        game_state[player.pos.1][player.pos.0] = ' ';
        game_state[player.pos.1][player.pos.0 - 1] = '/';
        game_state[player.pos.1][player.pos.0 + 1] = '\\';
        game_state[player.pos.1 - 1][player.pos.0] = '|';
        game_state[player.pos.1 - 1][player.pos.0 - 1] = '/';
        game_state[player.pos.1 - 1][player.pos.0 + 1] = '\\';
        game_state[player.pos.1 - 2][player.pos.0] = ' ';
        game_state[player.pos.1 - 2][player.pos.0 - 1] = ' ';
        game_state[player.pos.1 - 2][player.pos.0 + 1] = ' ';
    }
    // if we're in the walking animation state
    else if player.animation == 3 {
        //   
        //  /|\
        //   |
        game_state[player.pos.1][player.pos.0] = '|';
        game_state[player.pos.1][player.pos.0 - 1] = ' ';
        game_state[player.pos.1][player.pos.0 + 1] = ' ';
        game_state[player.pos.1 - 1][player.pos.0] = '|';
        game_state[player.pos.1 - 1][player.pos.0 - 1] = '/';
        game_state[player.pos.1 - 1][player.pos.0 + 1] = '\\';
        game_state[player.pos.1 - 2][player.pos.0] = ' ';
        game_state[player.pos.1 - 2][player.pos.0 - 1] = ' ';
        game_state[player.pos.1 - 2][player.pos.0 + 1] = ' ';
    }
    else {
        println!("Error setting player animation state!");
    }


    // enemy animation  and movement time!

    for i in 0..=enemies.len() - 1 {
        enemies[i].increment_frame_counter();
        enemies[i].animate();
            if enemies[i].animation == 0 {
                //  v
                // >#< 
                // / \
                game_state[enemies[i].pos.1][enemies[i].pos.0] = ' ';
                game_state[enemies[i].pos.1][enemies[i].pos.0 - 1] = '/';
                game_state[enemies[i].pos.1][enemies[i].pos.0 + 1] = '\\';
                game_state[enemies[i].pos.1 - 1][enemies[i].pos.0] = '#';
                game_state[enemies[i].pos.1 - 1][enemies[i].pos.0 - 1] = '>';
                game_state[enemies[i].pos.1 - 1][enemies[i].pos.0 + 1] = '<';
                game_state[enemies[i].pos.1 - 2][enemies[i].pos.0] = 'v';
                game_state[enemies[i].pos.1 - 2][enemies[i].pos.0 - 1] = ' ';
                game_state[enemies[i].pos.1 - 2][enemies[i].pos.0 + 1] = ' ';
            }
            else if enemies[i].animation == 1 {
                //  v
                // >#<
                //  |
                game_state[enemies[i].pos.1][enemies[i].pos.0] = '|';
                game_state[enemies[i].pos.1][enemies[i].pos.0 - 1] = ' ';
                game_state[enemies[i].pos.1][enemies[i].pos.0 + 1] = ' ';
                game_state[enemies[i].pos.1 - 1][enemies[i].pos.0] = '#';
                game_state[enemies[i].pos.1 - 1][enemies[i].pos.0 - 1] = '>';
                game_state[enemies[i].pos.1 - 1][enemies[i].pos.0 + 1] = '<';
                game_state[enemies[i].pos.1 - 2][enemies[i].pos.0] = 'v';
                game_state[enemies[i].pos.1 - 2][enemies[i].pos.0 - 1] = ' ';
                game_state[enemies[i].pos.1 - 2][enemies[i].pos.0 + 1] = ' ';
            }

                // Check that the player is on the ground before letting them jump.
                // if the enemy is facing left
                if enemies[i].dir == 0 {
                    // if there is no left space for it to walk on
                    if game_state[enemies[i].pos.1 + 1][enemies[i].pos.0 - 1] == ' ' {
                        enemies[i].flip_dir();
                    }
                    // otherwise, move it left
                    else {
                        enemies[i].set_pos(enemies[i].pos.0 - 1, enemies[i].pos.1);
                    }
                    
                }
                else if enemies[i].dir == 1 {
                    // if there is no right space for it to walk on
                    if game_state[enemies[i].pos.1 + 1][enemies[i].pos.0 + 1] == ' ' {
                        enemies[i].flip_dir();
                    }
                    // otherwise, move it right
                    else {
                        enemies[i].set_pos(enemies[i].pos.0 + 1, enemies[i].pos.1);
                    }
                }                
            }
    
    // Reset at end 
    keys_pressed[0] = false;
    keys_pressed[1] = false;
    //println!("Tick!")

    // if the player is in the win location (where the head is in the "o" spot)
    if player.pos.0 == WIN_LOCATION.0 && player.pos.1 <= WIN_LOCATION.1 + 2{
        player.win();
    }

    if player.has_won {
        game_state[WIN_LOCATION.1][WIN_LOCATION.0] = 'o';
    }
}

const GRAV_PER_FRAME: i32 = 1;

fn do_gravity(player: &mut Player, game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1]) {
        // Apply gravity
    player.set_accel(player.accel.0, player.accel.1 + GRAV_PER_FRAME);
    let mut killed = false;

    // Ground collision
    // If falling (positive Y velocity)
    if player.accel.1 > 0
    {
        let mut snapto: i32 = player.pos.1 as i32 + player.accel.1;
        let stored_accel = player.accel.1;
        // player's base
        'KILLPOINT: for j in 1..=stored_accel {
            for i in 0..=2 {
                let targety = if player.pos.1 + j as usize > WORLDSIZE.1 - 1 {WORLDSIZE.1 - 1} else {player.pos.1 + j as usize};
                if targety >= WORLDSIZE.1 - 1 {
                    kill(player);
                    killed = true;
                    break 'KILLPOINT;
                }
                else {
                    match game_state[targety][i + player.pos.0 - 1] {
                        '#' | 'T' => {
                            if targety as i32 - 1 < snapto {
                                snapto = targety as i32 - 1;
                            }
                            player.set_accel(player.accel.0, 0);
                            break 'KILLPOINT;
                        }
                        _ => {

                        }
                    }         
                }
            }
        }
        if !killed {player.set_pos(player.pos.0, snapto as usize);}
    }
    // Ascend (Negative Y Velocity)
    else if player.accel.1 < 0 {
        let mut snapto: i32 = player.pos.1 as i32 + player.accel.1;
        let stored_accel = player.accel.1;
        // player's base
        for j in 1..=-stored_accel {
            for i in 0..=2 {
                let targety = if player.pos.1 as i32 - j - 2 < 0 {0} else {player.pos.1 - 2 - j as usize};
                match game_state[targety][i + player.pos.0 - 1] {
                    '#' | 'T' => {
                        if targety as i32 + 3 > snapto {
                            snapto = targety as i32 + 3;
                        }
                        player.set_accel(player.accel.0, 0);
                    }
                    _ => {}
                }
            }
        }
        player.set_pos(player.pos.0, snapto as usize);
    }
    
}


fn do_walk(player: &mut Player, game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1]) {
    if player.accel.0 != 0 {
        // Collision Code here.
        let mut collided = false;
        let mut snapto = player.pos.0 as i32 + player.accel.0;

        // World End Collision
        if snapto - 2 < 0 {
            snapto = 2;
            player.accel.0 = 0
        }
        if snapto + 2 > WORLDSIZE.0 as i32 {
            snapto = WORLDSIZE.0 as i32 - 2;
            player.accel.0 = 0;
        }

        // Vertical first
        for i in 0..=2 {
            // Then horizontal
            if player.accel.0 > 0 {
                for j in 2..=player.accel.0 + 1 {
                    match game_state[player.pos.1 - i][j as usize + player.pos.0] {
                        '#' | 'T' => {
                            if j + player.pos.0 as i32 - 2 < snapto {
                                snapto = j + player.pos.0 as i32 - 2;
                            }
                            player.set_accel(0, player.accel.1);
                            collided = true;
                        }
                        _ => {}
                    }
                }
            }
            else if player.accel.0 < 0{
                for j in 2..=-(player.accel.0 - 1) {
                    match game_state[player.pos.1 - i][player.pos.0 - j as usize ] {
                        '#' | 'T' => {
                            if -j + player.pos.0 as i32 + 2 > snapto {
                                snapto = -j + player.pos.0 as i32 + 2;
                            }
                            player.set_accel(0, player.accel.1);
                            collided = true;
                        }
                        _ => {}
                    }
                }
            }
        }
        player.set_pos(snapto as usize, player.pos.1);
        
        // Slow the player down if they didn't collide this frame.
        if !collided{
            // Right
            if player.accel.0 > 0 {
                player.set_accel(player.accel.0 - 1, player.accel.1);
            }
            // left
            else if player.accel.0 < 0 {
                player.set_accel(player.accel.0 + 1, player.accel.1);
            }
        }
    }
}

fn set_walk(player: &mut Player, value: i32) {
    player.set_accel(value, player.accel.1);
}

fn jump(player: &mut Player, game_state: &mut [[char;WORLDSIZE.0];WORLDSIZE.1]) -> bool {
    let mut on_ground: bool = false;

    // Check that the player is on the ground before letting them jump.
    'LOOP: for i in 0..=2 {
        match game_state[player.pos.1 + 1][player.pos.0 + i - 1] {
            '#' | 'T' => {
                on_ground = true;
                break 'LOOP;
            }
            _ => {}
        }
    }
    // If On Ground, Jump.
    if on_ground {
        player.set_accel(0, -JUMP_STRENGTH);
    }
    // Returns true if the player successfully jumped
    return on_ground;
}

fn kill(player: &mut Player) {
    super::reset_map(player);
}