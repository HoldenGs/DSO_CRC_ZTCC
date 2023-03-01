
use crate::trellis::trellis::Trellis;

use convert_base::Convert;
use ::gf256::p16;
use ::bit_reverse::LookupReverse;
use ::queues::{IsQueue, Queue, queue};

use crate::vec2d::{
    Vec2d,
    fill_p16,
    fill,
};

// Creates a trellis and termination sequence for a nonsystematic high-rate ( R = (n-1) / n ) feedback encoder
pub fn generate_feedback_trellis(v: u16, numerators: [u16; 3], octal_denominator: u16) -> Trellis {

    const TWO: usize = 2;
    let k: u16 = numerators.len() as u16;
    let n: u16 = k + 1;

    let _base: convert_base::Convert = Convert::new(8, 2);

    let num_input_symbols: usize = TWO.pow(k.into());
    let num_output_symbols: usize = TWO.pow(n.into());
    let num_states: usize = TWO.pow((v - 1).into());

    let mut next_states: Vec2d<p16> = Vec2d::new(fill_p16(0, num_states * num_input_symbols), num_states, num_input_symbols);
    let mut outputs: Vec2d<p16> = Vec2d::new(fill_p16(0, num_states * num_input_symbols), num_states, num_input_symbols);

    let decimal_denominator: u16 = u16::from_str_radix(&octal_denominator.to_string(), 8).unwrap();

    // Reverse the vector and convert the octal numbers into their binary representation
    // (They will still look like base-10, but their underlying binary digits will align with the original octal representation)
    let revved_nums: Vec<p16> = numerators.iter().map(|n| p16(u16::from_str_radix(&n.to_string(), 8).unwrap())).rev().collect::<Vec<p16>>();

    for current_state in 0..num_states {
        for input_symbol in 0..num_input_symbols {

            let input_symbol_revved: u16 = ((input_symbol as u16).swap_bits() >> (16 - k)) as u16;
            let mut total_numerator: p16 = p16(0);

            for nth_digit in 0..k {
                let nth_bit: p16 = p16(((input_symbol_revved >> nth_digit) & 1) as u16);
                total_numerator += revved_nums[nth_digit as usize] * nth_bit;
            }
            total_numerator += p16(current_state as u16);

            // get remainder
            let revved_total_num = p16((u16::from(total_numerator)).swap_bits() >> (16 - v));
            let revved_denom = p16(decimal_denominator.swap_bits() >> (16 - v));
            let quotient = revved_total_num.naive_div(revved_denom);
            let remainder = revved_total_num.naive_rem(revved_denom);
            let revved_remainder: p16 = p16((u16::from(remainder)).swap_bits() >> (16 - v + 1));
            *next_states.index_mut(current_state, input_symbol) = revved_remainder;
            *outputs.index_mut(current_state, input_symbol) = p16(input_symbol as u16) + (quotient << k); // add quotient bit after the other bits
        }
    }

    // println!("outputs: {}", outputs);
    // println!("next states: {}", next_states);

    
    // Find shortest termination sequence

    let mut queue: Queue<u16> = queue![0];
    let mut visited: Vec<u16> = fill(0, num_states);
    let mut tree: Vec<u16> = fill(0, num_states);
    visited[0] = 1;

    while queue.size() != 0 {
        let target_state_option = queue.remove();
        match target_state_option {
            Ok(target_state) => {
                for pre_state in 0..num_states {
                    if next_states.row(pre_state).contains(&p16(target_state)) && visited[pre_state] == 0 {
                        visited[pre_state] = 1;
                        let res = queue.add(pre_state as u16);
                        match res {
                            Ok(correct) => correct,
                            Err(error) => panic!("Problem adding to queue: {:?}", error),
                        };
                        tree[pre_state] = target_state;
                    }
                }
            },
            Err(_error) => break,
        }
    }


    let mut terminations: Vec<Vec<u16>> = vec![vec![]; num_states];

    terminations[0].push(0);

    for current_state in 1..num_states {
        let mut current_tmp = current_state;
        while current_tmp != 0 {
            let father_state = tree[current_tmp];
            let index_option = next_states.row(current_tmp).iter().position(|&x| x == p16(father_state));
            match index_option {
                Some(index) => terminations[current_state].push(index as u16),
                None => (),
            }
            current_tmp = usize::from(father_state);
        }
    }

    let num_transitions: usize = usize::from((v-1 + k - 1) / k);
    for current_state in 0..num_states {
        let length = terminations[current_state].len();
        if length < num_transitions {
            let mut tmp = vec![0_u16; num_transitions - length];
            terminations[current_state].append(&mut tmp);
        }
    }

    //println!("{:?}", terminations);

    let trellis: Trellis = Trellis {
        num_input_symbols: num_input_symbols,
        num_output_symbols: num_output_symbols,
        num_states: num_states,
        next_states: next_states,
        outputs,
        terminations,
    };

    trellis
}