// This code was written by user @dqc on StackOverflow in the following StackOverflow answer:
//          https://stackoverflow.com/questions/13102786/two-dimensional-vectors-in-rust
//
// It provides a very nice 2d vector type which will be used throughout this project

use std::fmt;
use ::gf256::p16;

#[derive(Debug)]
pub struct Vec2d<T> {
    pub vec: Vec<T>,
    pub row: usize,
    pub col: usize,
}

pub fn Fill(fill: usize, len: usize) -> Vec<u16> {
    let vec: Vec<u16> = (0..len).map(|_| { fill as u16 }).collect();
    vec
}

pub fn Fill_p16(fill: usize, len: usize) -> Vec<p16> {
    let vec: Vec<p16> = (0..len).map(|_| { p16(fill as u16) }).collect();
    vec
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
        assert!(vec.len() == row * col);
        Self { vec, row, col }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.col * row;
        &self.vec[i..(i + self.col)]
    }

    pub fn index(&self, row: usize, col: usize) -> &T {
        let i = self.col * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, row: usize, col: usize) -> &mut T {
        let i = self.col * row;
        &mut self.vec[i + col]
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.row {
            if i != 0 {
                str.push_str(", ");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
        }
        write!(f, "[{}]", str)
    }
}



// fn main() {
//     let mut mv = Vec2d::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
//     *mv.index_mut(1, 2) = 10;
//     println!("Display: {}", mv);
//     println!("Debug: {:?}", mv);
// }