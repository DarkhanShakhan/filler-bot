pub fn is_piece_info(input: &str) -> bool {
    input.starts_with("Piece")
}

pub fn is_anfield_info(input: &str) -> bool {
    input.starts_with("Anfield")
}

//FIXME: functional programming
pub fn get_size(input: &str) -> Option<(i32, i32)> {
    let mut iter = input.strip_suffix("char")?.split_whitespace();
    let first_num = iter.nth(1)?;
    let second_num = iter.next()?;
    let w: i32 = first_num.parse().ok()?;
    let h: i32 = second_num.parse().ok()?;
    Some((w, h))
}

pub fn get_line(input: &str) -> Option<&str> {
    input.split_whitespace().last()
}

pub fn get_bits_from_line(input: &str, compare_to: (char, char)) -> u32 {
    let mut out = 0;
    for ch in input.chars() {
        out <<= 1;
        if ch == compare_to.0 || ch == compare_to.1 {
            out += 1;
        }
    }
    out
}

pub fn count_overlaps(nbr1: u32, nbr2: u32) -> u32 {
    (nbr1 & nbr2).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bits_from_line() {
        assert!(get_bits_from_line("OOOO$", ('$', 's')) == 1);
        assert!(get_bits_from_line("O$OO", ('$', 's')) == 4);
        assert!(get_bits_from_line("OO$O$O$O$O", ('$', 's')) == 170);
    }
    #[test]
    fn count_nbrs_overlaps() {
        assert!(count_overlaps(1, 4) == 0);
        assert!(count_overlaps(1, 3) == 1);
    }
}

pub trait Validate {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
