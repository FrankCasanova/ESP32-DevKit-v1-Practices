use crate::music::*;

pub const TEMPO: u16 = 150;

pub const MELODY: [(f64, i16); 261] = [
    (REST, 2),//1
    (REST, 2),//2
    (REST, 2),//3
    (REST, 4),//4
    (NOTE_G7, 8),
    (NOTE_G7, 8),
    (NOTE_G7, 4),//
    (NOTE_E7, 4), 
    (NOTE_C4, 3), //B
    (NOTE_G4, 8),
    (NOTE_G7, 8),//
    (NOTE_G7, 8),
    (NOTE_G7, 8),
    (NOTE_F7, 8),
    (NOTE_E7, 4),//
    (NOTE_F7, 8),
    (NOTE_G7, 8),
    (NOTE_G7, 4),//
    (NOTE_E7, 4),
    (NOTE_C4, 2), //B
    (NOTE_E4, 4),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_E7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_E7, 8),
    (NOTE_D7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_D7, 2),
    (NOTE_C6, 2),
    (NOTE_B6, 2),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_B7, 4),
    (NOTE_C6, 4),
    (NOTE_B6, 4),
    (NOTE_A6, 4),
    (NOTE_G6, 4),//
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_E7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_E7, 8),
    (NOTE_D7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_D7, 2),
    (NOTE_C6, 2),
    (NOTE_B6, 2),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_B7, 4),
    (NOTE_C6, 4),
    (NOTE_B6, 4),
    (NOTE_A6, 4),
    (NOTE_G6, 4),//
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_E7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_E7, 8),
    (NOTE_D7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_D7, 2),
    (NOTE_C6, 2),
    (NOTE_B6, 2),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_B7, 4),
    (NOTE_C5, 4),
    (NOTE_B5, 4),
    (NOTE_A5, 4),
    (NOTE_G5, 4),//
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_E7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_E7, 8),
    (NOTE_D7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_D7, 2),
    (NOTE_C5, 2),
    (NOTE_B6, 2),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_B7, 4),
    (NOTE_C6, 4),
    (NOTE_B6, 4),
    (NOTE_A6, 4),
    (NOTE_G6, 4),//
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_E7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_E7, 8),
    (NOTE_D7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_D7, 2),
    (NOTE_C6, 2),
    (NOTE_B5, 2),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_B7, 4),
    (NOTE_C5, 4),
    (NOTE_B5, 4),
    (NOTE_A5, 4),
    (NOTE_G5, 4),//
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_F7, 4),
    (NOTE_G7, 4),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_F7, 8),
    (NOTE_E7, 4),
    (NOTE_G6, 4),
    (NOTE_F6, 8),
    (NOTE_E6, 8),
    (NOTE_D6, 8),
    (NOTE_E6, 8),
    (NOTE_F6, 4),
    (NOTE_D6, 2),
    (NOTE_C5, 2),
    (NOTE_B5, 2),
    (NOTE_E7, 8),
    (NOTE_E7, 8),
    (NOTE_E7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_E7, 4),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_D7, 8),
    (NOTE_C7, 4),
    (NOTE_B7, 4),
    (NOTE_C6, 4),
    (NOTE_B6, 4),
    (NOTE_A6, 4),
    (NOTE_G6, 4),//
    
    
    
];
