use std::fs::read_to_string;

use d1::read_file;
use downloader::{get_token, head};

mod d1;
mod d2;
mod downloader;
mod d3;
mod d4;

fn main() {
    let input = downloader::download(2021, 4, &get_token());
    // let input = read_to_string("input-test.txt").unwrap();
    // head(&input.lines().collect::<Vec<_>>(), 10);
    d4::p2(input);
}
