use std::time::Instant;

pub mod d1;

fn main() {
    
    let before = Instant::now();
    d1::p1();
    d1::p2();
    
    let after =  Instant::now();
    println!("Time taken: {}", (after - before).as_micros());

}
