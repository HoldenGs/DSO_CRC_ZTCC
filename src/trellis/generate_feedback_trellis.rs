
use crate::trellis::trellis::Trellis;

use convert_base::Convert;
use ::gf256::*;
use ::bit_reverse::LookupReverse;

use crate::Vec2d::{
    Vec2d,
    Fill_p16,
};

// Creates a trellis and termination sequence for a nonsystematic high-rate ( R = (n-1) / n ) feedback encoder
pub fn generate_feedback_trellis(v: u16, numerators: Vec<u16>, denominator: u16) -> () {

    const two: usize = 2;
    let k: u8 = numerators.len() as u8;
    let n: u8 = k + 1;

    let mut base: convert_base::Convert = Convert::new(8, 2);

    let num_input_symbols: usize = two.pow(k.into());
    let num_output_symbols: usize = two.pow(n.into());
    let num_states: usize = two.pow((v - 1).into());

    let mut next_states: Vec2d<p16> = Vec2d::new(Fill_p16(0, num_states * num_input_symbols), num_states, num_input_symbols);
    let mut outputs: Vec2d<p16> = Vec2d::new(Fill_p16(0, num_states * num_input_symbols), num_states, num_input_symbols);

    let binary_denominator: p16 = p16(u16::from_str_radix(&denominator.to_string(), 8).unwrap());

    // Reverse the vector and convert the octal numbers into their binary representation
    // (They will still look like base-10, but their underlying binary digits will align with the original octal representation)
    let revved_nums: Vec<p16> = numerators.clone().iter().map(|n| p16(u16::from_str_radix(&n.to_string(), 8).unwrap())).rev().collect::<Vec<p16>>();

    println!("{:?}", revved_nums);

    for current_state in 0..num_states {
        for input_symbol in 0..num_input_symbols {

            let input_symbol_revved: u16 = ((input_symbol as u16).swap_bits() >> (16 - k)) as u16;
            let mut total_numerator: p16 = p16(0);

            for nth_digit in 0..k {
                let nth_bit: p16 = p16(((input_symbol_revved >> nth_digit) & 1) as u16);
                total_numerator += revved_nums[nth_digit as usize] * nth_bit;
                if current_state == 0 && input_symbol == 2 {
                    println!("nth digit {}\nnth bit: {}\n ", nth_digit, nth_bit);
                }
            }
            total_numerator += p16(current_state as u16);

            // get remainder 
            let revved_total_num = p16((u16::from(total_numerator)).swap_bits() >> (16 - v));
            let revved_denom = p16(denominator.swap_bits() >> (16 - v));
            let quotient = revved_total_num.naive_div(revved_denom);
            let remainder = revved_total_num.naive_rem(revved_denom);
            //let remainder: p16 = total_numerator.naive_rem(binary_denominator);
            let revved_remainder: p16 = p16((u16::from(remainder)).swap_bits() >> (16 - v + 1));
            *next_states.index_mut(current_state, input_symbol) = revved_remainder;
            *outputs.index_mut(current_state, input_symbol) = p16(input_symbol as u16) + (quotient << k); // add quotient bit after the other bits
            if current_state == 2 && input_symbol == 0 {
                println!("input_symbol: {:b}", input_symbol);
                println!("total num: {:b}", total_numerator);
                println!("total denom: {:b}", denominator);
                println!("total num: {:b}", revved_total_num);
                println!("total denom: {:b}", revved_denom);
                println!("remainder: {:b}", revved_remainder);
            }
        }
    }

    println!("outputs: {}", outputs);
    println!("next states: {}", next_states);

    
}