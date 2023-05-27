use std::io::{self, BufRead};
mod util;
use util::*;
mod bot;
use bot::{algorithm, anfield, piece};
// mod bot;
fn main() {
    let stdin = io::stdin();
    let mut anfield = anfield::Anfield::new();
    let mut piece = piece::Piece::new();
    let mut input: String;
    for line in stdin.lock().lines() {
        input = line.unwrap();
        if input.is_empty() {
            continue;
        }
        if anfield.is_collect() {
            if let Some(inp) = get_line(&input) {
                anfield.collect(inp);
            }
            continue;
        }
        if piece.is_collect() {
            piece.collect(input);
            if !piece.is_collect() {
                //TODO: run the algorithm
                let opts =
                    algorithm::find_available_options(anfield.get_board(), piece.get_piece());
                // println!("{:?}", opts);
                let res = algorithm::get_best_option(opts);

                println!("{} {}", res.0, res.1);
                piece.clear();
                anfield.clear();
            }
            continue;
        }
        if is_anfield_info(&input) {
            if let Some((w, h)) = get_size(&input) {
                anfield.set_size((w, h));
                anfield.set_is_collect(true);
            }
            continue;
        }
        if is_piece_info(&input) {
            if let Some((w, h)) = get_size(&input) {
                piece.set_size((w, h));
                piece.set_is_collect(true);
            }
        }
    }
}
