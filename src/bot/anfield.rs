use crate::util::Validate;

pub struct Anfield {
    board: Vec<Vec<char>>,
    size: (i32, i32),
    is_collect: bool,
    counter: i32,
}

//TODO: make board stable; without collecting it all the time; update it instead
impl Anfield {
    pub fn new() -> Self {
        Anfield {
            board: vec![],
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

    pub fn set_size(&mut self, size: (i32, i32)) {
        self.size = size
    }

    pub fn collect(&mut self, input: &str) {
        self.counter += 1;
        if self.counter > 1 {
            self.board.push(input.chars().collect());
        }
        if self.counter > self.size.1 {
            self.is_collect = false;
        }
    }

    pub fn clear(&mut self) {
        self.board.clear();
    }

    pub fn get_board(&mut self) -> &Vec<Vec<char>> {
        self.board.as_mut()
    }
}

impl Validate for Anfield {
    fn len(&self) -> usize {
        self.board.len()
    }
}