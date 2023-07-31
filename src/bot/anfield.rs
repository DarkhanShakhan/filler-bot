use crate::util::{Size, Validate};

#[derive(Default)]
pub struct Anfield {
    board: Vec<u128>,
    opposite: Vec<u128>,
    size: Size,
    counter: i32,
}

impl Anfield {
    pub fn is_full(&self) -> bool {
        self.counter == self.size.height + 1
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size
    }

    pub fn get_line_length(&self) -> i32 {
        self.size.width
    }

    pub fn get_size(&self) -> Size {
        self.size
    }
    pub fn collect(&mut self, input: u128) {
        if self.counter > 0 {
            self.board.push(input);
        }
    }

    pub fn collect_opposite(&mut self, input: u128) {
        if self.counter > 0 {
            self.opposite.push(input);
        }
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn clear(&mut self) {
        self.board.clear();
        self.opposite.clear();
        self.counter = 0;
    }

    pub fn get_board(&self) -> &Vec<u128> {
        self.board.as_ref()
    }

    pub fn get_opposite_board(&self) -> &Vec<u128> {
        self.opposite.as_ref()
    }
}

impl Validate for Anfield {
    fn len(&self) -> usize {
        self.board.len()
    }
}
