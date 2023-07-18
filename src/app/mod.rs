use super::util;
use std::io::{self, BufRead};
use std::{thread, time};

use super::bot::{algorithm, anfield, piece};

#[derive(Default)]
pub struct App {
    state: State,
    anfield: anfield::Anfield,
    piece: piece::Piece,
    player: Player,
    chars: (char, char),
    opp_chars: (char, char),
}

const PLAYER1_CHARS: (char, char) = ('a', '@');
const PLAYER2_CHARS: (char, char) = ('s', '$');
impl App {
    pub fn start(&mut self) {
        let stdin = io::stdin();
        let mut input: String;
        for line in stdin.lock().lines() {
            // match line {
            //     Ok(inp) => input = inp,
            //     Err(_) => {
            //         return;
            //     }
            // }
            input = line.unwrap();
            if input.is_empty() {
                continue;
            }
            // if self.is_switch_turn(input.as_str()) {
            //     self.switch_turn();
            //     continue;
            // }
            match self.state {
                State::Flow => self.flow(input.as_str()),
                State::CollectingPiece => self.collect_piece(input.as_str()),
                State::CollectingAnfield => self.collect_anfield(input.as_str()),
                State::OppositeTurn => self.opposite_turn(input.as_str()),
            }
        }
    }
    fn flow(&mut self, input: &str) {
        if input.is_empty() {
            return;
        }
        if util::is_anfield_info(input) {
            if let Some(size) = util::parse_size(input) {
                self.anfield.set_size(size);
                self.state = State::CollectingAnfield;
            };
            return;
        }
        if util::is_piece_info(input) {
            if let Some(size) = util::parse_size(input) {
                self.piece.set_size(size);
                self.state = State::CollectingPiece;
            }
            return;
        }
        if util::is_player_info(input) {
            match util::is_player_1(input) {
                true => {
                    self.player = Player::Player1;
                    self.chars = PLAYER1_CHARS;
                    self.opp_chars = PLAYER2_CHARS;
                }
                false => {
                    self.player = Player::Player2;
                    self.chars = PLAYER2_CHARS;
                    self.opp_chars = PLAYER1_CHARS;
                    self.state = State::OppositeTurn
                }
            }
        }
    }
    fn collect_piece(&mut self, input: &str) {
        self.piece.collect(util::parse_bits(
            util::parse_line(input).unwrap(),
            ('O', 'O'),
        ));
        if self.piece.is_full() {
            self.algorithm();
        }
    }
    fn collect_anfield(&mut self, input: &str) {
        let parsed_line = util::parse_line(input).unwrap();
        self.anfield
            .collect(util::parse_bits(parsed_line, self.chars));
        self.anfield
            .collect_opposite(util::parse_bits(parsed_line, self.opp_chars));
        self.anfield.increment_counter();
        if self.anfield.is_full() {
            self.state = State::Flow;
        }
    }
    fn algorithm(&mut self) {
        let board = self.anfield.get_board();
        let opp = self.anfield.get_opposite_board();
        let piece = self.piece.get_piece();
        let max = self.anfield.get_line_length() - self.piece.get_line_length();
        let opp_coords = algorithm::find_available_options(board, opp, piece, max as usize);
        // let opts = algorithm::find_available_options(board, opp, piece, max);W
        println!("{} {}", opp_coords[0].1, opp_coords[0].0);
        // self.state = State::OppositeTurn;
        self.anfield.clear();
        self.piece.clear();
    }
    fn opposite_turn(&mut self, input: &str) {
        if self.is_switch_turn(input) {
            println!("here");
            self.state = State::Flow;
        }
    }

    fn is_switch_turn(&mut self, input: &str) -> bool {
        input.starts_with("-> Answer")
    }
    fn switch_turn(&mut self) {
        match self.state {
            State::OppositeTurn => self.state = State::Flow,
            _ => self.state = State::OppositeTurn,
        }
    }

    // fn find_opposite_coords(opp:&Vec<u32>) -> Vec<(usize,usize)> {
    //     // opp_chars
    // }
}
#[derive(Default, Debug)]
enum State {
    CollectingPiece,
    CollectingAnfield,
    #[default]
    Flow,
    OppositeTurn,
}

#[derive(Default)]
enum Player {
    #[default]
    Player1,
    Player2,
}
