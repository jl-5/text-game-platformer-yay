use super::WORLDSIZE;

pub fn gen_world(world: &mut [[char;WORLDSIZE.0];WORLDSIZE.1]) {
    for i in 0..(WORLDSIZE.0) {
        world[WORLDSIZE.1 - 1][i] = 'T';
    }
}