




// pub fn poly2trellis_bruteforce(constraint_length: Vec<u16>, code_generator: Vec2d<u16>) -> Trellis {

//     const two: usize = 2;
//     let numOutputSymbols: usize = two.pow((code_generator.col) as u32) as usize;
//     let cols: usize = two.pow(constraint_length.len() as u32) as usize;
//     let rows: usize = two.pow((constraint_length.iter().map(|n| n - 1).sum::<u16>()) as u32) as usize;

//     let mut states: Vec2d<u16> = Vec2d::new(Fill(0, rows * cols), rows, cols);

//     for n in 0..states.row {
//         for m in 0..states.col {
//             // shift bits corresponding to row position right by 1
//             // take a 1 and shift to right by constraint_length - 1
//             // take shifted bits OR 

//             // shift states for each set of memory bits
//             *states.index_mut(n, m) = Shift_registers(n, m, &constraint_length);
//             //new_state = ((n >> 1) | (m << (constraint_length[0] - 2))) as u16;
//         }
//     }

//     let mut outputs: Vec2d<u16> = Vec2d::new(Fill(0, rows * cols), rows, cols);

//     for n in 0..outputs.row {
//         for m in 0..outputs.col {

//             let mut output: u32 = 0;
//             // need to modify this so that we grab the 
//             for (i, node) in code_generator.vec.iter().rev().enumerate() {
//                 // crazy ass function
//                 // basically you are calculating the output of each node for a given state
//                 // we start backward because the last node's output will be the LSB
//                 // each node is acting as a bit in an output sum
//                 output += ((node & (n + (m << (constraint_length[0] - 1))) as u16).count_ones() % 2) << i;
//             }
            
//             *outputs.index_mut(n, m) = output as u16;
//         }
//     }

    // let trellis: Trellis = Trellis {
    //     numInputSymbols: cols,
    //     numOutputSymbols: numOutputSymbols, // will have to change this to find the n*m size for a 2d array at some point, but for now (since we only have 1 input) this will work
    //     numStates: rows,
    //     nextStates: states,
    //     outputs: outputs,
    // };

//     trellis
// }

// fn Shift_registers(n: usize, m: usize, constraint_lengths: &Vec<u16>) -> u16 {
//     let mut new_state: u16 = 0;
//     let mut bits_to_skip = 0;
//     for (i, constraint_len) in constraint_lengths.iter().enumerate() {
//         // we want to extract the bits corresponding to each constraint length (constraint_length goes in ascending order from LSB to MSB)

//         // finds the value of the column's bit at index: constraint_lengths.len() - 1 - i
//         let mut input_bit = (m >> constraint_lengths.len() - 1 - i) & 1;
//         //let mut input_bit = Get_bit_at_idx(m, constraint_lengths.len() - i - 1);
//         let mut new_bits_for_constraint: u16 = (((n >> bits_to_skip) & !(!0 << constraint_len)) >> 1 | (input_bit << (constraint_len - 2))) as u16;
//         if n < 3 {
//             println!("index for input_bit for row {} col {}: {}, input bit: {}", n, m, constraint_lengths.len() - i - 1, input_bit);
//         }
//         new_state |= (new_bits_for_constraint << bits_to_skip);
//         bits_to_skip += constraint_len - 1;
//     }

//     new_state
// }