
use num_bigint::BigUint;

use rayon::prelude::*;

use crate::trellis::trellis::Trellis;

use serde::{Deserialize, Serialize};
//use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEvents {
    pub error_events: Vec<Vec<BigUint>>,
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
    let mut zero_state: Vec<Vec<Vec<BigUint>>> = vec![];
    let mut column: Vec<Vec<Vec<Vec<BigUint>>>> = vec![vec![]; 2];

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
                // if input == 2 {
                //     println!("{} {}", trellis.outputs.index(0, input), weight);
                // }
                column[index][next_state as usize][(weight - 1) as usize].push(BigUint::from(input));
            }
        } else {
            for current_state in 1..trellis.num_states {
                if !column[prev_index][current_state].is_empty() {
                    for distance in 0..max_search_distance {
                        if !column[prev_index][current_state][distance].is_empty() {
                            for input in 0..trellis.num_input_symbols {

                                let next_state = usize::from(*trellis.next_states.index(current_state, input));
                                if column[index][next_state].is_empty() {
                                    column[index][next_state] = vec![vec![]; max_search_distance];
                                }

                                let weight = trellis.outputs.index(current_state, input).count_ones() as usize;

                                // Append input bits to each element in tmp
                                
                                if distance + weight < max_search_distance {
                                    let tmp = column[prev_index][current_state][distance].clone();
                                    let new_path: Vec<BigUint> = tmp.par_iter().map(
                                        |x| (x << k) | BigUint::from(input)
                                    ).collect();
                                    // if distance + weight == 2 && i == 2 && next_state == 0 {
                                    //     println!("{}", ((2 << k) | input as u64));
                                    //     println!("{} {} {:?} {:?}", weight, input, tmp, new_path);
                                    // }
                                    column[index][next_state][distance + weight].par_extend(new_path);
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

    let mut error_events: Vec<Vec<BigUint>> = vec![vec![]; max_search_distance];
    let mut error_event_lengths: Vec<Vec<u32>> = vec![vec![]; max_search_distance];
    
    for i in 0..MAX_ITERATION {
        if !zero_state[i].is_empty() {
            for distance in 0..max_search_distance {
                if !zero_state[i][distance].is_empty() {
                    if error_events[distance].is_empty() {
                        error_events[distance] = zero_state[i][distance].clone();
                    } else {
                        let length = get_length(&error_events[distance][0]);
                        // pad with zeros until uniform length
                        error_events[distance] = error_events[distance].par_iter().map(
                            |x| x << (k * (i + 1) - length)
                        ).collect();
                        error_events[distance].append(&mut zero_state[i][distance].clone());
                    }
                    //error_event_lengths[distance].append(&mut vec![k as u32 * i as u32; zero_state[i][distance].len()]);
                    error_event_lengths[distance].append(&mut vec![k as u32 * i as u32; get_length(&error_events[distance][0])]);
                }
            }
        }
    }

    ErrorEvents {
        error_events,
        error_event_lengths
    }
}

fn get_length(num: &BigUint) -> usize {
    let bits;
    let bytes = num.to_bytes_be();
    bits = 8 - bytes[0].leading_zeros() + 8 * (bytes.len() as u32 - 1);

    bits as usize
}