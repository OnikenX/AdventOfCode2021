// mod d1;
// mod d2;
mod downloader;
// mod d3;
// mod d4;
mod d5;

fn main() {
    let input = downloader::download(2021, 5, &downloader::get_token());
    d5::p1(input);
}
