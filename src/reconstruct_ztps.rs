

use polynomen::Poly;
use bitvec::prelude::*;

use std::fs::File;
use std::io::{BufWriter, Write};


use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZTPs {
    pub zero_terminated_paths: Vec<Vec<BitVec>>,
    pub aggregate: usize
}

use crate::find_irreducible_error_event::ErrorEvents;

//  This function is to reconstruct all zero-terminated paths (ZTPs) from the
//  irreducible error events (iees). Traditional method by Lou et al. does
//  not adapt to the high-rate ZTCCs due to nontrivial terminations.

//  Input parameters:
//    1) v: (v-1) denotes # memory elements in systematic feedback encoder.
//    2) numerators: a 1-by-k row vector with entries in octal, where k
//    denotes the # input rails
//    3) denominator: a scalar in octal
//    4) d_tilde: a scalar denoting the distance threshold (achievable)
//    5) N: a scalar denoting the primal trellis length

//  Output parameters: ZTP_node a struct composed of following fields
//    1) list: a d_tilde-by-1 column vector denoting the list of length-kN
//        ZTPs arranged in ascending distances. The true distance is one less
//        than its index.
//    2) aggregate: a scalar denoting the number of length-kN ZTPs of
//        distance less than 'd_tilde'.

//  Remarks:
//    1) Need to run "find_irreducible_error_event.m" if iees are not
//        generated before.
//    2) Need to run "Compute_ZTCC_weight_spectrum.m" if weight_node is not
//        generated before.
//    3) The distance index is true distance plus one

//  Written by Hengjie Yang (hengjie.yang@ucla.edu)   04/17/21
pub fn reconstruct_ztps(
    _v: u16, numerators: [u16; 3], _octal_denominator: u16,
    max_search_distance: usize, trellis_len: u16,
    weight_spectrum: Poly<f64>, err_events: ErrorEvents) -> ZTPs {
    
    let k = numerators.len() as u32;
    
    if max_search_distance > weight_spectrum.degree().unwrap() {
        println!("max_search_distance ({}) is larger than weight_spectrum degree ({})",
                max_search_distance, weight_spectrum.degree().unwrap());
        panic!("max_search_distance ({}) is larger than weight_spectrum degree ({})",
        max_search_distance, weight_spectrum.degree().unwrap())
    }

    let mut zero_terminated_paths: Vec<Vec<BitVec>> = vec![vec![]; max_search_distance + 1];

    println!("% Step 2: use dynamic programming to reconstruct the length-kN ZTPs.");

    // Add trivial error event to our data
    let mut error_events = err_events.error_events.clone();
    error_events.insert(0, vec![bitvec![0; k as usize]]);
    let mut error_event_lengths = err_events.error_event_lengths.clone();
    error_event_lengths.insert(0, vec![k]);

    let mut temp_ztps: Vec<Vec<Vec<BitVec>>> = vec![vec![vec![]; trellis_len as usize + 1]; max_search_distance + 1];

    for distance in 0..(max_search_distance + 1) {
        println!("\tCurrent distance: {}", distance);

        for test_length in 1..(trellis_len + 1) as usize {
            for weight in (0..(distance + 1)).rev() {
                for path_index in 0..error_events[weight].len() {
                    let error_len: usize = (error_event_lengths[weight][path_index] / k) as usize;

                    if weight == distance && error_len == test_length {
                        let mut iee = error_events[weight][path_index].clone();
                        iee.resize(error_len * k as usize, true); // truncate to true path length
                        temp_ztps[distance][test_length].push(iee);
                        
                    } else if error_len < test_length && !temp_ztps[distance - weight][test_length - error_len].is_empty() {
                        let num_rows: usize = temp_ztps[distance - weight][test_length - error_len].len();
                        let mut iee: BitVec = error_events[weight][path_index].clone();
                        iee.resize(error_len * k as usize, true); // truncate to true path length

                        // create our new ztp
                        let mut new_ztps: Vec<BitVec> = vec![];
                        for i in 0..num_rows {
                            new_ztps.push(temp_ztps[distance - weight][test_length - error_len][i].clone());
                            new_ztps[i].append(&mut iee);
                        }

                        // add it to the list
                        temp_ztps[distance][test_length].append(&mut new_ztps);
                    }
                }
            }
        }
    }

    for distance in 0..(max_search_distance + 1) {
        if !temp_ztps[distance][trellis_len as usize].is_empty() {
            zero_terminated_paths[distance] = temp_ztps[distance][trellis_len as usize].clone();
        }
    }

    // add zeros to end
    let zero_terminated_paths: Vec<Vec<BitVec>> = zero_terminated_paths.iter().map(|x| {
        let new = x.iter().map(|b| {
            let mut new_b = b.clone();
            if b.len() < (trellis_len as u32 * k) as usize {
                let length = (trellis_len as u32 * k) as usize - b.len();
                new_b.extend(bitvec![0; length]);
            }
            new_b
        }).collect();
        new
    }).collect();

    println!("Step 3: check if shifting is required.");

    let mut need_shift = false;
    for distance in 0..(max_search_distance + 1) {
        if zero_terminated_paths[distance].len() != weight_spectrum[distance] as usize {
            need_shift = true;
            break;
        }
    }

    let mut aggregate = 0;
    for distance in 0..(max_search_distance + 1) {
        aggregate += zero_terminated_paths[distance].len();
    }



    if !need_shift {
        println!("Congratulations! No need to shift before saving results!");
    } else {
        println!("Sorry, the shifting operation is required...");
    }

    // Write to file that's easy to parse
    let f = File::create("ztps_file.txt").expect("unable to create file");
    let mut f = BufWriter::new(f);
    for (i, error_event) in zero_terminated_paths[6].clone().iter().enumerate() {
        let err_vec_print: Vec<u8> = error_event.iter().map(|x| { 
            if !x { return 0; }
            else { return 1; }
        }).collect();
        writeln!(f, "{:?}", err_vec_print).expect("error writeing!");
    }

    let ztps: ZTPs = ZTPs {
        zero_terminated_paths: zero_terminated_paths,
        aggregate: aggregate
    };

    ztps
}