
use crate::trellis::trellis::Trellis;
//use crate::trellis::poly2trellis::poly2trellis_bruteforce;

use crate::Vec2d::{
    Vec2d,
    Fill
};

pub fn find_dominant_error_event_fast(constraint_length: Vec<u16>, code_generator: Vec2d<u16>, fd_tilde: i8) {
    
    // let trellis: Trellis = poly2trellis_bruteforce(constraint_length, code_generator);

    // const MaxIteraction: u8 = 100;

    // let mut ZeroState: Vec2d<u16> = Vec2d::new(Fill(0, 8), 8, 1);

    // println!("States: {}", trellis.nextStates);
    // println!("# input symbols: {}\n# output symbols: {}\n# states: {}\n", trellis.numInputSymbols, trellis.numOutputSymbols, trellis.numStates);
}

