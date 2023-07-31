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
    pub height: i32,
    pub width: i32,
}

impl Size {
    pub fn as_usize(&self) -> (usize, usize) {
        (self.height as usize, self.width as usize)
    }
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
    let size: Vec<i32> = input
        .strip_suffix(':')?
        .split_whitespace()
        .rev()
        .take(2)
        .map(|item| item.parse::<i32>().unwrap_or_default())
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
