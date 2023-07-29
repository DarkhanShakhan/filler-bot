use crate::bot::algorithm::get_best_option;

use super::bot::{algorithm, anfield, piece};
use super::util;
use std::io::{self, BufRead, Write};

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
        let lines = stdin.lock().lines();
        for line in lines {
            match line {
                Ok(inp) => input = inp,
                Err(err) => {
                    return;
                }
            }
            if input.is_empty() {
                continue;
            }
            match self.state {
                State::Flow => self.flow(input.as_str()),
                State::CollectingPiece => self.collect_piece(input.as_str()),
                State::CollectingAnfield => self.collect_anfield(input.as_str()),
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
        let options = algorithm::find_available_options(board, opp, piece, max as usize);
        let opt = get_best_option(board, opp, options.as_ref(), self.anfield.get_size());

        println!("{} {}", opt.1, opt.0);
        // self.state = State::OppositeTurn;
        self.state = State::Flow;
        self.anfield.clear();
        self.piece.clear();
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
    // OppositeTurn,
}

#[derive(Default)]
enum Player {
    #[default]
    Player1,
    Player2,
}
