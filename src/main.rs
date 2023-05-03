use std::io::{self, BufRead};
mod anfield;
mod piece;
mod player;
mod util;
use anfield::*;
use piece::*;
use player::*;
use util::*;
mod algorithm;
use algorithm::*;
fn main() {
    let stdin = io::stdin();
    let mut anfield = Anfield::new();
    let mut piece = Piece::new();
    let mut input: String;
    for line in stdin.lock().lines() {
        input = line.unwrap();
        if input.trim().len() == 0 {
            continue;
        }
        if anfield.is_collect() {
            match get_line(&input) {
                Some(inp) => anfield.collect(inp.to_string()),
                None => {}
            }
            continue;
        }
        if piece.is_collect() {
            piece.collect(input);
            if !piece.is_collect() {
                //TODO: run the algorithm
                let opts = find_available_options(
                    anfield.get_board(),
                    piece.get_piece(),
                );
                // println!("{:?}", opts);
                let res = get_best_option(opts);

                println!("{} {}", res.0, res.1);
                piece.clear();
                anfield.clear();
            }
            continue;
        }
        if is_anfield_info(&input) {
            match get_size(&input) {
                Some((w, h)) => {
                    anfield.set_size((w, h));
                    anfield.set_is_collect(true);
                }
                None => {}
            }
            continue;
        }
        if is_piece_info(&input) {
            match get_size(&input) {
                Some((w, h)) => {
                    piece.set_size((w, h));
                    piece.set_is_collect(true);
                }
                None => {}
            }
        }
    }
}
