use std::fmt::Display;
use std::ops::Add;

pub enum InfoType {
    Anfield(Size),
    Piece(Size),
    Player(PlayerType),
}

#[derive(Default)]
pub enum PlayerType {
    #[default]
    PlayerOne,
    PlayerTwo,
}

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

pub fn info_type(input: &str) -> Option<InfoType> {
    if input.starts_with("Piece") {
        if let Some(size) = parse_size(input) {
            return Some(InfoType::Piece(size));
        }
    }
    if input.starts_with("Anfield") {
        if let Some(size) = parse_size(input) {
            return Some(InfoType::Anfield(size));
        }
    }
    if input.starts_with("$$$ exec") {
        if let Some(player_type) = parse_player_type(input) {
            return Some(InfoType::Player(player_type));
        }
    }
    None
}

pub fn parse_player_type(input: &str) -> Option<PlayerType> {
    if let Some(player) = input.split_whitespace().nth(2) {
        match player {
            "p1" => return Some(PlayerType::PlayerOne),
            "p2" => return Some(PlayerType::PlayerTwo),
            _ => return None,
        }
    }
    None
}

pub fn parse_size(input: &str) -> Option<Size> {
    let size: Vec<usize> = input
        .strip_suffix(':')?
        .split_whitespace()
        .rev()
        .take(2)
        .map(|item| item.parse::<usize>().unwrap_or_default())
        .collect();
    match size.len() {
        2 => Some(Size {
            height: size[0],
            width: size[1],
        }),
        _ => None,
    }
}

pub fn parse_line(input: &str) -> Option<&str> {
    input.split_whitespace().last()
}

pub fn parse_bits(input: &str, compare_to: (char, char)) -> u128 {
    let mut out = 0;
    for ch in input.chars() {
        out <<= 1;
        if ch == compare_to.0 || ch == compare_to.1 {
            out += 1;
        }
    }
    out
}

pub fn count_overlaps(nbr1: u128, nbr2: u128) -> u32 {
    (nbr1 & nbr2).count_ones()
}

pub trait Validate {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Board {
    board: Vec<Vec<i32>>,
    board_size: Size,
    mine_points: Vec<Point>,
    opposite_points: Vec<Point>,
}

impl Board {
    pub fn new(size: Size) -> Self {
        Board {
            board: vec![vec![0; size.width]; size.height],
            board_size: size,
            mine_points: vec![],
            opposite_points: vec![],
        }
    }
    pub fn set_point(&mut self, point: Point) {
        self.board[point.x][point.y] = i32::from(point.cell);
        match point.cell {
            CellOwnership::Mine => self.mine_points.push(point),
            CellOwnership::Opponent => self.opposite_points.push(point),
            CellOwnership::Empty => {}
        }
    }
    // pub fn get_point(&mut self, point: (usize, usize)) -> Point {
    //     Point {
    //         x: point.0,
    //         y: point.1,
    //         cell: CellOwnership::from(self.board[point.0][point.1]),
    //     }
    // }
    fn check_point(&self, point: &Point, x_coef: i32, y_coef: i32, k: usize) -> (bool, u32) {
        let mut free = true;
        let mut blocked = false;
        for ix in 1..k {
            match self.board[(point.x as i32 + (ix as i32 * x_coef)) as usize]
                [(point.y as i32 + (ix as i32 * y_coef)) as usize]
            {
                0 => {}
                1 => {
                    blocked = true;
                    free = false;
                }
                _ => free = false,
            }
        }
        match free {
            true => (blocked, 1),
            false => (blocked, 0),
        }
    }
    pub fn borders(&self, points: &[Point], k: usize) -> Vec<Point> {
        let mut res = vec![];
        let mut blocked = false;
        let mut count_free = 0;
        let mut check_res: (bool, u32);
        for p in points {
            if p.x >= k {
                check_res = self.check_point(p, -1, 0, k);
                blocked = check_res.0;
                count_free += check_res.1;
            }
            if p.x + k < self.board_size.height {
                check_res = self.check_point(p, 1, 0, k);
                blocked = check_res.0;
                count_free += check_res.1;
            }
            if p.y >= k {
                check_res = self.check_point(p, 0, -1, k);
                blocked = check_res.0;
                count_free += check_res.1;
            }
            if p.y + k < self.board_size.width {
                check_res = self.check_point(p, 0, 1, k);
                blocked = check_res.0;
                count_free += check_res.1;
            }
            match count_free {
                0 => {}
                1 => {
                    if !blocked {
                        res.push(*p);
                    }
                }
                _ => res.push(*p),
            }
            blocked = false;
            count_free = 0;
        }
        res
    }
}
#[derive(Default, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub cell: CellOwnership,
}

impl Point {
    pub fn new(x: usize, y: usize, cell: CellOwnership) -> Self {
        Point { x, y, cell }
    }
}

impl Add for Point {
    type Output = usize;
    fn add(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.y, self.x)
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0,
            y: value.1,
            cell: CellOwnership::Empty,
        }
    }
}
#[derive(Default, Copy, Clone)]
pub enum CellOwnership {
    Mine,
    Opponent,
    #[default]
    Empty,
}

impl From<i32> for CellOwnership {
    fn from(nbr: i32) -> Self {
        match nbr {
            1 => CellOwnership::Mine,
            -1 => CellOwnership::Opponent,
            _ => CellOwnership::Empty,
        }
    }
}

impl From<CellOwnership> for i32 {
    fn from(cell_status: CellOwnership) -> Self {
        match cell_status {
            CellOwnership::Mine => 1,
            CellOwnership::Opponent => -1,
            CellOwnership::Empty => 0,
        }
    }
}
