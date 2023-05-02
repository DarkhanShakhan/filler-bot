use std::vec;

pub fn find_available_options(
    first: bool,
    anfield: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    //TODO: choose the player
    let mut out = vec![];
    for i in 0..=anfield.len() - piece.len() {
        for j in 0..=anfield[0].len() - piece[0].len() {
            match is_available(first, anfield, piece, i, j) {
                true => out.push((j,i)),
                false => continue,
            }
        }
    }
    out
}

fn is_available(
    first: bool,
    anfield: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> bool {
    let mut anf_char: char;
    let mut piece_char: char;
    let mut is_overlap = false;
    for ix in 0..piece.len() {
        for jx in 0..piece[0].len() {
            anf_char = anfield[i + ix][j + jx];
            piece_char = piece[ix][jx];
            if anf_char == '.' || piece_char == '.' {
                continue;
            }
            if first && anf_char == '@' {
                return piece_char == 'O'
            }
            if !is_overlap && anf_char == 'a' && piece_char == 'O' {
                is_overlap = true;
                continue;
            }
            return false
        }
    }
   return is_overlap
}
