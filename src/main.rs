use std::{time::Instant, io::Write};

use downloader::get_token;

mod d1;
mod d2;
mod downloader;
mod d3;

fn main() {
    let input = downloader::download(2021, 3, &get_token());
    // let input = "".to_string();
    d3::p2(input);
}
