mod find_dominant_error_event_fast;
mod trellis;
mod Vec2d;
mod find_irreducible_error_event;

use std::time::Instant;

use find_irreducible_error_event::find_irreducible_error_event;

use trellis::generate_feedback_trellis::generate_feedback_trellis;

fn main() {
    //find_dominant_error_event_fast(vec![4], Vec2d::Vec2d::new(vec![0o13, 0o17], 1, 2), 20);

    //find_dominant_error_event_fast(vec![5, 4], Vec2d::Vec2d::new(vec![23, 35, 0, 0, 5, 13], 2, 3), 20);

    let now = Instant::now();
    
    let n = 100;

    for i in 0..n {
        generate_feedback_trellis(6, vec![13, 17], 5);
    }

    let elapsed_time = now.elapsed();
    println!("Running it took {:?}", elapsed_time);

    //find_irreducible_error_event(6, vec![13, 17], 5, 10, trellis);
}