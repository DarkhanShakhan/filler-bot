use super::util;
use std::io::{self, BufRead};

use super::bot::{algorithm, anfield, piece};

#[derive(Default)]
pub struct App {
    state: State,
    anfield: anfield::Anfield,
    piece: piece::Piece,
    player: Player,
    chars: (char, char),
}

impl App {
    pub fn start(&mut self) {
        let stdin = io::stdin();
        let mut input: String;
        for line in stdin.lock().lines() {
            match line {
                Ok(inp) => input = inp,
                Err(_) => return,
            }
            match self.state {
                State::Flow => self.flow(input.as_str()),
                State::CollectingPiece => self.collect_piece(input.as_str()),
                State::Algorithm => self.algorithm(input.as_str()),
                State::CollectingAnfield => self.collect_anfield(input.as_str()),
                State::OppositeTurn => self.opposite_turn(input.as_str()),
            }
        }
    }
    fn flow(&mut self, input: &str) {
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
                    self.chars = ('a', '@')
                }
                false => {
                    self.player = Player::Player2;
                    self.chars = ('s', '$');
                }
            }
        }
    }
    fn collect_piece(&mut self, input: &str) {
        self.piece.collect(util::parse_bits(
            util::parse_line(input).unwrap(),
            self.chars,
        ));
        if self.piece.is_full() {
            self.state = State::Algorithm;
        }
    }
    fn collect_anfield(&mut self, input: &str) {
        self.anfield.collect(util::parse_bits(
            util::parse_line(input).unwrap(),
            self.chars,
        ));
        if self.anfield.is_full() {
            self.state = State::Flow;
        }
    }
    fn algorithm(&mut self, input: &str) {}
    fn opposite_turn(&mut self, input: &str) {}
}

#[derive(Default)]
enum State {
    CollectingPiece,
    CollectingAnfield,
    #[default]
    Flow,
    Algorithm,
    OppositeTurn,
}

#[derive(Default)]
enum Player {
    #[default]
    Player1,
    Player2,
}
