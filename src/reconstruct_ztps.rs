
use std::path;

use num_traits::Zero;
use polynomen::Poly;
use nalgebra::DMatrix;
use num_bigint::BigUint;

use crate::find_irreducible_error_event::ErrorEvents;

//  This function is to reconstruct all zero-terminated paths (ZTPs) from the
//  irreducible error events (IEEs). Traditional method by Lou et al. does
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
//    1) Need to run "find_irreducible_error_event.m" if IEEs are not
//        generated before.
//    2) Need to run "Compute_ZTCC_weight_spectrum.m" if weight_node is not
//        generated before.
//    3) The distance index is true distance plus one

//  Written by Hengjie Yang (hengjie.yang@ucla.edu)   04/17/21
pub fn reconstruct_ztps(
    v: u16, numerators: [u16; 3], octal_denominator: u16,
    max_search_distance: usize, trellis_len: u16,
    weight_spectrum: Poly<f64>, err_events: ErrorEvents) -> Vec<DMatrix<u16>> {
    
    let k = numerators.len() as u32;
    
    if max_search_distance > weight_spectrum.degree().unwrap() {
        println!("max_search_distance ({}) is larger than weight_spectrum degree ({})",
                max_search_distance, weight_spectrum.degree().unwrap());
        panic!("max_search_distance ({}) is larger than weight_spectrum degree ({})",
        max_search_distance, weight_spectrum.degree().unwrap())
    }

    let zero_terminated_paths: Vec<DMatrix<u16>> = vec![DMatrix::zeros(1, 1); max_search_distance + 1];

    println!("% Step 2: use dynamic programming to reconstruct the length-kN ZTPs.");

    // Add trivial error event to our data
    let mut error_events = err_events.error_events.clone();
    error_events.insert(0, vec![BigUint::zero(); k as usize]);
    let mut error_event_lengths = err_events.error_event_lengths.clone();
    error_event_lengths.insert(0, vec![k]);
    println!("e_event_lens shape: {} {}", error_event_lengths.len(), error_event_lengths[0].len());

    let mut temp_ztps: Vec<Vec<Vec<BigUint>>> = vec![vec![vec![]; trellis_len as usize + 1]; max_search_distance + 1];

    for distance in 0..max_search_distance {
        println!("Current distance: {}", distance);

        for test_length in 0..trellis_len {
            for weight in (0..distance).rev() {
                for path_index in 0..error_events[weight].len() {
                    println!("error_events[weight].len(): {}", error_event_lengths[weight].len());
                    println!("path_index: {}", path_index);
                    let error_len = error_event_lengths[weight][path_index] / k;
                    if weight == distance && error_len as u16 == test_length {
                        let tmp = BigUint::zero();
                        for i in 0..k*error_len {
                            
                        }
                        temp_ztps[distance][test_length as usize].push(error_events[weight][path_index].clone());
                    }
                }
            }
        }
    }

    zero_terminated_paths
}