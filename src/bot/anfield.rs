use crate::util::Validate;

#[derive(Default)]
pub struct Anfield {
    board: Vec<u32>,
    size: (i32, i32),
    counter: i32,
}

//TODO: make board stable; without collecting it all the time; update it instead
impl Anfield {
    pub fn is_full(&self) -> bool {
        self.counter == self.size.1
    }

    pub fn set_size(&mut self, size: (i32, i32)) {
        self.size = size
    }

    pub fn collect(&mut self, input: u32) {
        self.board.push(input);
        self.counter += 1;
    }

    pub fn clear(&mut self) {
        self.board.clear();
        self.counter = 0;
    }

    pub fn get_board(&mut self) -> &Vec<u32> {
        self.board.as_mut()
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
}
