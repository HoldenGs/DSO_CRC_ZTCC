mod find_dominant_error_event_fast;
mod trellis;
mod vec2d;
mod find_irreducible_error_event;
mod compute_ztcc_weight_spectrum_fast;
mod reconstruct_ztps;
mod dso_crc_polynomial_search;
pub mod poly_wrapper;

use std::time::Instant;

use find_irreducible_error_event::{find_irreducible_error_event, ErrorEvents};

use trellis::generate_feedback_trellis::generate_feedback_trellis;

use compute_ztcc_weight_spectrum_fast::compute_ztcc_weight_spectrum_fast;

use reconstruct_ztps::{reconstruct_ztps, ZTPs};

use dso_crc_polynomial_search::dso_crc_polynomial_search;

use std::fs;
use bincode;
use polynomen::Poly;
use num_bigint::BigUint;
use std::env;
use std::path::Path;
use bitvec::prelude::*;


fn main() {
    const V: u16 = 7;
    const NUMERATOR: [u16; 3] = [107, 135, 133];
    const DENOMINATOR: u16 = 141;
    const MAX_SEARCH_DISTANCE: usize = 10;
    const TRELLIS_LEN: u16 = 54;
    const TARGET_CRC_DEGREE: u32 = 10;
    let args: Vec<String> = env::args().collect();

    let instant = Instant::now();

    const PATH_STR: &str = "./simulation_results";

    let mut error_events = ErrorEvents {
        error_events: vec![vec![bitvec![]]],
        error_event_lengths: vec![vec![0]]
    };

    let mut zero_terminated_paths = ZTPs {
        zero_terminated_paths: vec![vec![bitvec![]]],
        aggregate: 0
    };

    let trel = generate_feedback_trellis(V, NUMERATOR, DENOMINATOR);

    // Generates error events if they don't exist or we want to regenerate them
    if args.contains(&"error_events".to_string()) || !Path::new(&format!("{}/error_events.json", PATH_STR)).exists() {
        error_events = find_irreducible_error_event(
            V, NUMERATOR, DENOMINATOR, MAX_SEARCH_DISTANCE, &trel);

        let json = serde_json::to_string(&error_events).expect("couldn't serialize weight spectrum to json");
        fs::write(format!("{}/error_events.json", PATH_STR), json).expect("couldn't write weight spectrum json to file");
    }
    
    // Generates weight spectrum if it doesn't exist or we want to regenerate them
    if args.contains(&"weight_spectrum".to_string()) || !Path::new(&format!("{}/weight_spectrum.json", PATH_STR)).exists() {
        let weight_spectrum = compute_ztcc_weight_spectrum_fast(
            V,  NUMERATOR, DENOMINATOR, TRELLIS_LEN, trel).unwrap();
    
        // let encoded_poly = bincode::serialize(&weight_spectrum.coeffs()).expect("couldn't encode weight spectrum");
        // fs::write(format!("{}/weight_spectrum", PATH_STR), encoded_poly).expect("couldn't write weight spectrum to file");
        let json = serde_json::to_string(&weight_spectrum.coeffs()).expect("couldn't serialize weight spectrum to json");
        fs::write(format!("{}/weight_spectrum.json", PATH_STR), json).expect("couldn't write weight spectrum json to file");
    }

    let weight_spectrum_string: String = json_from_file(&format!("{}/weight_spectrum.json", PATH_STR));
    let decoded_v: Vec<f64> = serde_json::from_str(&weight_spectrum_string).expect("couldn't read weight spectrum from file");
    let weight_spectrum: Poly<f64> = Poly::new_from_coeffs(&decoded_v);

    let error_events_string: String = json_from_file(&format!("{}/error_events.json", PATH_STR));
    let error_events: ErrorEvents = serde_json::from_str(&error_events_string).expect("couldn't read weight spectrum from file");
    // let decoded_v: Vec<usize> = bincode::deserialize(&read_v).expect("Couldn't deserialize weight spectrum");
    // let weight_spectrum = Poly::new_from_coeffs(&decoded_v);
    
    if args.contains(&"reconstruct_ztps".to_string()) || !Path::new(&format!("{}/reconstruct_ztps.json", PATH_STR)).exists() {
        zero_terminated_paths = reconstruct_ztps(V, NUMERATOR,
            DENOMINATOR, MAX_SEARCH_DISTANCE, TRELLIS_LEN, weight_spectrum, error_events);
        let json = serde_json::to_string(&zero_terminated_paths).expect("couldn't serialize ztp to json");
        fs::write(format!("{}/reconstruct_ztps.json", PATH_STR), json).expect("couldn't write ztp json to file");
    }
    
    let ztps_string: String = json_from_file(&format!("{}/reconstruct_ztps.json", PATH_STR));
    let zero_terminated_paths: ZTPs = serde_json::from_str(&ztps_string).expect("couldn't read ztps from file");
    

    //println!("ztp[6,1]: {}", zero_terminated_paths.zero_terminated_paths[6][0]);

    dso_crc_polynomial_search(V, NUMERATOR, DENOMINATOR, MAX_SEARCH_DISTANCE, TRELLIS_LEN, TARGET_CRC_DEGREE, zero_terminated_paths);
    
    println!("elapsed time since start: {:?}", instant.elapsed());
}

fn json_from_file(file: &String) -> String {
    let blob: String = fs::read_to_string(file).expect(&format!("Couldn't read file {}", &file));
    blob
}