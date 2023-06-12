use crate::util::{self, Validate};

#[derive(Default)]
pub struct Piece {
    piece: Vec<u32>,
    size: (i32, i32),
    counter: i32,
}

impl Piece {
    pub fn set_size(&mut self, size: (i32, i32)) {
        self.size = size
    }

    pub fn collect(&mut self, input: u32) {
        self.piece.push(input);
        self.counter += 1;
    }
    pub fn is_full(&self) -> bool {
        self.counter == self.size.1
    }
    pub fn clear(&mut self) {
        self.piece.clear();
        self.counter = 0;
    }

    pub fn get_piece(&mut self) -> &Vec<u32> {
        self.piece.as_mut()
    }
}

impl Validate for Piece {
    fn len(&self) -> usize {
        self.piece.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_piece_size() {
        let mut piece = Piece::default();
        piece.set_size((45, 15));
        assert!(piece.size == (45, 15));
    }
    #[test]
    fn collect_lines() {
        let mut piece = Piece::default();
        piece.collect(8);
        assert!(piece.counter == 1);
        assert!(piece.piece[0] == 8);
    }
    #[test]
    fn full_collect() {
        let mut piece = Piece::default();
        piece.set_size((6, 3));
        piece.collect(3);
        piece.collect(6);
        piece.collect(7);
        assert!(piece.is_full());
        assert!(piece.get_piece() == &[3, 6, 7]);
    }
    #[test]
    fn clear_piece() {
        let mut piece = Piece::default();
        piece.set_size((6, 3));
        piece.collect(3);
        piece.collect(6);
        piece.collect(4);
        piece.clear();
        assert!(piece.get_piece().is_empty());
        assert!(piece.counter == 0);
    }
}
