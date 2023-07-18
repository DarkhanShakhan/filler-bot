use crate::util::Validate;

#[derive(Default)]
pub struct Anfield {
    board: Vec<u128>,
    opposite: Vec<u128>,
    size: (i32, i32),
    counter: i32,
}

//TODO: make board stable; without collecting it all the time; update it instead
impl Anfield {
    pub fn is_full(&self) -> bool {
        self.counter == self.size.1 + 1
    }

    pub fn set_size(&mut self, size: (i32, i32)) {
        self.size = size
    }

    pub fn get_line_length(&self) -> i32 {
        self.size.0
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_board_size() {
        let mut anfield = Anfield::default();
        anfield.set_size((45, 15));
        assert!(anfield.size == (45, 15));
    }
    #[test]
    fn collect_lines() {
        let mut anfield = Anfield::default();
        anfield.collect(8);
        assert!(anfield.counter == 1);
        assert!(anfield.board[0] == 8);
    }
    #[test]
    fn full_collect() {
        let mut anfield = Anfield::default();
        anfield.set_size((6, 3));
        anfield.collect(3);
        anfield.collect(6);
        anfield.collect(7);
        assert!(anfield.is_full());
        assert!(anfield.get_board() == &[3, 6, 7]);
    }
    #[test]
    fn clear_piece() {
        let mut anfield = Anfield::default();
        anfield.set_size((6, 3));
        anfield.collect(3);
        anfield.collect(6);
        anfield.collect(4);
        anfield.clear();
        assert!(anfield.get_board().is_empty());
        assert!(anfield.counter == 0);
    }
    #[test]
    fn collect_opposite_board() {
        let mut anfield = Anfield::default();
        anfield.set_size((4, 2));
        anfield.collect_opposite(3);
        anfield.collect_opposite(2);
        assert!(anfield.get_opposite_board() == &[3, 2])
    }
}
