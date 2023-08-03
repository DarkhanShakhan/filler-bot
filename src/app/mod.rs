use crate::bot::algorithm::best_option;
use crate::util::info_type;

use super::bot::{algorithm, anfield, piece};
use super::util;
use std::io::{self, BufRead};

#[derive(Default)]
pub struct App {
    state: State,
    anfield: anfield::Anfield,
    piece: piece::Piece,
    player: util::PlayerType,
    chars: (char, char),
    opp_chars: (char, char),
}

const PLAYER1_CHARS: (char, char) = ('a', '@');
const PLAYER2_CHARS: (char, char) = ('s', '$');
impl App {
    pub fn start(&mut self) {
        let mut input: String;
        for line in io::stdin().lock().lines() {
            input = line.unwrap_or_default();
            if input.is_empty() {
                continue;
            }
            match self.state {
                State::Flow => self.flow(&input),
                State::CollectingPiece => self.collect_piece(&input),
                State::CollectingAnfield => self.collect_anfield(&input),
            }
        }
    }
    fn flow(&mut self, input: &str) {
        if let Some(t) = info_type(input) {
            match t {
                util::InfoType::Anfield(size) => {
                    self.anfield.set_size(size);
                    self.state = State::CollectingAnfield;
                }
                util::InfoType::Piece(size) => {
                    self.piece.set_size(size);
                    self.state = State::CollectingPiece;
                }
                util::InfoType::Player(player_type) => self.set_player(player_type),
            }
        }
    }
    fn set_player(&mut self, player: util::PlayerType) {
        match player {
            util::PlayerType::PlayerOne => {
                self.chars = PLAYER1_CHARS;
                self.opp_chars = PLAYER2_CHARS;
            }
            util::PlayerType::PlayerTwo => {
                self.chars = PLAYER2_CHARS;
                self.opp_chars = PLAYER1_CHARS;
            }
        }
        self.player = player;
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
        let options = algorithm::available_options(board, opp, piece, max);
        let size = self.anfield.get_size();
        let opt = best_option(board, opp, &options, size);
        print!("{}", opt);
        self.reset();
    }
    fn reset(&mut self) {
        self.state = State::Flow;
        self.anfield.clear();
        self.piece.clear();
    }
}
#[derive(Default, Debug)]
enum State {
    CollectingPiece,
    CollectingAnfield,
    #[default]
    Flow,
}
