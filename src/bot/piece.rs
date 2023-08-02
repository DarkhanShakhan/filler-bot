use crate::util::{Size, Validate};

#[derive(Default)]
pub struct Piece {
    piece: Vec<u128>,
    size: Size,
    counter: usize,
}

impl Piece {
    pub fn set_size(&mut self, size: Size) {
        self.size = size
    }
    pub fn get_line_length(&self) -> usize {
        self.size.width
    }
    pub fn collect(&mut self, input: u128) {
        self.piece.push(input);
        self.counter += 1;
    }
    pub fn is_full(&self) -> bool {
        self.counter == self.size.height
    }
    pub fn clear(&mut self) {
        self.piece.clear();
        self.counter = 0;
    }
    pub fn get_piece(&self) -> &Vec<u128> {
        self.piece.as_ref()
    }
}

impl Validate for Piece {
    fn len(&self) -> usize {
        self.piece.len()
    }
}
