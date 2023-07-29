//TODO: consts
pub fn is_piece_info(input: &str) -> bool {
    input.starts_with("Piece")
}

pub fn is_anfield_info(input: &str) -> bool {
    input.starts_with("Anfield")
}

pub fn is_player_info(input: &str) -> bool {
    input.starts_with("$$$ exec ")
}

pub fn is_player_1(input: &str) -> bool {
    input == "$$$ exec p1 : [solution/filler_bot]"
}

pub fn parse_size(input: &str) -> Option<(i32, i32)> {
    let size: Vec<i32> = input
        .strip_suffix(':')?
        .split_whitespace()
        .rev()
        .take(2)
        .map(|item| item.parse::<i32>().unwrap_or_default())
        .collect();
    match size.len() {
        2 => Some((size[1], size[0])),
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

pub fn merge_bits(input: &[u32], diff: usize, length: usize) -> u32 {
    let mut out = 0;
    for ix in 0..input.len() {
        out |= input[ix];
        out <<= diff;
        if ix != input.len() - 1 {
            out <<= length;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_from_line() {
        assert!(parse_bits("OOOO$", ('$', 's')) == 1);
        assert!(parse_bits("O$OO", ('$', 's')) == 4);
        assert!(parse_bits("OO$O$O$O$O", ('$', 's')) == 170);
    }

    #[test]
    fn count_nbrs_overlaps() {
        assert!(count_overlaps(1, 4) == 0);
        assert!(count_overlaps(1, 3) == 1);
    }

    #[test]
    fn line_from_string() {
        assert!(parse_line("4 OOOOO$") == Some("OOOOO$"));
        assert!(parse_line("7 OOO$OO") == Some("OOO$OO"));
    }

    #[test]
    fn size_from_string() {
        assert!(parse_size("Anfield 45 50:").unwrap() == (45, 50));
        assert!(parse_size("Piece 5 7:").unwrap() == (5, 7));
    }

    #[test]
    fn is_info() {
        assert!(is_anfield_info("Anfield 45 45:"));
        assert!(!is_anfield_info("Any line"));
        assert!(is_piece_info("Piece 5 7:"));
        assert!(!is_piece_info("Any line"));
    }
    #[test]
    fn merge_nbr_bits() {
        assert!(merge_bits(&[1, 2, 3], 2, 2) == 1164);
        assert!(merge_bits(&[1, 2, 3], 0, 2) == 27);
    }
}

pub trait Validate {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
