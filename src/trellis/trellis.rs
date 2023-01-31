use crate::vec2d::Vec2d;
use ::gf256::p16;

pub struct Trellis {
    pub num_input_symbols: usize,
    pub num_output_symbols: usize,
    pub num_states: usize,
    pub next_states: Vec2d<p16>,
    pub outputs:  Vec2d<p16>,
    pub terminations: Vec<Vec<u16>>,
}