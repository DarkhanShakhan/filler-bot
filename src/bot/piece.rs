use crate::util::Validate;

pub struct Piece {
    piece: Vec<Vec<char>>,
    size: (i32, i32),
    is_collect: bool,
    counter: i32,
}

impl Piece {
    pub fn new() -> Self {
        Piece {
            piece: vec![],
            size: (0, 0),
            is_collect: false,
            counter: 0,
        }
    }

    pub fn is_collect(&self) -> bool {
        self.is_collect
    }

    pub fn set_is_collect(&mut self, is_collect: bool) {
        self.is_collect = is_collect;
        self.counter = 0;
    }

    // pub fn is_empty(&self) -> bool {
    //     self.piece.len() == 0
    // }

    pub fn set_size(&mut self, size: (i32, i32)) {
        self.size = size
    }

    pub fn collect(&mut self, input: String) {
        self.piece.push(input.chars().collect());
        self.counter += 1;
        if self.counter >= self.size.1 {
            self.is_collect = false;
        }
    }

    pub fn clear(&mut self) {
        self.piece.clear();
    }

    pub fn get_piece(&mut self) -> &Vec<Vec<char>> {
        self.piece.as_mut()
    }
}

impl Validate for Piece {
    fn len(&self) -> usize {
        self.piece.len()
    }
}
