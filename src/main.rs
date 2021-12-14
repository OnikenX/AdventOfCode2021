use std::time::Instant;

use crate::downloader::{download, get_token};

mod downloader;

// mod d1;
// mod d2;
// mod d3;
// mod d4;
// mod d5;
// mod d6;
// mod d7;
// mod d8;
// mod d9;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;

fn main() {
    // let input = download(2021, 9, &get_token());
    let start = Instant::now();
    d14::p1();
    println!("timetaken in macros: {}", (Instant::now() - start).as_micros());

}
