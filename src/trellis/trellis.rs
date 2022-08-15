use crate::Vec2d::Vec2d;
use ::gf256::p16;

pub struct Trellis {
    pub numInputSymbols: usize,
    pub numOutputSymbols: usize,
    pub numStates: usize,
    pub nextStates: Vec2d<p16>,
    pub outputs:  Vec2d<p16>,
    pub terminations: Vec<Vec<u16>>,
}