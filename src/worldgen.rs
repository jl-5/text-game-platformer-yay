use super::WORLDSIZE;
use rand::Rng;

// The code used to generate the map.
pub fn gen_world(world: &mut [[char;WORLDSIZE.0];WORLDSIZE.1]) {
    // Generate Ground
    for i in 0..(WORLDSIZE.0) {
        world[WORLDSIZE.1 - 1][i] = 'T';
    }

    // "I can't believe it's not Flagpole! [TM]"
    
}