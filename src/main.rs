use std::time::Instant;

mod d1;
mod d2;

fn main() {
    
    let before = Instant::now();
    // d1::p1();
    // d1::p2();
    d2::p2();
    let after =  Instant::now();
    println!("Time taken: {}μs", (after - before).as_micros());
}
