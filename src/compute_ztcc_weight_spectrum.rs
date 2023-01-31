

use std::num::ParseIntError;

use crate::trellis::trellis::Trellis;
use crate::poly_wrapper::{PolyWrapper, PolyToWrapped, WrappedToPoly};
use crate::poly_wrapper::BigUintWrapper;
use polynomen::One;
use polynomen::Zero;

use nalgebra::DMatrix;
use polynomen::{Poly, poly};

//   This function computes the weight spectrum of a given high-rate ZTCC
//   of length N

//   Inputs:
//       1) v-1: the overall constraint length
//       2) numerators: conventional octal form of numerators in 1st column
//       3) denominator: conventional octal form of den. in 1st column
//       4) N: the trellis length

//   Outputs: weight_node, a struct that includes
//       1) weight_spectrum: a (d_max+1)-by-1 column vector denoting the #
//           codewords of weight i. Index 'i' represents weight 'i-1'.
//       2) overall_weight_function: a polynomial representation of WEF

//   Written by Hengjie Yang (hengjie.yang@ucla.edu) 03/30/21
//   Translated into Rust by Holden Grissett (holdengs@g.ucla.edu) 08/15/2022

pub fn compute_ztcc_weight_spectrum(_v: u16, _numerators: Vec<u16>, _denominator: u16, trellis_len: u16, trell: Trellis) -> Result<Poly<BigUintWrapper>, ParseIntError> {

    let num_states = trell.num_states;
    let mut transfer_function: DMatrix<PolyWrapper<BigUintWrapper>> = DMatrix::zeros(num_states, num_states);
    let mut tmp;
    let mut test_vec: Vec<Vec<i16>> = vec![vec![-1; num_states]; num_states];
    println!("Step 1: Compute the transfer function");
    for current_state in 0..trell.num_states {
        for input in 0..trell.num_input_symbols {
            let next_state = trell.next_states.index(current_state, input);
            let output = u16::from(*trell.outputs.index(current_state, input));
            tmp = usize::from(*next_state);
            transfer_function[(current_state, usize::from(*next_state))] = calc_polynomial_from_weight(output.count_ones()); // need to make polynomial conversion function
            test_vec[current_state][tmp] = output.count_ones() as i16;
            if current_state == 0 && tmp == 16 {
                println!("{}", output);
            }
        }
    }

    println!("Step 2: compute the weight enumerating function for each starting state.");
    print_matrix(test_vec);
    let mut identity_matrix: DMatrix<PolyWrapper<BigUintWrapper>> = DMatrix::<PolyWrapper<BigUintWrapper>>::identity(num_states, num_states);
    for i in 0..trellis_len {
        println!("Current depths: {}", i);
        identity_matrix = identity_matrix * &transfer_function;
    }

    // Step 3: Compute the overall weight enumerating function for finite-length ZTCC
    println!("Step 3: Compute the overall weight enumerating function.");

    let ret: Poly<BigUintWrapper> = identity_matrix[(0, 0)].clone().unwrap();
    Ok(ret)
}

fn print_matrix(matrix: Vec<Vec<i16>>) -> () {
    for row in matrix {
        for e in row {
            if e == -1 {
                print!("_ ");
            } else {
                print!("{} ", e);
            }
        }
        println!();
    }
}

fn calc_polynomial_from_weight(weight: u32) -> PolyWrapper<BigUintWrapper> {
    let polynomial: PolyWrapper<BigUintWrapper>;
    if weight == 0 {
        polynomial = poly!(BigUintWrapper::one()).wrap();
    } else if weight == 1 {
        polynomial = poly!(BigUintWrapper::zero(), BigUintWrapper::one()).wrap();
    } else {
        let mut tmp = vec![BigUintWrapper::zero(); weight as usize];
        tmp.push(BigUintWrapper::one());
        polynomial = Poly::new_from_coeffs_iter(tmp).wrap();
    }

    polynomial
}

#[cfg(test)]
mod tests {

    use polynomen::{poly, Zero, One};
    use crate::{poly_wrapper::{PolyToWrapped}, compute_ztcc_weight_spectrum::calc_polynomial_from_weight};
    use crate::poly_wrapper::BigUintWrapper;

    #[test]
    fn test_calc_polynomial_basic() {
        let polynomial = poly!(BigUintWrapper::zero(), BigUintWrapper::zero(), BigUintWrapper::one()).wrap();
        assert_eq!(calc_polynomial_from_weight(2), polynomial);
    }
}