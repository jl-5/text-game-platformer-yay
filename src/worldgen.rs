use super::WORLDSIZE;

// The code used to generate the map.
pub fn gen_world(world: &mut [[char;WORLDSIZE.0];WORLDSIZE.1]) {
    // Generate Ground
    // this loop iterates from 0 to Worldsize.0 (500), setting one row of 


    // loop through each row of the world and draw each manually

    // rows will hold all the arrays of strings
    let mut rows: [String; 30] = Default::default();

    // rows[0] is the top of the level
    rows[0]  = format!("{}", "#############################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################");
    rows[1]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[2]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[3]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[4]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[5]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[6]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[7]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[8]  = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[9]  = format!("{}", "                                                                                                                                                                                                                                                                 M                                                                                                                                                                                                                                                                                                                           ");
    rows[10] = format!("{}", "                                                                                                                                                                                                                                                            ###########                                                                                                                                                                                                         ##################                                                                                           ");
    rows[11] = format!("{}", "                                                                                                                                                                                                                                                                                             ######################################                                                                                                                                                                                                                                                          ");
    rows[12] = format!("{}", "                                                                                                                                                                                                                                                                        #######                                                                                                                                                                                                                                                                                                              ");
    rows[13] = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                       ######################################################                                                                                                                ");
    rows[14] = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[15] = format!("{}", "                                                                                                                                                                                                                                                                               ##########                                                                                                       #########                                                                                                                                                                                    ");                                                  
    rows[16] = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[17] = format!("{}", "                                                                                                                                                                                                   ###################                                                                                                                                                                                                                                                                                                                                                                       ");
    rows[18] = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                             #############3                                                                                                                                                                  ");
    rows[19] = format!("{}", "                                                                                                                                                                                                                                     MMM        MM                                           ####################                                                                                                                                                                                                                                                                            ");
    rows[20] = format!("{}", "                                                                                                                                                                                                                                     ###################                                                                                                                                                                                                                                                                                           You w n!                                  ");
    rows[21] = format!("{}", "                                                                                                                                                                              ###############                                                                                                                                                                                               #############                                                                                                                                                                                    ");
    rows[22] = format!("{}", "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ");
    rows[23] = format!("{}", "                                                                                                                                 ####################                                                                                                           #######################                                                                                                                                                                                                                                                                                                      ");
    rows[24] = format!("{}", "                                                                                                                                                                                                     #####################                                                                                                                               MMM                                                                                                                                                                                                                                 ");
    rows[25] = format!("{}", "                                                                                                                                                                                                                                                                                                                                                  ####################################                                                                                                                                                                                                       ");
    rows[26] = format!("{}", "                                                                                                          MMMM                                                                                                                                                                                                                                                                                                                                                                                                                                                                               ");
    rows[27] = format!("{}", "#############################################################           ################################################                                     #################################                                                                                                                                                                                                                                                                                                                 ##############################################################################");
    rows[28] = format!("{}", "#############################################################           ################################################                                     #################################                                                                                                                                                                                                                                                                                                                 ##############################################################################");
    rows[29] = format!("{}", "#############################################################           ################################################                                     #################################                                                                                                                                                                                                                                                                                                                 ##############################################################################");
    // rows[29] is the bottom of the level

    // this loop should populate "world" with the level above by iterating over each row's string's characters
    for i in 0..(WORLDSIZE.1) {
        //println!("made it into row {}", i);
        for (j, block) in rows[i].chars().enumerate() {
            //println!("made it into block {}", j);
            world[i][j] = block;
        }
    }

    // "I can't believe it's not Flagpole! [TM]"
    
}