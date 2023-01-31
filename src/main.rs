mod find_dominant_error_event_fast;
mod trellis;
mod vec2d;
mod find_irreducible_error_event;
mod compute_ztcc_weight_spectrum_fast;
mod reconstruct_ztps;
pub mod poly_wrapper;

use std::time::Instant;

use find_irreducible_error_event::{find_irreducible_error_event, ErrorEvents};

use trellis::generate_feedback_trellis::generate_feedback_trellis;

use compute_ztcc_weight_spectrum_fast::compute_ztcc_weight_spectrum_fast;

use reconstruct_ztps::reconstruct_ztps;

use std::fs;
use bincode;
use polynomen::Poly;
use num_bigint::BigUint;
use std::env;
use std::path::Path;


fn main() {
    const v: u16 = 7;
    const numerator: [u16; 3] = [107, 135, 133];
    const denominator: u16 = 141;
    const max_search_distance: usize = 10;
    const trellis_len: u16 = 54;
    let args: Vec<String> = env::args().collect();

    let instant = Instant::now();

    const path_str: &str = "./simulation_results";

    let mut error_events = find_irreducible_error_event::ErrorEvents {
        error_events: vec![vec![BigUint::from(0 as usize)]],
        error_event_lengths: vec![vec![0]]
    };

    let trel = generate_feedback_trellis(v, numerator, denominator);

    // Generates error events if they don't exist or we want to regenerate them
    if args.contains(&"error_events".to_string()) || !Path::new(&format!("{}/error_events", path_str)).exists() {
        error_events = find_irreducible_error_event(
            v, numerator, denominator, max_search_distance, &trel);

        let json = serde_json::to_string(&error_events).expect("couldn't serialize weight spectrum to json");
        fs::write(format!("{}/error_events.json", path_str), json).expect("couldn't write weight spectrum json to file");
    }
    
    // Generates weight spectrum if it doesn't exist or we want to regenerate them
    if args.contains(&"weight_spectrum".to_string()) || !Path::new(&format!("{}/weight_spectrum", path_str)).exists() {
        let weight_spectrum = compute_ztcc_weight_spectrum_fast(
            v,  numerator, denominator, trellis_len, trel).unwrap();
    
        let encoded_poly = bincode::serialize(&weight_spectrum.coeffs()).expect("couldn't encode weight spectrum");
        fs::write(format!("{}/weight_spectrum", path_str), encoded_poly).expect("couldn't write weight spectrum to file");
        let json = serde_json::to_string(&weight_spectrum.coeffs()).expect("couldn't serialize weight spectrum to json");
        fs::write(format!("{}/weight_spectrum.json", path_str), json).expect("couldn't write weight spectrum json to file");
    }

    let decoded_v: Vec<f64> = serde_json::from_str(&format!("{}/weight_spectrum.json", path_str)).expect("couldn't read weight spectrum from file");
    let weight_spectrum: Poly<f64> = Poly::new_from_coeffs(&decoded_v);

    let error_events: ErrorEvents = serde_json::from_str(&format!("{}/error_events.json", path_str)).expect("couldn't read weight spectrum from file");
    // let decoded_v: Vec<f64> = bincode::deserialize(&read_v).expect("Couldn't deserialize weight spectrum");
    // let weight_spectrum = Poly::new_from_coeffs(&decoded_v);
    

    let zero_terminated_paths = reconstruct_ztps(v, numerator,
    denominator, max_search_distance, trellis_len, weight_spectrum, error_events);

    
    println!("elapsed time since start: {:?}", instant.elapsed());
}