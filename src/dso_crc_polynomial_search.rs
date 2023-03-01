

use crate::reconstruct_ztps::{ZTPs};

use bitvec::prelude::BitVec;
use bitvec::prelude::*;
use galois_field::*;
use rayon::prelude::{IntoParallelIterator, IndexedParallelIterator};
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone)]
pub struct Poly<T> {
    pub poly: Polynomial,
    pub poly_data: T,
    pub data_idx: u16
}

#[derive(Debug, Clone)]
pub struct ZTP_Polys {
    pub zero_terminated_paths: Vec<Vec<Poly<BitVec>>>,
    pub aggregate: usize
}

pub fn dso_crc_polynomial_search(v: u16, numerators: [u16; 3], octal_denominator: u16,
    max_search_distance: usize, trellis_len: u16, polynomial_degree: u32,
    classic_ztps: ZTPs) -> u16 {

    let mut crc_polynomial: Poly<u16> = Poly { poly: Polynomial { coef: vec![] }, poly_data: 0, data_idx: 0 };

    let stopped_distance = -1;
    let mut success = false;
    let mut min_distance: i32 = -1;
    let k: u16 = numerators.len() as u16; // # input rails
    let mu: u32 = ((v as f32 - 1.) / k as f32).ceil() as u32; // the # termination transitions

    
    println!("Step 0: convert CRCs into polynomial data structure");
    let base: u16 = 2;
    let list_size: u16 = base.pow(polynomial_degree - 1);
    let mut candidate_crcs: Vec<Poly<u16>> = generate_crcs(list_size, polynomial_degree);
    let mut undetected_spectrum: Vec<Vec<i32>> = vec![vec![-1]; list_size as usize];
    let ztps = convert_ztps(classic_ztps, k, mu);

    println!("Step 1: search the DSO CRC polynomial");

    if polynomial_degree == 1 {
        let e1: FiniteField = FiniteField{
            char: 2,
            element: Element::PrimeField {element: 1} // 1
        };
        candidate_crcs = vec![Poly { poly: Polynomial { coef: vec![e1.clone(), e1.clone()] }, poly_data: 3, data_idx: 0}];
    }

    let mut locations: Vec<u16> = (0..list_size as u16).map(|i: u16| { i }).collect();

    let mut crc_gen_polynomials = vec![];
    

    for distance in 1..(max_search_distance + 1) {
        for i in 0..list_size as usize {
            undetected_spectrum[i].push(-1);
        }
        
        let mut weight_vector: Vec<u32> = vec![100000; locations.len()];
        if !ztps.zero_terminated_paths[distance].is_empty() {
            weight_vector = candidate_crcs.clone().into_par_iter().map(|crc| {
                let weight = check_divisible_by_distance(crc.clone(), ztps.zero_terminated_paths[distance].clone(), k, mu);
                weight
            }).collect();

            for i in 0..locations.len() {
                println!("weight @ {}: {}", locations[i], weight_vector[i]);
                undetected_spectrum[locations[i] as usize][distance] = weight_vector[i] as i32;
            }

            let min_weight = weight_vector.iter().min().unwrap();
            let mut min_locations = vec![];
            for location in locations {
                if &(weight_vector[location as usize] as u32) == min_weight {
                    min_locations.push(location);
                }
            }
            locations = min_locations.clone();
            println!("Current distance: {}, number of candidates: {}", distance, min_locations.len());

            if locations.len() == 1 {
                crc_gen_polynomials = vec![];
                crc_gen_polynomials.push(candidate_crcs[locations[0] as usize].clone());
                success = true;
                break;
            }
        }

        if distance == max_search_distance && locations.len() > 1 {
            crc_gen_polynomials = vec![];
            for location in &locations {
                crc_gen_polynomials.push(candidate_crcs[*location as usize].clone());
            }
            let stopped_distance = max_search_distance;
            println!("max_search_distance is insufficient to find the DSO CRC...");
            println!("Stopped distance: {stopped_distance}");
            println!("# of candidate polynomials: {}", crc_gen_polynomials.len());
        }
    }

    if success {
        println!("Step 4: Identify the minimum undetected distance by the DSO CRC");
        for distance in 1..max_search_distance {
            if !ztps.zero_terminated_paths[distance].is_empty() {
                let w = check_divisible_by_distance(crc_gen_polynomials[0].clone(), ztps.zero_terminated_paths[distance].clone(), k, mu);
                if w > 0 {
                    min_distance = distance as i32;
                    crc_polynomial = crc_gen_polynomials[0].clone();
                    println!("DSO CRC Polynomial: {:#x}", crc_gen_polynomials[0].poly_data);
                    println!("Minimum undetected distance: {}", min_distance);
                    break;
                }
                if w == 0 && distance == max_search_distance {
                    println!("max_search_distance is insufficient to determine the minimum undetected distance");
                }
            }
        }
    }

    println!("{:#x}", crc_polynomial.poly_data);

    crc_polynomial.poly_data
}


fn generate_crcs(list_size: u16, polynomial_degree: u32) -> Vec<Poly<u16>> {
    let char: u32 = 2;
    let e0: FiniteField = FiniteField{
        char: char,
        element: Element::PrimeField {element: 0} // 0
    };
    let e1: FiniteField = FiniteField{
        char: char,
        element: Element::PrimeField {element: 1} // 1
    };

    let crc_polys = (0..list_size).map(|crc_idx: u16| {
        let mut crc: u16;
        crc = crc_idx | (1 << (polynomial_degree - 1));
        crc = crc << 1;
        crc = crc | 1;
        let crc_vec: Vec<FiniteField> = (0..(polynomial_degree + 1)).rev().map(|x| { 
            if (1 & (crc >> x)) == 0 { return e0.clone(); }
            else { return e1.clone() };
        }).collect();
        let crc_poly: Polynomial = Polynomial { coef: crc_vec };
        Poly {
            poly: crc_poly,
            poly_data: crc,
            data_idx: crc_idx
        }
    }).collect();

    crc_polys
}

fn convert_ztps(ztps: ZTPs, k: u16, mu: u32) -> ZTP_Polys {
    let char: u32 = 2;
    let e0: FiniteField = FiniteField{
        char: char,
        element: Element::PrimeField {element: 0} // 0
    };
    let e1: FiniteField = FiniteField{
        char: char,
        element: Element::PrimeField {element: 1} // 1
    };
    let poly = Poly { poly: Polynomial { coef: vec![] }, poly_data: bitvec![], data_idx: 0};

    let new_ztps: Vec<Vec<Poly<BitVec>>> = ztps.zero_terminated_paths.iter().map(|ees_at_distance| {
        let poly_ees_at_distance: Vec<Poly<BitVec>> = ees_at_distance.iter().map(|b| {
            
            let mut err_vec: Vec<FiniteField> = b.iter().map(|x| { 
                if !x { return e0.clone(); }
                else { return e1.clone(); }
            }).collect();

            err_vec.truncate(err_vec.len() - (k as usize * mu as usize));
            err_vec = err_vec.into_iter().collect();

            let err_poly: Poly<BitVec> = Poly {
                poly: Polynomial { coef: err_vec },
                poly_data: b.clone(),
                data_idx: 0
            };

            err_poly
        }).collect();

        poly_ees_at_distance
    }).collect();

    let new_ztp_struct = ZTP_Polys {
        zero_terminated_paths: new_ztps,
        aggregate: ztps.aggregate
    };

    new_ztp_struct
}


fn check_divisible_by_distance(crc: Poly<u16>, error_events: Vec<Poly<BitVec>>, k: u16, mu: u32) -> u32 {
    let mut weight: u32 = 0;

    let crc_vec_print: Vec<u16> = (0..(10 + 1)).map(|x| { 1 & (crc.poly_data >> x) }).collect();
    println!("crc_poly: {:?}", crc_vec_print);

    for (i, error_event) in error_events.iter().enumerate() {
        // divide error event by polynomial to see if it catches the error

        let mut err_vec_print: Vec<u8> = error_event.poly_data.iter().map(|x| { 
            if !x { return 0; }
            else { return 1; }
        }).collect();

        err_vec_print.truncate(err_vec_print.len() - (k as usize * mu as usize));

        err_vec_print = err_vec_print.into_iter().rev().collect();

        let remainder = error_event.poly.clone() % crc.poly.clone();
        //let q = error_event.poly.clone() / crc.poly.clone();

        // let rem_vec: Vec<u8> = remainder.coef.iter().map(|x| {
        //     if x.is_1() { return 1; }
        //     else { return 0; };
        // }).collect();
        // let q_vec: Vec<u8> = q.coef.iter().map(|x| {
        //     if x.is_1() { return 1; }
        //     else { return 0; };
        // }).collect();

        if !remainder.coef.iter().any(|x| x.is_1()) {
            weight += 1;
        }

        // if i == 47 && crc.poly_data == 1045 {
        //     let crc_vec_print: Vec<u16> = (0..(10 + 1)).map(|x| { 1 & (crc.poly_data >> x) }).collect();
        //     println!("crc_poly: {:?}", crc_vec_print);
        //     println!("err_poly: {:?}", err_vec_print);
        //     println!("{} rem: {:?}", k as usize * mu as usize, rem_vec);
        //     println!("q: {:?}", q_vec);
        //     println!("test");
        // }

    }
    //println!("{weight}");

    weight
}