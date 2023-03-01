
use rayon::prelude::*;

use crate::trellis::trellis::Trellis;

use bitvec::prelude::*;

use serde::{Deserialize, Serialize};
//use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEvents {
    pub error_events: Vec<Vec<BitVec>>,
    pub error_event_lengths: Vec<Vec<u32>>
}

pub fn find_irreducible_error_event(
    _v: u16, numerators: [u16; 3], _denominator: u16, max_search_distance: usize, trellis: &Trellis
) -> ErrorEvents {
    
    let _path: String = "./Simulation_results/".to_owned();
    let k = numerators.len();

    // let mut num_string: String = "".to_owned();

    // for i in 0..k {
    //     num_string.push_str(&(numerators[i].to_string() + "_"));
    // }

    // let mut file_name: String = "Trellis_v_".to_owned();
    // file_name.push_str(&(v.to_string() + "_num_" + &num_string + "den_" + &denominator.to_string()));

    // if !Path::new(&(path + &file_name)).exists() {
    //     println!("no file {}!", file_name);
    //     return;
    // }

    // Need to either use multiple u64s or use a matrix algebra library to finish this now... Since termination
    // Solution: BigInt

    const MAX_ITERATION: usize = 200;
    let mut zero_state: Vec<Vec<Vec<BitVec>>> = vec![];
    let mut column: Vec<Vec<Vec<Vec<BitVec>>>> = vec![vec![]; 2];

    //let mut column: ColumnType = ColumnType.new();

    for i in 0..MAX_ITERATION {
        println!("Current trellis depth: {}", i);

        let index: usize = ((i + 1) % 2) as usize;
        let prev_index: usize = (i % 2) as usize;
        column[index] = vec![vec![]; trellis.num_states];

        if i == 0 {
            for input in 1..trellis.num_input_symbols {
                let next_state = u16::from(*trellis.next_states.index(0, input)) as usize;
                if column[index][next_state].is_empty() {
                    column[index][next_state] = vec![vec![]; max_search_distance];
                }
                let weight = trellis.outputs.index(0, input).count_ones();

                let mut input_bits: BitVec = bitvec![];
                for n in (0..k).rev() {
                    let bit = input >> n & 1;
                    input_bits.push(bit != 0); // I'm just converting the nth input bit to a bool before pushing it to the BitVec
                }
                column[index][next_state as usize][(weight - 1) as usize].push(input_bits);
            }
        } else {
            for current_state in 1..trellis.num_states {
                if !column[prev_index][current_state].is_empty() {
                    for distance in 0..max_search_distance {
                        if !column[prev_index][current_state][distance].is_empty() {
                            for input in 0..trellis.num_input_symbols {

                                let next_state = usize::from(*trellis.next_states.index(current_state, input)); // safe
                                if column[index][next_state].is_empty() {
                                    column[index][next_state] = vec![vec![]; max_search_distance];
                                }

                                let weight = trellis.outputs.index(current_state, input).count_ones() as usize; // safe

                                // Append input bits to each element in tmp
                                
                                if distance + weight < max_search_distance {
                                    let mut tmp = column[prev_index][current_state][distance].clone();
                                    for m in 0..tmp.len() {
                                        for n in (0..k).rev() {
                                            let bit = input >> n & 1;
                                            tmp[m].push(bit != 0); // I'm just converting the nth input bit to a bool before pushing it to the BitVec
                                        }
                                    }
                                    column[index][next_state][distance + weight].par_extend(tmp);
                                }
                            }
                        }
                    }
                }
            }
        }
        zero_state.push(column[index][0].clone());
    }

    //println!("{:?}", zero_state[14]);

    let _length = zero_state.len();

    let mut error_events: Vec<Vec<BitVec>> = vec![vec![]; max_search_distance];
    let mut error_event_lengths: Vec<Vec<u32>> = vec![vec![]; max_search_distance];
    
    for i in 0..MAX_ITERATION {
        if !zero_state[i].is_empty() {
            for distance in 0..max_search_distance {
                if !zero_state[i][distance].is_empty() {
                    let mut new_len = 0;
                    if error_events[distance].is_empty() {
                        error_events[distance] = zero_state[i][distance].clone();
                    } else {
                        // pad with zeros until uniform length

                        for n in 0..error_events[distance].len() {
                            let length = error_events[distance][n].len();
                            error_events[distance][n].extend(bitvec![0; (k * (i + 1) - length)]);
                        }
                        error_events[distance].append(&mut zero_state[i][distance].clone());
                    }
                    let new_len = zero_state[i][distance].len();
                    println!("new_len {}", new_len);
                    for n in 0..new_len {
                        error_event_lengths[distance].push(zero_state[i][distance][n].len() as u32);
                    }
                }
            }
        }
    }

    ErrorEvents {
        error_events,
        error_event_lengths
    }
}
